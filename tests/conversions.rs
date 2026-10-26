//! Integer <-> Bits conversions, the four byte-order/bit-order variants, and Add.
//! Bits index 0 is the most significant bit.
use bitman::{Bit, Bits};
use proptest::prelude::*;

fn bits_from(pattern: &str) -> Bits {
    Bits::new(
        &pattern
            .chars()
            .map(|c| Bit(c == '1'))
            .collect::<Vec<Bit>>(),
    )
}

#[test]
fn u8_to_bits_has_eight_bits_msb_first() {
    assert_eq!(Bits::from(0b1000_0001u8), bits_from("10000001"));
}

#[test]
fn bit_widths_match_the_integer_type() {
    assert_eq!(Bits::from(5u16).len(), 16);
    assert_eq!(Bits::from(5u32).len(), 32);
    assert_eq!(Bits::from(5u64).len(), 64);
    assert_eq!(Bits::from(5u128).len(), 128);
}

#[test]
fn bits_to_u8_reads_msb_first() {
    assert_eq!(u8::from(&bits_from("10000001")), 129);
}

#[test]
fn short_bits_convert_as_a_right_aligned_number() {
    assert_eq!(u8::from(&bits_from("101")), 5);
}

#[test]
fn long_bits_keep_the_low_order_bits() {
    assert_eq!(u8::from(&bits_from("1111111100000101")), 5);
}

#[test]
fn to_be_bytes_packs_msb_first() {
    assert_eq!(bits_from("10000001").to_be_bytes(), vec![129]);
    assert_eq!(bits_from("0000000100000010").to_be_bytes(), vec![1, 2]);
}

#[test]
fn to_le_bytes_reverses_byte_order() {
    assert_eq!(bits_from("0000000100000010").to_le_bytes(), vec![2, 1]);
}

#[test]
fn to_be_bytes_of_le_bits_reverses_bits_within_each_byte() {
    assert_eq!(bits_from("10000000").to_be_bytes_of_le_bits(), vec![1]);
    assert_eq!(
        bits_from("1000000001000000").to_be_bytes_of_le_bits(),
        vec![1, 2]
    );
}

#[test]
fn to_le_bytes_of_le_bits_reverses_both() {
    assert_eq!(
        bits_from("1000000001000000").to_le_bytes_of_le_bits(),
        vec![2, 1]
    );
}

#[test]
fn partial_byte_is_padded_with_leading_zero_bits() {
    assert_eq!(bits_from("101").to_be_bytes(), vec![5]);
}

#[test]
fn from_bytes_variants_match_the_documented_layout() {
    assert_eq!(Bits::from_be_bytes(&[1, 2]), bits_from("0000000100000010"));
    assert_eq!(Bits::from_le_bytes(&[2, 1]), bits_from("0000000100000010"));
    assert_eq!(
        Bits::from_be_bytes_of_le_bits(&mut [1, 2]),
        bits_from("1000000001000000")
    );
    assert_eq!(
        Bits::from_le_bytes_of_le_bits(&[2, 1]),
        bits_from("1000000001000000")
    );
}

#[test]
fn add_is_binary_addition_at_the_wider_width() {
    assert_eq!(Bits::from(3u8) + Bits::from(5u8), Bits::from(8u8));
    assert_eq!(Bits::from(200u8) + Bits::from(100u8), Bits::from(44u8)); // wraps
}

proptest! {
    #[test]
    fn u8_round_trips(v: u8) { prop_assert_eq!(u8::from(&Bits::from(v)), v); }
    #[test]
    fn u16_round_trips(v: u16) { prop_assert_eq!(u16::from(&Bits::from(v)), v); }
    #[test]
    fn u32_round_trips(v: u32) { prop_assert_eq!(u32::from(&Bits::from(v)), v); }
    #[test]
    fn u64_round_trips(v: u64) { prop_assert_eq!(u64::from(&Bits::from(v)), v); }

    #[test]
    fn be_bytes_match_std_u32(v: u32) {
        prop_assert_eq!(Bits::from(v).to_be_bytes(), v.to_be_bytes().to_vec());
    }
    #[test]
    fn le_bytes_match_std_u32(v: u32) {
        prop_assert_eq!(Bits::from(v).to_le_bytes(), v.to_le_bytes().to_vec());
    }
    #[test]
    fn be_bytes_of_le_bits_match_std(v: u32) {
        let expected: Vec<u8> = v.to_be_bytes().iter().map(|b| b.reverse_bits()).collect();
        prop_assert_eq!(Bits::from(v).to_be_bytes_of_le_bits(), expected);
    }
    #[test]
    fn le_bytes_of_le_bits_match_std(v: u32) {
        let expected: Vec<u8> = v.to_le_bytes().iter().map(|b| b.reverse_bits()).collect();
        prop_assert_eq!(Bits::from(v).to_le_bytes_of_le_bits(), expected);
    }

    #[test]
    fn be_bytes_round_trip(bytes in proptest::collection::vec(any::<u8>(), 0..16)) {
        prop_assert_eq!(Bits::from_be_bytes(&bytes).to_be_bytes(), bytes);
    }
    #[test]
    fn le_bytes_round_trip(bytes in proptest::collection::vec(any::<u8>(), 0..16)) {
        prop_assert_eq!(Bits::from_le_bytes(&bytes).to_le_bytes(), bytes);
    }
    #[test]
    fn be_bytes_of_le_bits_round_trip(bytes in proptest::collection::vec(any::<u8>(), 0..16)) {
        let mut input = bytes.clone();
        prop_assert_eq!(Bits::from_be_bytes_of_le_bits(&mut input).to_be_bytes_of_le_bits(), bytes);
    }
    #[test]
    fn le_bytes_of_le_bits_round_trip(bytes in proptest::collection::vec(any::<u8>(), 0..16)) {
        prop_assert_eq!(Bits::from_le_bytes_of_le_bits(&bytes).to_le_bytes_of_le_bits(), bytes);
    }

    #[test]
    fn add_matches_wrapping_add_u32(a: u32, b: u32) {
        prop_assert_eq!(u32::from(&(Bits::from(a) + Bits::from(b))), a.wrapping_add(b));
    }
}
