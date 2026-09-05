#!/bin/bash
# Side-by-side `cargo bench -p comparison` before and after the working-tree
# changes — a quick survey of whether anything moved, not an A/B you should
# act on.
#
#   ./diff-bench.sh
#
# Two caveats, both from "How to benchmark on this machine" in OPTIMIZING.md:
#
#  * This is a stash-and-compare, so the two halves are separate builds run
#    one after the other. That is exactly the comparison whose floor is ~1%
#    of binary-layout noise, and the survey tables run at `scaling`'s default
#    1% precision on top of that. Treat anything under a few percent as
#    nothing, and confirm a real finding with the focused `src/bin/` workloads
#    alternated between builds.
#  * It needs a reserved CPU like everything else. Both halves run under
#    `quiet-bench run`; the build steps deliberately do not, so compilation is
#    not squeezed onto one core.
set -euo pipefail

quiet-bench run true 2>/dev/null || {
    echo "machine is not quiesced; see OPTIMIZING.md" >&2
    exit 1
}

run() {
    cargo bench --no-run --package comparison >&2
    quiet-bench run cargo bench --package comparison
}

git stash
run > old.txt
git stash pop
run > new.txt

diff --side-by-side old.txt new.txt
