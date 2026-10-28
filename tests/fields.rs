//! Typed fields: read and write a range of bits of any `BitMan` value as an integer.
//! Offsets count from the most significant bit (bit 0).
use bitman::{Bit, BitMan, Field};
use proptest::prelude::*;

#[test]
fn reads_a_field_as_an_unsigned_integer() {
    // 0b1011_0100: bits 0..4 are 1011, bits 4..8 are 0100
    let value = 0b1011_0100u8;
    assert_eq!(Field::new(0, 4).read::<u8, _>(&value), 0b1011);
    assert_eq!(Field::new(4, 4).read::<u8, _>(&value), 0b0100);
}

#[test]
fn reads_into_a_wider_type() {
    assert_eq!(Field::new(2, 12).read::<u32, _>(&0xFFFFu16), 0xFFF);
}

#[test]
fn writes_a_field_without_touching_other_bits() {
    let mut value = 0xFFu8;
    Field::new(2, 3).write(&mut value, 0b010u8);
    assert_eq!(value, 0b1101_0111);
}

#[test]
fn write_uses_the_low_order_bits_of_the_value() {
    let mut value = 0u8;
    Field::new(0, 3).write(&mut value, 0b1111_0101u8);
    assert_eq!(value, 0b1010_0000);
}

#[test]
fn works_across_element_boundaries_of_a_byte_array() {
    let mut buffer = [0u8; 3];
    Field::new(4, 12).write(&mut buffer, 0xABCu16);
    assert_eq!(buffer, [0x0A, 0xBC, 0x00]);
    assert_eq!(Field::new(4, 12).read::<u16, _>(&buffer), 0xABC);
}

#[test]
fn len_and_offset_are_exposed() {
    let field = Field::new(3, 5);
    assert_eq!((field.offset(), field.len()), (3, 5));
    assert!(!field.is_empty());
    assert!(Field::new(3, 0).is_empty());
}

#[test]
#[should_panic]
fn reading_past_the_end_panics() {
    let _ = Field::new(4, 8).read::<u8, _>(&0u8);
}

#[test]
fn a_one_bit_field_matches_bit() {
    let value = 0b0010_0000u8;
    assert_eq!(Field::new(2, 1).read::<u8, _>(&value), 1);
    assert_eq!(value.bit(2), Bit(true));
}

proptest! {
    #[test]
    fn write_then_read_round_trips(start in 0usize..32, len in 1usize..=32, seed: u64, v: u32) {
        prop_assume!(start + len <= 64);
        let mut word = seed;
        let field = Field::new(start, len);
        field.write(&mut word, v);
        let mask = if len == 32 { u32::MAX } else { (1u32 << len) - 1 };
        prop_assert_eq!(field.read::<u32, _>(&word), v & mask);
    }
    #[test]
    fn write_leaves_every_other_bit_alone(start in 0usize..32, len in 1usize..=32, seed: u64, v: u32) {
        prop_assume!(start + len <= 64);
        let mut word = seed;
        Field::new(start, len).write(&mut word, v);
        for i in (0..64).filter(|i| *i < start || *i >= start + len) {
            prop_assert_eq!(word.bit(i), seed.bit(i));
        }
    }
    #[test]
    fn read_matches_shifting_u64(start in 0usize..64, len in 1usize..=32, seed: u64) {
        prop_assume!(start + len <= 64);
        let expected = ((seed << start) >> (64 - len)) as u32;
        prop_assert_eq!(Field::new(start, len).read::<u32, _>(&seed), expected);
    }
}
