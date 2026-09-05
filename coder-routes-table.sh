#!/bin/bash
# Regenerate the "Ans against Range" tables in OPTIMIZING.md.
#
#   ./coder-routes-table.sh                    # every workload
#   ./coder-routes-table.sh -q                 # 1% precision instead of 0.1%
#   ./coder-routes-table.sh strings records    # just these workloads
#   ./coder-routes-table.sh -e                 # encode table only
#   ./coder-routes-table.sh -d                 # decode table only
#
# Prints two markdown tables on stdout — decode then encode — ready to paste
# under the section that names them.
#
# There is no async encode for either coder, so the encode table has two routes
# where the decode table has three. That gap is the point: "async" currently
# means decode only.
#
# Requires the machine to be quiesced (`sudo `which quiet-bench` reserve 2`);
# every cell runs through `quiet-bench run`, which pins it to the reserved CPU.
# See "How to benchmark on this machine" in OPTIMIZING.md.
#
# Each cell is one `scaling` measurement: the binary samples until the standard
# error of its mean is under 0.1% (or `-q`'s 1%) and prints the figure it
# reached, so there is no iteration count to tune here and no min-of-N to take.
# The Δ column carries the error the two cells imply, and marks with `?` any
# difference those error bars cannot support — both arms are the *same binary*
# with different arguments, so binary layout cancels and that error bar really
# is the whole uncertainty.
set -euo pipefail

do_decode=yes
do_encode=yes
declare -a want=()
while [ $# -gt 0 ]; do
    case "$1" in
        -q) export BENCH_REL_ERROR=0.01; shift ;;
        -e) do_decode=no; shift ;;
        -d) do_encode=no; shift ;;
        -h|--help) sed -n '2,27p' "$0" | sed 's/^# \?//'; exit 0 ;;
        -*) echo "unknown flag $1" >&2; exit 2 ;;
        *) want+=("$1"); shift ;;
    esac
done

workloads=(
    strings enums enums17 floats compressible records records-wide
    atmost3 atmost8 atmost16 atmost32 atmost128
)

# `quiet-bench run true` succeeds iff a reservation exists — `quiet-bench
# status` answers for *this* process, which is not itself pinned.
quiet-bench run true 2>/dev/null || {
    echo "machine is not quiesced; see OPTIMIZING.md" >&2
    exit 1
}

# Build outside the reservation (so compilation is not squeezed onto one core)
# and take the path cargo reports: a bench executable's name carries a hash, so
# it cannot be hardcoded. Running it directly rather than through `cargo bench`
# keeps cargo itself out of the pinned CPU for every one of the ~120 cells.
bin=$(cargo bench --no-run --features stream,benchmarking --bench coder-routes 2>&1 |
      sed -n 's|.*Executable benches/coder-routes\.rs (\(.*\))$|\1|p' | tail -1)
[ -n "$bin" ] && [ -x "$bin" ] || {
    echo "could not find the coder-routes bench executable" >&2
    exit 1
}

# One cell: `ns err size flags`, straight off the binary's `result` line.
cell() {
    quiet-bench run "$bin" "$1" "$2" "$3" 2>/dev/null | awk '
        /^result / {
            for (i = 2; i <= NF; i++) { split($i, kv, "="); v[kv[1]] = kv[2] }
            printf "%s %s %s %s\n", v["ns"], v["err"], v["size"], v["flags"]
            found = 1
        }
        END { if (!found) { print "coder-routes printed no result line" > "/dev/stderr"; exit 1 } }'
}

# One table over the given routes.
table() {
    printf '| workload | route | Range | Ans | Δ | size Δ |\n'
    printf '|---|---|---|---|---|---|\n'
    for workload in "${workloads[@]}"; do
        if [ ${#want[@]} -gt 0 ]; then
            printf '%s\n' "${want[@]}" | grep -qx "$workload" || continue
        fi
        for route in "$@"; do
            read -r rns rerr rsize rflags < <(cell "$workload" range "$route")
            read -r ans aerr asize aflags < <(cell "$workload" ans "$route")
            awk -v w="$workload" -v r="$route" \
                -v rns="$rns" -v rerr="$rerr" -v ans="$ans" -v aerr="$aerr" \
                -v rsize="$rsize" -v asize="$asize" \
                -v rflags="$rflags" -v aflags="$aflags" 'BEGIN {
                    # A time and its error in one unit, as `scaling` prints them.
                    ratio = ans / rns
                    delta = (ratio - 1) * 100
                    # Relative errors in quadrature, carried through the ratio.
                    derr = ratio * sqrt((aerr/ans)^2 + (rerr/rns)^2) * 100
                    # `?` when the gap is inside three of those error bars, or
                    # when scaling flagged either measurement.
                    mark = ""
                    size = delta < 0 ? -delta : delta
                    if (rflags != "ok" || aflags != "ok") mark = "?"
                    else if (size < 3 * derr) mark = "?"
                    printf "| `%s` | `%s` | %s | %s | **%+.1f±%.1f%%%s** | %+.2f%% |\n",
                        w, r, t(rns, rerr), t(ans, aerr), delta, derr, mark,
                        (asize/rsize - 1) * 100
                }
                function t(ns, err,   div, unit) {
                    div = 1; unit = "ns"
                    if (ns >= 1e6) { div = 1e6; unit = "ms" }
                    else if (ns >= 1e3) { div = 1e3; unit = "µs" }
                    return sprintf("%.3f±%.3f%s", ns/div, err/div, unit)
                }'
        done
    done
}

if [ "$do_decode" = yes ]; then
    echo "#### Decode"
    echo
    table slice from stream
    echo
fi
if [ "$do_encode" = yes ]; then
    echo "#### Encode"
    echo
    table encode encode-to
fi
