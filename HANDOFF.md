# bitman HANDOFF (current state only)

SCALED TO ZERO 2026-10-28 (CireSnave ruled; PM task). Wake on: PM mail (merge/gate of #13/#14, retarget), a new issue, or a [TASK].
Published: 3.1.0. Merged since: 3.1.2 docs.
In flight (gate in order): PR #13 (3.1.3 docs, ROADMAP owners), then PR #14 typed-fields (3.2.0, `Field`; stacked on #13). After #13 squash-merges: `rebase --onto origin/main <old #13 tip>`, force-push with lease, retarget #14 to main. Then PM publishes/tags; lane fast-forwards primary and removes worktrees (bitman-wt-rm2, bitman-wt-fields) + local branches.
Pending: none held. ant-fit census sent (all no; ReleaseAnt maybe).
Backlog (ROADMAP): Tarpaulin coverage; Criterion/Flamegraph (BitMan on arrays is O(elements) per bit). Issues #1, #2 closed; sweep issues weekly (decide any open one by 2026-11-08).
Notes: src/bits.rs, src/lib.rs blobs are CRLF, others LF: edit EOL-preserving, `git add` explicit paths, `cargo fmt --check` on a git-archive of the pushed head. CARGO_BUILD_JOBS=2, one build at a time.
Patent sweep docs/PATENT-SWEEP.md names 3.x; add Field (idea 8) at next touch; re-sweep by 2027-10-26.
