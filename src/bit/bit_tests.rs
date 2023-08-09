use crate as bitman;
use num_traits::{One, Zero};
use proptest::prelude::*;

#[test]
fn deref_of_default_bit_is_false() {
    assert_eq!(*bitman::Bit::default(), false);
}

#[test]
fn writing_true_to_mutable_ref_of_default_bit_makes_it_true() {
    let mut bit: bitman::Bit = bitman::Bit::default();
    *bit = true;
    assert_eq!(*bit, true);
}

#[test]
fn display_of_bit_works() {
    print!("{} {}", bitman::Bit(false), bitman::Bit(true));
}

#[test]
fn bitand_of_a_true_bit_and_a_true_bit_is_true() {
    assert!(*(bitman::Bit(true) & bitman::Bit(true)));
}

#[test]
fn bitand_of_a_true_bit_and_a_false_bit_is_false() {
    assert!(!*(bitman::Bit(true) & bitman::Bit(false)));
}

#[test]
fn bitand_of_a_false_bit_and_a_false_bit_is_false() {
    assert!(!*(bitman::Bit(false) & bitman::Bit(false)));
}

#[test]
fn bitandassign_of_a_true_bit_and_a_true_bit_is_true() {
    let mut bit: bitman::Bit = bitman::Bit(true);
    bit &= bitman::Bit(true);
    assert!(*bit);
}

#[test]
fn bitandassign_of_a_true_bit_and_a_false_bit_is_false() {
    let mut bit: bitman::Bit = bitman::Bit(true);
    bit &= bitman::Bit(false);
    assert!(!*bit);
}

#[test]
fn bitandassign_of_a_false_bit_and_a_false_bit_is_false() {
    let mut bit: bitman::Bit = bitman::Bit(false);
    bit &= bitman::Bit(false);
    assert!(!*bit);
}

#[test]
fn bitor_of_a_true_bit_and_a_true_bit_is_true() {
    assert!(*(bitman::Bit(true) | bitman::Bit(true)));
}

#[test]
fn bitor_of_a_true_bit_and_a_false_bit_is_true() {
    assert!(*(bitman::Bit(true) | bitman::Bit(false)));
}

#[test]
fn bitor_of_a_false_bit_and_a_false_bit_is_false() {
    assert!(!*(bitman::Bit(false) | bitman::Bit(false)));
}

#[test]
fn bitorassign_of_a_true_bit_and_a_true_bit_is_true() {
    let mut bit: bitman::Bit = bitman::Bit(true);
    bit |= bitman::Bit(true);
    assert!(*bit);
}

#[test]
fn bitorassign_of_a_true_bit_and_a_false_bit_is_true() {
    let mut bit: bitman::Bit = bitman::Bit(true);
    bit |= bitman::Bit(false);
    assert!(*bit);
}

#[test]
fn bitorassign_of_a_false_bit_and_a_false_bit_is_false() {
    let mut bit: bitman::Bit = bitman::Bit(false);
    bit |= bitman::Bit(false);
    assert!(!*bit);
}

#[test]
fn bitxor_of_a_true_bit_and_a_true_bit_is_false() {
    assert!(!*(bitman::Bit(true) ^ bitman::Bit(true)));
}

#[test]
fn bitxor_of_a_true_bit_and_a_false_bit_is_true() {
    assert!(*(bitman::Bit(true) ^ bitman::Bit(false)));
}

#[test]
fn bitxor_of_a_false_bit_and_a_false_bit_is_false() {
    assert!(!*(bitman::Bit(false) ^ bitman::Bit(false)));
}

#[test]
fn bitxorassign_of_a_true_bit_and_a_true_bit_is_false() {
    let mut bit: bitman::Bit = bitman::Bit(true);
    bit ^= bitman::Bit(true);
    assert!(!*bit);
}

#[test]
fn bitxorassign_of_a_true_bit_and_a_false_bit_is_true() {
    let mut bit: bitman::Bit = bitman::Bit(true);
    bit ^= bitman::Bit(false);
    assert!(*bit);
}

#[test]
fn bitxorassign_of_a_false_bit_and_a_false_bit_is_false() {
    let mut bit: bitman::Bit = bitman::Bit(false);
    bit ^= bitman::Bit(false);
    assert!(!*bit);
}

#[test]
fn not_of_a_false_bit_is_true() {
    assert!(!*bitman::Bit(false));
}

#[test]
fn not_of_a_true_bit_is_false() {
    assert_eq!(!bitman::Bit(true), bitman::Bit(false));
}

#[test]
fn true_bit_shifted_left_by_0_is_true() {
    assert!(*(bitman::Bit(true) << 0u32));
}

#[test]
fn false_bit_shifted_left_by_0_is_false() {
    assert!(!*(bitman::Bit(false) << 0u32));
}

proptest! {
    #[test]
    fn true_bit_shifted_left_by_any_number_is_false(shift_amount in 1usize..(usize::BITS as usize)) {
        assert!(!*(bitman::Bit(true) << shift_amount));
    }

    #[test]
    fn false_bit_shifted_left_by_any_number_is_false(shift_amount in 1usize..(usize::BITS as usize)) {
        assert!(!*(bitman::Bit(false) << shift_amount));
    }
}

