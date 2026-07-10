#!/bin/bash
# Set up (or tear down) a low-noise benchmarking environment.
#
#   sudo ./bench-quiet.sh 2         # quiesce the machine, reserving CPU 2
#   sudo ./bench-quiet.sh 2,3       # reserve CPUs 2 and 3
#   sudo ./bench-quiet.sh restore   # undo everything
#
# Then run benchmarks through the `bench` wrapper it installs, e.g.:
#   bench perf stat -e cpu_core/cycles/ <bin>
#
# What it does:
#   - disables turbo and pins min frequency to 100% (thermal repeatability)
#   - performance governor on all CPUs
#   - offlines the SMT siblings of the reserved CPUs
#   - moves all existing/future processes off the reserved CPUs (per-task
#     affinity, so the bench wrapper's taskset can still claim them)
#   - stops irqbalance and steers IRQs off the reserved CPUs
#   - disables ASLR and opens up perf_event_paranoid for unprivileged perf stat

set -euo pipefail

STATE=/run/bench-quiet.state
CPUS_FILE=/run/bench-quiet.cpus
WRAPPER=/usr/local/bin/bench
PSTATE=/sys/devices/system/cpu/intel_pstate

usage() {
    echo "usage: sudo $0 <cpu-list>    e.g. sudo $0 2" >&2
    echo "       sudo $0 restore" >&2
    exit 1
}

