use crate as bitman;
use bitman::{Bit, BitMan, Bits};
use num_traits::{One, Zero};

#[test]
fn bit_method_test_on_u8() {
    assert_eq!(0u8.bit(&0), bitman::Bit(false));
    assert_eq!(0u8.bit(&1), bitman::Bit(false));
    assert_eq!(0u8.bit(&2), bitman::Bit(false));
    assert_eq!(0u8.bit(&3), bitman::Bit(false));
    assert_eq!(0u8.bit(&4), bitman::Bit(false));
    assert_eq!(0u8.bit(&5), bitman::Bit(false));
    assert_eq!(0u8.bit(&6), bitman::Bit(false));
    assert_eq!(0u8.bit(&7), bitman::Bit(false));
}

#[test]
fn set_bit_method_test_on_u8() {
    let mut my_u8 = 0u8;
    my_u8.set_bit(&0, &bitman::Bit(true));
    assert_eq!(my_u8.bit(&0), bitman::Bit(true));
    my_u8.set_bit(&1, &bitman::Bit(true));
    assert_eq!(my_u8.bit(&1), bitman::Bit(true));
    let mut my_u8 = 0u8;
    my_u8.set_bit(&2, &bitman::Bit(true));
    assert_eq!(my_u8.bit(&2), bitman::Bit(true));
    let mut my_u8 = 0u8;
    my_u8.set_bit(&3, &bitman::Bit(true));
    assert_eq!(my_u8.bit(&3), bitman::Bit(true));
    let mut my_u8 = 0u8;
    my_u8.set_bit(&4, &bitman::Bit(true));
    assert_eq!(my_u8.bit(&4), bitman::Bit(true));
    let mut my_u8 = 0u8;
    my_u8.set_bit(&5, &bitman::Bit(true));
    assert_eq!(my_u8.bit(&5), bitman::Bit(true));
    let mut my_u8 = 0u8;
    my_u8.set_bit(&6, &bitman::Bit(true));
    assert_eq!(my_u8.bit(&6), bitman::Bit(true));
    let mut my_u8 = 0u8;
    my_u8.set_bit(&7, &bitman::Bit(true));
    assert_eq!(my_u8.bit(&7), bitman::Bit(true));
}

#[test]
fn u8_zero_as_bits_compared_set_to_one_and_compared() {
    let mut my_u8_as_bits = 0u8.bits();
    assert_eq!(
        my_u8_as_bits[..],
        [
            bitman::Bit(false),
            bitman::Bit(false),
            bitman::Bit(false),
            bitman::Bit(false),
            bitman::Bit(false),
            bitman::Bit(false),
            bitman::Bit(false),
            bitman::Bit(false)
        ]
    );
    my_u8_as_bits = u8::one().bits();
    assert_eq!(
        my_u8_as_bits[..],
        [
            bitman::Bit(false),
            bitman::Bit(false),
            bitman::Bit(false),
            bitman::Bit(false),
            bitman::Bit(false),
            bitman::Bit(false),
            bitman::Bit(false),
            bitman::Bit(true)
        ]
    );
}

#[test]
fn bits_method_test_on_u8() {
    let new_bits = 0u8.bits();
    assert_eq!(new_bits.bit_len(), 8);
    assert_eq!(new_bits[0], bitman::Bit(false));
    assert_eq!(0u8.bits(), bitman::Bits::new(&[bitman::Bit::zero(); 8]));
}

#[test]
fn bits_iterate_in_index_order_by_value_and_by_reference() {
    let bits = Bits::new(&[Bit(true), Bit(false), Bit(false)]);
    let by_ref: Vec<Bit> = (&bits).into_iter().copied().collect();
    assert_eq!(by_ref, std::vec![Bit(true), Bit(false), Bit(false)]);
    let by_value: Vec<Bit> = bits.into_iter().collect();
    assert_eq!(by_value, std::vec![Bit(true), Bit(false), Bit(false)]);
}

#[test]
fn bits_can_be_mutated_through_a_for_loop() {
    let mut bits = Bits::new(&[Bit(true), Bit(false)]);
    for bit in &mut bits {
        *bit = !*bit;
    }
    assert_eq!(bits, Bits::new(&[Bit(false), Bit(true)]));
}

#[test]
fn set_bits_writes_ones_and_zeros_msb_first_at_the_given_index() {
    let mut value = 0u8;
    value.set_bits(0, &Bits::new(&[Bit(true), Bit(false), Bit(true)]));
    assert_eq!(value, 0b1010_0000);
    let mut value = 0xFFu8;
    value.set_bits(4, &Bits::new(&[Bit(false), Bit(false)]));
    assert_eq!(value, 0b1111_0011);
}

proptest::proptest! {
    #[test]
    fn set_bits_then_bits_reads_back(start in 0u32..24, len in 1usize..8, seed: u32, pattern: u8) {
        let mut value = seed;
        let written = Bits::new(&(0..len).map(|i| Bit((pattern >> i) & 1 == 1)).collect::<std::vec::Vec<_>>());
        value.set_bits(start, &written);
        for (offset, bit) in written.iter().enumerate() {
            proptest::prop_assert_eq!(value.bit(&(start + offset as u32)), *bit);
        }
    }
}
