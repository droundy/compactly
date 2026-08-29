#!/bin/bash
# Regenerate the "Ans against Range" table in OPTIMIZING.md.
#
#   ./decode-routes-table.sh                    # every workload, min of 5
#   ./decode-routes-table.sh -n 3               # fewer repetitions, quicker
#   ./decode-routes-table.sh strings records    # just these workloads
#
# Prints a markdown table on stdout, ready to paste under the section it names.
# Takes a few seconds per cell and there are six cells per workload, so the full
# run is around twenty minutes.
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
declare -a want=()
while [ $# -gt 0 ]; do
    case "$1" in
        -n) reps=$2; shift 2 ;;
        -h|--help) sed -n '2,20p' "$0" | sed 's/^# \?//'; exit 0 ;;
        -*) echo "unknown flag $1" >&2; exit 2 ;;
        *) want+=("$1"); shift ;;
    esac
done

# workload:iterations. Iterations are chosen so each cell runs a few seconds —
# long enough that startup and encoding do not show, short enough to sit through.
workloads=(
    strings:300
    enums:1200
    enums17:400
    floats:1500
    compressible:30
    records:60
    records-wide:40
    atmost3:1200
    atmost8:1200
    atmost16:1200
    atmost32:1200
    atmost128:1200
)

bin=target/release/decode-routes
bench true || { echo "machine is not quiesced; see OPTIMIZING.md" >&2; exit 1; }
[ -x "$bin" ] || cargo build --release --features stream --bin decode-routes >&2

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

printf '| workload | route | Range cyc | Ans cyc | Δ | Range ins | Ans ins | Δ | size Δ |\n'
printf '|---|---|---|---|---|---|---|---|---|\n'
for entry in "${workloads[@]}"; do
    workload=${entry%%:*}
    iters=${entry##*:}
    if [ ${#want[@]} -gt 0 ]; then
        printf '%s\n' "${want[@]}" | grep -qx "$workload" || continue
    fi
    for route in slice from stream; do
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
