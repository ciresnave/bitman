# bitman — lane instructions

Adds to `C:\Projects\CLAUDE.md` (never relaxes it). Owner of this repo: the bitman lane (taken over 2026-10-25;
CireSnave: *"BitMan now has a lane.  We maintain that project."*, relayed by the PM).

- Crate `bitman` on crates.io (published 2.0.1 from `47c6565`, tagged `v2.0.1`). MIT, Eric Evans.
- Primary checkout `C:\Projects\bitman` stays on `main`; branches live in `C:\Projects\bitman-wt-<topic>`.
- Lane never merges, publishes, or allocates versions; the PM does.
- Facts about this repo: `src/lib.rs` uses `#![feature(specialization)]`, so it builds on nightly only (stable fails E0554, measured 2026-10-25 at `47c6565`).