[ $# -eq 1 ] || usage
[ "$(id -u)" -eq 0 ] || { echo "error: must run as root (sudo)" >&2; exit 1; }

NCPU=$(getconf _NPROCESSORS_CONF)
[ "$NCPU" -le 64 ] || { echo "error: >64 CPUs; affinity mask math below won't fit" >&2; exit 1; }

# Expand a cpu-list like "1,3-5" to "1 3 4 5".
expand() {
    local part
    for part in ${1//,/ }; do
        if [[ $part == *-* ]]; then
            seq "${part%-*}" "${part#*-}"
        else
            echo "$part"
        fi
    done
}

# Hex affinity mask for a list of CPU numbers.
mask_of() {
    local m=0 c
    for c in $1; do
        m=$((m | (1 << c)))
    done
    printf '%x' "$m"
}

restore() {
    # Saved originals from setup, if present.
    local orig_no_turbo=0 orig_min_perf=
    [ -f "$STATE" ] && . "$STATE"

    echo "$orig_no_turbo" > "$PSTATE/no_turbo"
    [ -n "$orig_min_perf" ] && echo "$orig_min_perf" > "$PSTATE/min_perf_pct"

    local online
    for online in /sys/devices/system/cpu/cpu[0-9]*/online; do
        echo 1 > "$online" 2>/dev/null || true
    done

    local gov
    for gov in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do
        echo powersave > "$gov" 2>/dev/null || true
    done

    # Empty AllowedCPUs resets the slices to "all CPUs" (older versions of
    # this script set a cpuset; current ones only use plain affinity).
    local unit
    for unit in system.slice user.slice init.scope; do
        systemctl set-property --runtime "$unit" AllowedCPUs="" 2>/dev/null || true
    done

    # Let every process roam all (now re-onlined) CPUs again.
    local pid
    for pid in /proc/[0-9]*; do
        taskset -apc "0-$((NCPU - 1))" "${pid#/proc/}" >/dev/null 2>&1 || true
    done

    echo 2 > /proc/sys/kernel/randomize_va_space
    echo 2 > /proc/sys/kernel/perf_event_paranoid

    local allmask irq
    allmask=$(mask_of "$(seq 0 $((NCPU - 1)))")
    echo "$allmask" > /proc/irq/default_smp_affinity
    for irq in /proc/irq/[0-9]*/smp_affinity; do
        echo "$allmask" > "$irq" 2>/dev/null || true
    done
    systemctl start irqbalance 2>/dev/null || true

    rm -f "$STATE" "$CPUS_FILE" "$WRAPPER"
    echo "restored: turbo=$([ "$orig_no_turbo" = 0 ] && echo on || echo off), all CPUs online, powersave governor, ASLR on, irqbalance running"
}

if [ "$1" = restore ]; then
    restore
    exit 0
fi

BENCH_CPUS=$(expand "$1")
for c in $BENCH_CPUS; do
    [ -d /sys/devices/system/cpu/cpu"$c" ] || { echo "error: no such CPU: $c" >&2; exit 1; }
done

# Everything except the bench CPUs, for housekeeping.
OTHER_CPUS=$(seq 0 $((NCPU - 1)) | grep -Fxv -e "$(echo "$BENCH_CPUS" | tr ' ' '\n')" || true)
[ -n "$OTHER_CPUS" ] || { echo "error: must leave at least one CPU for the rest of the system" >&2; exit 1; }

# Save originals so restore can put them back. Write-once: on a re-run
# (including after a partial failure) the values already reflect our own
# changes, and the first run's snapshot is the true original.
if [ ! -f "$STATE" ]; then
    {
        echo "orig_no_turbo=$(cat "$PSTATE/no_turbo")"
        echo "orig_min_perf=$(cat "$PSTATE/min_perf_pct")"
    } > "$STATE"
fi

# 1. Kill turbo and pin frequency: the single biggest source of run-to-run
#    drift on this laptop is thermal throttling out of turbo mid-benchmark.
echo 1 > "$PSTATE/no_turbo"
echo 100 > "$PSTATE/min_perf_pct"

# 2. Performance governor everywhere.
for gov in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do
    echo performance > "$gov" 2>/dev/null || true
done

# 3. Offline the SMT siblings of the bench CPUs (E-cores have none; harmless).
for c in $BENCH_CPUS; do
    for sib in $(expand "$(cat /sys/devices/system/cpu/cpu"$c"/topology/thread_siblings_list)"); do
        if ! echo "$BENCH_CPUS" | grep -qw "$sib"; then
            echo 0 > /sys/devices/system/cpu/cpu"$sib"/online
            echo "offlined SMT sibling cpu$sib of cpu$c"
        fi
    done
done

# 4. Herd every existing process (and, via inheritance, its future children)
#    onto the housekeeping CPUs using plain per-task affinity. Deliberately
#    NOT a cgroup cpuset (systemd AllowedCPUs=): taskset cannot escape a
#    cpuset, so that would lock the benchmark itself out of the reserved
#    CPUs. Plain affinity is overridable by the bench wrapper. Kernel
#    threads that refuse to move are ignored.
OTHER_LIST=$(echo "$OTHER_CPUS" | paste -sd,)
# Clear any cpuset left behind by an older version of this script — it
# would silently veto the wrapper's taskset.
for unit in system.slice user.slice init.scope; do
    systemctl set-property --runtime "$unit" AllowedCPUs="" 2>/dev/null || true
done
moved=0
for pid in /proc/[0-9]*; do
    taskset -apc "$OTHER_LIST" "${pid#/proc/}" >/dev/null 2>&1 && moved=$((moved + 1))
done
echo "moved $moved processes onto CPUs $OTHER_LIST"

# 5. Steer interrupts away from the bench CPUs. Some IRQs (managed, per-CPU)
#    refuse the write; that's fine.
systemctl stop irqbalance 2>/dev/null || true
OTHER_MASK=$(mask_of "$OTHER_CPUS")
echo "$OTHER_MASK" > /proc/irq/default_smp_affinity
moved=0 stuck=0
for irq in /proc/irq/[0-9]*/smp_affinity; do
    if echo "$OTHER_MASK" > "$irq" 2>/dev/null; then
        moved=$((moved + 1))
    else
        stuck=$((stuck + 1))
    fi
done
echo "IRQs steered off bench CPUs: $moved moved, $stuck immovable"

# 6. Deterministic address layout and unprivileged perf counters.
echo 0 > /proc/sys/kernel/randomize_va_space
echo -1 > /proc/sys/kernel/perf_event_paranoid

# 7. `bench` wrapper that pins to the reserved CPUs, so benchmark commands
#    don't need to remember the list. The CPU list lives in a file on /run
#    (tmpfs: cleared by restore and by reboot, so never stale) and the
#    wrapper refuses to run without it — "does `bench` work" doubles as the
#    "is the quiet setup active" check. The wrapper itself can't live on
#    /run because /run is mounted noexec.
echo "$1" > "$CPUS_FILE"
cat > "$WRAPPER" <<EOF
#!/bin/sh
cpus=\$(cat $CPUS_FILE 2>/dev/null) || {
    echo "machine is not quiesced: run  sudo $(readlink -f "$0") <cpu-list>" >&2
    exit 1
}
exec taskset -c "\$cpus" "\$@"
EOF
chmod 755 "$WRAPPER"

cat <<EOF

ready: CPUs $1 reserved (turbo off, perf governor, IRQs and processes moved off)

Run benchmarks through the pinning wrapper (as your normal user, not root):

  # cycle counts, the most stable metric — run several times and take the min:
  bench perf stat -e cpu_core/cycles/ <bin>

  # criterion benchmarks:
  bench cargo bench --bench bench

'bench' ($WRAPPER) expands to 'taskset -c $1'; anything not run through it
lands on the crowded housekeeping CPUs ($OTHER_LIST) and gains nothing.

Undo with:  sudo $0 restore
EOF
