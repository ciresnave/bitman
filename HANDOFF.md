# bitman HANDOFF (current state only)

Task from PM 2026-10-25: (1) setup, (2) current deps/CI, (3) triage issues #1 #2, (4) patent sweep + provenance, (5) tags.
Done: v2.0.1 tag pushed at 47c6565 (= crates.io .cargo_vcs_info sha). Branch ci-and-deps in worktree bitman-wt-ci-deps: CI, pinned nightly, deps, clippy fixes.
Findings: stable build fails (specialization); README documents an API that does not exist (to_bits/AnyIntegerType); BitMan trait is private; BitMan::set_bits is buggy+unused.
Next: PR #3 fixes in flight; then patent sweep.
