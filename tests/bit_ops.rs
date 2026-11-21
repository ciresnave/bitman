//! `Bit` arithmetic and shifts, and `One`/`Zero` for `Bit` and `Bits`.
use bitman::{Bit, BitMan, Bits};
use num_traits::{CheckedShl, One, Zero};

const T: Bit = Bit(true);
const F: Bit = Bit(false);

#[test]
fn shifts_keep_a_bit_only_for_a_shift_of_zero() {
    assert_eq!(T << 0usize, T);
    assert_eq!(T << 1usize, F);
    assert_eq!(T << 0u32, T);
    assert_eq!(T << 3u32, F);
    assert_eq!(T >> 0usize, T);
    assert_eq!(T >> 1usize, F);
    let mut a = T;
    a <<= 0usize;
    assert_eq!(a, T);
    a <<= 1usize;
    assert_eq!(a, F);
    let mut b = T;
    b <<= 0u32;
    assert_eq!(b, T);
    b <<= 2u32;
    assert_eq!(b, F);
    let mut c = T;
    c >>= 0usize;
    assert_eq!(c, T);
    c >>= 1usize;
    assert_eq!(c, F);
}

#[test]
fn checked_shl_never_fails_for_a_bit() {
    assert_eq!(T.checked_shl(0), Some(T));
    assert_eq!(T.checked_shl(5), Some(F));
}

#[test]
fn arithmetic_is_modulo_two_for_add_and_saturating_for_sub() {
    // Add is XOR. Sub does NOT wrap (0 - 1 = 0); recorded as current behaviour.
    assert_eq!([F + F, F + T, T + F, T + T], [F, T, T, F]);
    assert_eq!([F - F, F - T, T - F, T - T], [F, F, T, F]);
    assert_eq!([F * F, F * T, T * F, T * T], [F, F, F, T]);
    assert_eq!(T / T, T);
    assert_eq!(F / T, F);
}

#[test]
#[should_panic(expected = "Divide by Zero")]
fn dividing_by_zero_panics() {
    let _ = T / F;
}

#[test]
fn one_and_zero() {
    assert_eq!(Bit::one(), T);
    assert_eq!(Bit::zero(), F);
    assert!(F.is_zero());
    assert!(!T.is_zero());
}

#[test]
fn a_bit_is_a_one_bit_sequence() {
    assert_eq!(T.bit_len(), 1);
    assert_eq!(T.bit(0), T);
    let mut b = F;
    b.set_bit(0, T);
    assert_eq!(b, T);
}

#[test]
#[should_panic(expected = "out of range")]
fn a_bit_has_no_bit_one() {
    let _ = T.bit(1);
}

#[test]
fn bits_one_is_the_value_one() {
    let one = Bits::one();
    assert!(one.is_one());
    assert_eq!(one.len(), Bits::zero().len());
    assert_eq!(one.iter().filter(|b| b.0).count(), 1);
    assert!(
        one[one.len() - 1].0,
        "the least significant (last) bit is the set one"
    );
}

#[test]
fn is_one_accepts_any_length_with_only_the_last_bit_set() {
    assert!(Bits::new(&[T]).is_one());
    assert!(Bits::new(&[F, F, T]).is_one());
    assert!(!Bits::new(&[F]).is_one());
    assert!(!Bits::new(&[T, F]).is_one());
    assert!(!Bits::new(&[T, T]).is_one());
    assert!(!Bits::new(&[]).is_one());
}

#[test]
fn bits_multiply_like_integers() {
    assert_eq!(u128::from(&(Bits::from(6u8) * Bits::from(7u8))), 42);
    assert_eq!(u128::from(&(Bits::from(5u32) * Bits::one())), 5);
}
