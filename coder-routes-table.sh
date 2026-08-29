#!/bin/bash
# Regenerate the "Ans against Range" tables in OPTIMIZING.md.
#
#   ./coder-routes-table.sh                    # every workload, min of 5
#   ./coder-routes-table.sh -n 3               # fewer repetitions, quicker
#   ./coder-routes-table.sh strings records    # just these workloads
#   ./coder-routes-table.sh -e                 # encode table only
#   ./coder-routes-table.sh -d                 # decode table only
#
# Prints two markdown tables on stdout — decode then encode — ready to paste
# under the section that names them. Ten cells per workload at a few seconds
# each, so the full run is around half an hour.
#
# There is no async encode for either coder, so the encode table has two routes
# where the decode table has three. That gap is the point: "async" currently
# means decode only.
#
# Requires the machine to be quiesced (`sudo ./bench-quiet.sh 2`) — every cell
# runs through the `bench` wrapper that installs, because measurements taken off
# the reserved CPU are not worth the time they cost. See "How to benchmark on
# this machine" in OPTIMIZING.md.
#
# Both arms of every comparison are the *same binary* with different arguments,
# so binary-layout noise cancels and the only difference is the coder and the
# route. That is why this can be a simple min-of-N rather than the alternated
# A/B an across-commits comparison needs.
set -euo pipefail

reps=5
do_decode=yes
do_encode=yes
declare -a want=()
while [ $# -gt 0 ]; do
    case "$1" in
        -n) reps=$2; shift 2 ;;
        -e) do_decode=no; shift ;;
        -d) do_encode=no; shift ;;
        -h|--help) sed -n '2,26p' "$0" | sed 's/^# \?//'; exit 0 ;;
        -*) echo "unknown flag $1" >&2; exit 2 ;;
        *) want+=("$1"); shift ;;
    esac
done

# workload:decode-iterations:encode-iterations. Chosen so each cell runs a few
# seconds — long enough that startup and the one-off encode do not show, short
# enough to sit through. Encode gets its own count because it is not uniformly
# the same cost as decode: Lz77 `compressible` encodes far slower than it
# decodes, while the rest are close enough to share.
workloads=(
    strings:300:300
    enums:1200:1200
    enums17:400:400
    floats:1500:1500
    compressible:30:8
    records:60:60
    records-wide:40:40
    atmost3:1200:1200
    atmost8:1200:1200
    atmost16:1200:1200
    atmost32:1200:1200
    atmost128:1200:1200
)

bin=target/release/coder-routes
bench true || { echo "machine is not quiesced; see OPTIMIZING.md" >&2; exit 1; }
[ -x "$bin" ] || cargo build --release --features stream --bin coder-routes >&2

# Min-of-$reps cycles, instructions and the encoded size for one cell.
#
# The **min**, not `perf --repeat`'s mean: what is being estimated is the cost
# with nothing else interfering, and interference only ever adds. One stray
# scheduling event moves a mean and cannot move a min.
#
# `cpu_core/` and the pinning are both load-bearing on this hybrid CPU; a bare
# `-e cycles` counts on one core type while the process migrates between them,
# and silently reports a fraction of the work.
cell() {
    local workload=$1 coder=$2 route=$3 iters=$4 i
    for ((i = 0; i < reps; i++)); do
        bench perf stat -x, -e cpu_core/cycles/,cpu_core/instructions/ \
            "$bin" "$workload" "$coder" "$route" "$iters" 2>&1
    done | awk -F, '
        # perf -x, gives value,unit,event,...; a multiplexed counter can come
        # back "<not counted>", which must not be mistaken for a small number.
        /encoded size/ { n = split($0, a, " "); size = a[n] }
        $1 + 0 != $1 { next }
        $3 ~ /cpu_core\/cycles\// { if (!cyc || $1 < cyc) cyc = $1 }
        $3 ~ /cpu_core\/instructions\// { if (!ins || $1 < ins) ins = $1 }
        END {
            if (!cyc || !ins) { print "perf produced no counts" > "/dev/stderr"; exit 1 }
            printf "%d %d %d\n", cyc, ins, size
        }'
}

# One table over the given routes.
table() {
    printf '| workload | route | Range cyc | Ans cyc | Δ | Range ins | Ans ins | Δ | size Δ |\n'
    printf '|---|---|---|---|---|---|---|---|---|\n'
    for entry in "${workloads[@]}"; do
        IFS=: read -r workload diters eiters <<<"$entry"
        case "$1" in encode*) iters=$eiters ;; *) iters=$diters ;; esac
        if [ ${#want[@]} -gt 0 ]; then
            printf '%s\n' "${want[@]}" | grep -qx "$workload" || continue
        fi
        for route in "$@"; do
            read -r rcyc rins rsize < <(cell "$workload" range "$route" "$iters")
            read -r acyc ains asize < <(cell "$workload" ans "$route" "$iters")
            awk -v w="$workload" -v r="$route" \
                -v rc="$rcyc" -v ac="$acyc" -v ri="$rins" -v ai="$ains" \
                -v rs="$rsize" -v as="$asize" 'BEGIN {
                    printf "| `%s` | `%s` | %.3fG | %.3fG | **%+.1f%%** | %.3fG | %.3fG | %+.1f%% | %+.2f%% |\n",
                        w, r, rc/1e9, ac/1e9, (ac/rc - 1) * 100,
                        ri/1e9, ai/1e9, (ai/ri - 1) * 100, (as/rs - 1) * 100
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
