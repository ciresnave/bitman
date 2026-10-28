# bitman HANDOFF (current state only)

FULL SPEED (throttle ended 2026-10-28). Published: 3.1.0. Merged since: 3.1.2 docs.
In flight (gate in order): PR #13 (3.1.3 docs, ROADMAP owners) then PR typed-fields (3.2.0, `Field`; stacked on #13, rebase onto main after #13 squash).
Issues #1, #2 closed. Backlog (ROADMAP): Tarpaulin coverage; Criterion/Flamegraph (BitMan on arrays is O(elements) per bit).
Notes: src/bits.rs, src/lib.rs blobs are CRLF, other files LF: edit with EOL preserved, `git add` explicit paths, `cargo fmt --check` on a git-archive of the pushed head. Box memory limit: CARGO_BUILD_JOBS=2, one build at a time.
Sweep record docs/PATENT-SWEEP.md names 3.x; add Field (idea 8) at next sweep touch; re-sweep by 2027-10-26.
