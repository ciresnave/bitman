# Patent sweep: bitman 2.0.1 line (covers 2.0.x; swept at `ffd3fe0`, source identical to published 2.0.1 apart from build config)

Date: 2026-10-25; Run by: bitman lane, Claude Sonnet 5.5; Method: AI-assisted search (web search restricted to patents.google.com) and claim read via `tools/patent-claims.py`; NOT legal advice; NOT a freedom-to-operate opinion.

## Ideas swept
1. A newtype `Bit` wrapping a boolean, with arithmetic/bitwise operator overloads.
2. A `Bits` newtype over a vector of `Bit`, supporting indexing, slicing, shifts and bitwise operators.
3. Splitting any primitive integer into a `Bits` (most-significant bit first) and rebuilding an integer from `Bits`.
4. Reading and writing a single bit of an integer by index.
5. Converting `Bits` to and from byte arrays in four variants (big/little byte order x big/little bit order).
6. Macro generating `From` conversions between `Bits` and integer types.

(Overlap-prone mechanisms only; the crate has no other mechanisms. It is a conventional bit-manipulation helper library.)

## Queries (source: Google Patents via web search, 2026-10-25)
- "method converting integer variable into array of boolean bits with endianness byte order bit order conversion library"
- "bit field read write arbitrary subfield of variable accessor type-generic bit manipulation protocol parsing"

## Hits
| # | patent | title | assignee | status | claim(s) read | overlap | our difference | verdict |
|---|---|---|---|---|---|---|---|---|
| 1 | US6938241B1 | Compiler independent bit-field macros | LSI Logic | expired (est. 2023) | 1, 9 | bit-field within a machine-word variable | claims concatenating start/end bit numbers into a "bit-field definition number"; we have no such encoding | clear (expired; no matching element) |
| 2 | US9207937B2 | Bitfield manipulation instruction (insert/extract) | ARM | active (est. 2034) | 1, 13, 14 | bitfield extract/insert | claims a processor with an instruction decoder and an instruction carrying width W / positions A,B; we are a software library, no instruction or decoder | clear (hardware/ISA claims) |
| 3 | US5867690A | Apparatus for converting data between endian formats | AMD | expired | 1-5 | byte-order conversion | claims programmable address-aperture byte-swapping hardware | clear |
| 4 | US7158990B1 | Methods and apparatus for data conversion | Oracle | expired | 1, 28, 32 | field decode/encode | claims generated encoders/decoders from a data format description with mapping information; we have no format-description or generator | clear |
| 5 | US7013302B2 | Bit field manipulation | Nortel/Avaya | expired | 1, 4 | direct bit-field access | claims processor instructions plus packet task manager/analysis machine pipeline | clear |
Also surfaced, not claim-read (titles only; hardware/other topics): US5948099A, US6714145B1, US7389315B1, US6081869A, EP0715252A1, US6959367B2, EP2958286A1. Marked `unread`, not `clear`.

## Needs review (to the PM)
None found. The `unread` entries above were not read; a deeper pass is open.

## Limitations
Two queries only; search engine results, not an exhaustive patent-office search; machine-converted claim text; only US and some EP coverage; software-library ideas here are long-established prior art (language-level bit ops), which lowers risk but is not a clearance.
