BitMan Roadmap for v2.0
=======================

- Rewrite BitMan to make it's usage more user friendly.
  - Move any trait methods that can't be blanket implemented into a macro (done!)
    - Create another crate for a derive macro to add BitMan to a type (done!)
  - Change from Vec\<bool> to Bits newtype that encloses the Vec\<bool> (done!)
  - Change from bool to Bit newtype that encloses the bool (done!)
  - Make Bits type support indexing (both for single Bit and slices of Bits) (done!)
  - Create bit()/bits() methods that splits any BitMan capable value to Bits (done!)
  - Create set_bit()/set_bits() methods that write to BitMan capable variables (done!)
  - Add ability to convert from Bits to any type that supports BitMan (done!)
  - Add ability to assign to a range of bits within any Bits. (done!)
  - Change to range based parameters to BitMan methods (done!)
  - Add Field to get read()/write() methods for any BitMan type within a Bits (done!)
  - Add functions to output Bits variables as [u8] in any endianness:
    - Add to_be_bytes() to output big endian bytes of big endian bits (done!)
    - Add to_le_bytes() to output little endian bytes of big endian bits (done!)
    - Add to_le_bytes_of_le_bits() to output little endian bytes & bits (done!)
    - Add to_be_bytes_of_le_bits() to output big endian bytes & little endian bits (done!)
  - Add functions to read &[u8] to Bits in any endianness:
    - Add from_be_bytes() to fill a Bits from big endian bytes of big endian bits (done!)
    - Add from_le_bytes() to fill a Bits from little endian bytes of big endian bits (done!)
    - Add from_le_bytes_of_le_bits() to fill a Bits from little endian bytes & bits (done!)
    - Add from_be_bytes_of_le_bits() to fill a Bits from big endian bytes & little endian bits (done!)
- Split BitMan into separate modules for easier reading/testing (done!)
- Add git cliff for automatic CHANGELOG.md generation (done!)
- Add PropTest for property-based testing (done! tests/conversions.rs, tests/bitman_trait.rs)
- Add test coverage monitoring (done 2026-11-19, `coverage` job in verify.yml, Linux only). Tool: cargo-llvm-cov, not Tarpaulin (pm-ants 2026-11-19: the goal is coverage monitoring, not the brand; llvm-cov runs on Windows/macOS/Linux and is installed here). Baseline at cf2c350: 67.2% lines (bits.rs 47%, bit.rs 85%, field.rs 100%, lib.rs 98%). After the 2026-11-21 operator tests: 99.3% lines (bits.rs 98.8%, bit.rs 100%, field.rs 100%, lib.rs 98.4%), cargo llvm-cov at the head of branch tests.
- Add Criterion and Flamegraph for profiling (done 2026-11-20: `benches/bitman.rs`, `cargo bench`; flamegraph via `cargo flamegraph`, documented in CONTRIBUTING.md, not a dependency). First numbers (this box, release, one run): array bit lookup is linear in elements as predicted (last bit: 1.5 ns at 1 byte, 18 ns at 16, 314 ns at 256, 4.6 us at 4096); `Field::write` u32 is ~540 ns and `read` ~160 ns, both slow because they go through `Bits`; `u64::bits()` ~160 ns. Candidate work: O(1) element lookup for fixed-width arrays, direct shift/mask in `Field`.

- Decide `Bits` operators on unequal lengths (owner: bitman lane; decided 2026-11-21, not yet implemented): `a op= b` must equal `a = a op b`. Today the plain `&`, `|`, `^` stop at the shorter operand while `&=`, `|=`, `^=` only touch the overlap and keep the longer tail (pinned in tests/bits_ops.rs). Plan: make the assigning forms truncate like the plain ones; Breaking, goes in a 4.0.0 PR with a CHANGELOG line (not done silently).
- Decide the width of `Bits::zero()` / `Bits::one()` (owner: bitman lane; open): both are 192 bits because they use `size_of::<Bits>() * 8`, the size of a `Vec`, which means nothing. Candidates: 128 (matches `Mul`, which goes through `u128`) or 1.

Open issues (decided 2026-10-25)
--------------------------------
- #2 Bits iteration: fixed by replacing `Iterator for Bits` with `IntoIterator` for `Bits`, `&Bits`, `&mut Bits` (breaking, 3.0.0).
- #1 arrays/vectors/groups: done in 3.1.0 (README matches real API; public `BitMan` for primitives, `Bit`, `Bits`, `[T; N]`, `[T]`, `Vec<T>`). Closed.

Backlog
-------
- (done in 3.2.0 as `Field`) Typed fields at a bit offset (the 2.0 roadmap's "Field to get read()/write()" item, never implemented): `read::<T>(offset, len)` / `write(offset, value)` on any `BitMan`, converting via the existing `From<&Bits>`/`From<int> for Bits`. Design question: a `Field { offset, len }` value that can be defined once and reused, vs. plain methods on the trait.