#[test]
fn true_bit_shifted_left_by_0_and_assigned_is_true() {
    let mut bit = bitman::Bit(true);
    bit <<= 0u32;
    assert!(*bit);
}

#[test]
fn false_bit_shifted_left_by_0_and_assigned_is_false() {
    let mut bit = bitman::Bit(false);
    bit <<= 0u32;
    assert!(!*bit);
}

proptest! {
    #[test]
    fn true_bit_shifted_left_by_any_number_and_assigned_is_false(shift_amount in 1usize..(usize::BITS as usize)) {
        let mut bit = bitman::Bit(true);
        bit <<= shift_amount;
        assert!(!*bit);
    }

    #[test]
    fn false_bit_shifted_left_by_any_number_and_assigned_is_false(shift_amount in 1usize..(usize::BITS as usize)) {
        let mut bit = bitman::Bit(false);
        bit <<= shift_amount;
        assert!(!*bit);
    }
}

#[test]
fn true_bit_shifted_right_by_0_is_true() {
    assert!(*(bitman::Bit(true) >> 0));
}

#[test]
fn false_bit_shifted_right_by_0_is_false() {
    assert!(!*(bitman::Bit(false) >> 0));
}

proptest! {
    #[test]
    fn true_bit_shifted_right_by_any_number_is_false(shift_amount in 1usize..(usize::BITS as usize)) {
        assert!(!*(bitman::Bit(true) >> shift_amount));
    }

    #[test]
    fn false_bit_shifted_right_by_any_number_is_false(shift_amount in 1usize..(usize::BITS as usize)) {
        assert!(!*(bitman::Bit(false) >> shift_amount));
    }
}

#[test]
fn true_bit_shifted_right_by_0_and_assigned_is_true() {
    let mut bit = bitman::Bit(true);
    bit >>= 0;
    assert!(*bit);
}

#[test]
fn false_bit_shifted_right_by_0_and_assigned_is_false() {
    let mut bit = bitman::Bit(false);
    bit >>= 0;
    assert!(!*bit);
}

proptest! {
    #[test]
    fn true_bit_shifted_right_by_any_number_and_assigned_is_false(shift_amount in 1usize..(usize::BITS as usize)) {
        let mut bit = bitman::Bit(true);
        bit >>= shift_amount;
        assert!(!*bit);
    }

    #[test]
    fn false_bit_shifted_right_by_any_number_and_assigned_is_false(shift_amount in 1usize..(usize::BITS as usize)) {
        let mut bit = bitman::Bit(false);
        bit >>= shift_amount;
        assert!(!*bit);
    }
}

#[test]
fn false_bit_multiplied_by_false_is_false() {
    assert!(!*(bitman::Bit(false) * bitman::Bit(false)));
}

#[test]
fn false_bit_multiplied_by_true_is_false() {
    assert!(!*(bitman::Bit(false) * bitman::Bit(true)));
}

#[test]
fn true_bit_multiplied_by_false_is_false() {
    assert!(!*(bitman::Bit(true) * (bitman::Bit(false))));
}

#[test]
fn true_bit_multiplied_by_true_is_true() {
    assert!(*(bitman::Bit(true) * bitman::Bit(true)));
}

#[test]
#[should_panic]
fn false_bit_divided_by_false_panics() {
    assert!(*(bitman::Bit(false) / bitman::Bit(false)));
}

#[test]
#[should_panic]
fn true_bit_divided_by_false_panics() {
    assert!(*(bitman::Bit(true) / bitman::Bit(false)));
}

#[test]
fn false_bit_divided_by_true_is_false() {
    assert!(!*(bitman::Bit(false) / bitman::Bit(true)));
}

#[test]
fn true_bit_divided_by_true_is_true() {
    assert!(*(bitman::Bit(true) / bitman::Bit(true)))
}

#[test]
fn true_bit_plus_true_is_false() {
    assert!(!*(bitman::Bit(true) + bitman::Bit(true)));
}

#[test]
fn true_bit_plus_false_is_true() {
    assert!(*(bitman::Bit(true) + bitman::Bit(false)));
}

#[test]
fn false_bit_plus_true_is_true() {
    assert!(*(bitman::Bit(false) + bitman::Bit(true)));
}

#[test]
fn false_bit_plus_false_is_false() {
    assert!(!*(bitman::Bit(false) + bitman::Bit(false)));
}

#[test]
fn true_bit_minus_true_is_false() {
    assert!(!*(bitman::Bit(true) - bitman::Bit(true)));
}

#[test]
fn true_bit_minus_false_is_true() {
    assert!(*(bitman::Bit(true) - bitman::Bit(false)));
}

#[test]
fn false_bit_minus_true_is_false() {
    assert!(!*(bitman::Bit(false) - bitman::Bit(true)));
}

#[test]
fn false_bit_minus_false_is_false() {
    assert!(!*(bitman::Bit(false) - bitman::Bit(false)));
}

#[test]
fn bit_zero_is_false() {
    assert!(!*bitman::Bit::zero());
}

#[test]
fn bit_false_is_zero() {
    assert!(bitman::Bit(false).is_zero());
}

#[test]
fn bit_one_is_true() {
    assert!(*bitman::Bit::one());
}
