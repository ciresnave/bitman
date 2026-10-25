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
| 6 | US5948099A | Swapping byte order of a data item | Intel | expired | 1, 6, 11, 13, 17, 19, 22 (full set) | byte-order swap of 32-bit item | claims hardware matrix/barrel shifter driven by a single instruction | clear |
| 7 | US6714145B1 | Integer-based encoding/decoding of bits | individual | expired | 1, 16, 18, 30-34 | bit <-> integer mapping | claims encoding a bit as an even/odd integer from a data array; we do not | clear |
| 8 | US7389315B1 | Byte swapping file access data structures | NetApp | expired | 1, 3, 7, 8, 9, 12, 13, 18 | endianness conversion of data structures | claims a critical-path check plus descriptor-table byte-swapping engine | clear |
| 9 | US6081869A / EP0715252A1 | Bit-field peripheral | NEC/Renesas | expired | US 1, 8, 15, 16; EP 1 | bit access by index | claims a peripheral device with bit-field memory and address-decoded logic | clear |
| 10 | US6959367B2 | System having read-modify-write unit | Toshiba | expired | 1 only (independent) | none | hardware bus read-modify-write unit | clear |
| 11 | EP2958286A1 | Generic modification instructions for packets | Cavium | ACTIVE | 1 only | bit vector of valid fields | claims a network-device rewrite engine generalizing protocol headers; we have no packet/header handling | clear |
| 12 | US6681315B1 | Bit vector array | IBM | expired | 1, 6 | bit vector, logical ops | claims control-processor hardware with vector registers | clear |
| 13 | US7669191B1 | Compile-time dispatch on heterogeneous containers | Xilinx | expired | 1 only | compile-time method selection | claims compiler processing a variant-type container class | clear |
| 14 | US9678729B2 | Co-specialization of specialized classes | Oracle | ACTIVE | 1 only | "specialization" | claims runtime specialization of generic classes in a managed runtime; and bitman 2.0.3 no longer uses specialization at all | clear |
| 15 | US7370180B2, US20110099353A1, US9256434B2 | Bitfield extract/insert instructions | ARM, MIPS, IBM | expired | 1 only each | bit-field extract | all processor-instruction claims | clear |

For hits 10-15 only independent claim 1 was read (full text for 1-9 as listed); dependent claims unread. Hits not claim-read: US7305383B1, US10229143B2 (surfaced by the bit-array query; search-index/automaton topics, titles only): `unread`.

## Second-pass queries (2026-10-25)
- "bit array data structure set of bits vector of booleans operations shift and or patent claim software" (bit sets/arrays)
- "patent compile-time specialization default implementation overridden trait dispatch generic programming type-specific method selection" (specialization dispatch)
- "patent extracting and inserting bit field from integer using mask and shift software library bit-field accessor" (bit-field manipulation)

## Needs review (to the PM)
None found. The `unread` entries (US7305383B1, US10229143B2) were not read.

## Limitations
Five queries total; search engine results, not an exhaustive patent-office search; machine-converted claim text; only US and some EP coverage; software-library ideas here are long-established prior art (language-level bit ops), which lowers risk but is not a clearance.
