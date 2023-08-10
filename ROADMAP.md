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
- Add PropTest for property-based testing
- Add Tarpaulin for test coverage monitoring
- Add Criterion and Flamegraph for profiling
- Add git cliff for automatic CHANGELOG.md generation (done!)
