//! The public `BitMan` trait: read and write bits of values without converting them.
//! Bit 0 is the most significant bit.
use bitman::{Bit, BitMan, Bits};
use proptest::prelude::*;

#[test]
fn bit_len_matches_the_type_width() {
    assert_eq!(0u8.bit_len(), 8);
    assert_eq!(0i16.bit_len(), 16);
    assert_eq!(0u32.bit_len(), 32);
    assert_eq!(0u64.bit_len(), 64);
    assert_eq!(0u128.bit_len(), 128);
    assert_eq!(0usize.bit_len(), usize::BITS as usize);
    assert_eq!(Bit(true).bit_len(), 1);
    assert_eq!(Bits::from(0u16).bit_len(), 16);
}

#[test]
fn bit_reads_msb_first() {
    assert_eq!(0b1000_0000u8.bit(0), Bit(true));
    assert_eq!(0b1000_0000u8.bit(7), Bit(false));
    assert_eq!(1u8.bit(7), Bit(true));
    assert_eq!((-1i8).bit(0), Bit(true));
    assert_eq!(i8::MIN.bit(0), Bit(true));
    assert_eq!(i8::MIN.bit(1), Bit(false));
}

#[test]
fn set_bit_sets_and_clears() {
    let mut value = 0u8;
    value.set_bit(0, Bit(true));
    assert_eq!(value, 128);
    value.set_bit(7, Bit(true));
    assert_eq!(value, 129);
    value.set_bit(0, Bit(false));
    assert_eq!(value, 1);
    let mut signed = 0i8;
    signed.set_bit(0, Bit(true));
    assert_eq!(signed, i8::MIN);
}

#[test]
#[should_panic]
fn bit_out_of_range_panics() {
    let _ = 0u8.bit(8);
}

#[test]
#[should_panic]
fn set_bit_out_of_range_panics() {
    let mut value = 0u8;
    value.set_bit(8, Bit(true));
}

#[test]
fn bits_and_set_bits_round_trip_a_range() {
    let mut value = 0u16;
    value.set_bits(
        4,
        &Bits::new(&[Bit(true), Bit(true), Bit(false), Bit(true)]),
    );
    assert_eq!(value, 0b0000_1101_0000_0000);
    assert_eq!(value.bits(), Bits::from(value));
}

#[test]
fn bits_implements_the_trait() {
    let mut bits = Bits::from(0u8);
    bits.set_bit(2, Bit(true));
    assert_eq!(bits.bit(2), Bit(true));
    assert_eq!(u8::from(&bits), 0b0010_0000);
}

#[test]
fn arrays_slices_and_vecs_are_one_long_bit_sequence() {
    let mut array = [0u8; 2];
    array.set_bit(0, Bit(true));
    array.set_bit(15, Bit(true));
    assert_eq!(array, [0b1000_0000, 0b0000_0001]);
    assert_eq!(array.bit_len(), 16);
    assert_eq!(array.bit(8), Bit(false));
    assert_eq!(array.bits(), Bits::from_be_bytes(&[0b1000_0000, 1]));

    let mut vector = vec![0u16, 0u16];
    vector.set_bit(16, Bit(true));
    assert_eq!(vector, vec![0, 0x8000]);
    assert_eq!(vector.bit_len(), 32);

    let slice: &mut [u8] = &mut [0, 0, 0];
    slice.set_bits(7, &Bits::new(&[Bit(true), Bit(true)]));
    assert_eq!(slice, [1, 0b1000_0000, 0]);
}

proptest! {
    #[test]
    fn bit_agrees_with_shifting_u32(v: u32, i in 0usize..32) {
        prop_assert_eq!(v.bit(i), Bit((v >> (31 - i)) & 1 == 1));
    }
    #[test]
    fn set_bit_then_bit_i64(v: i64, i in 0usize..64, b: bool) {
        let mut x = v;
        x.set_bit(i, Bit(b));
        prop_assert_eq!(x.bit(i), Bit(b));
        for j in 0..64 { if j != i { prop_assert_eq!(x.bit(j), v.bit(j)); } }
    }
    #[test]
    fn bits_matches_from_for_every_unsigned(v: u64) {
        prop_assert_eq!(v.bits(), Bits::from(v));
    }
    #[test]
    fn byte_array_bits_match_from_be_bytes(a: [u8; 5]) {
        prop_assert_eq!(a.bits(), Bits::from_be_bytes(&a));
    }
    #[test]
    fn set_bit_on_vec_touches_exactly_one_bit(v in proptest::collection::vec(any::<u8>(), 1..8), pick: usize) {
        let i = pick % (v.len() * 8);
        let mut w = v.clone();
        w.set_bit(i, Bit(!v.bit(i).0));
        for j in 0..v.len() * 8 {
            prop_assert_eq!(w.bit(j) == v.bit(j), j != i);
        }
    }
}
