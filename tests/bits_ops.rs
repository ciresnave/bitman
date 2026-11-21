//! Operators on `Bits`. Bit 0 is the most significant bit, so `<<` moves bits toward index 0.
use bitman::{Bit, Bits};

fn bits(pattern: &str) -> Bits {
    Bits::new(&pattern.chars().map(|c| Bit(c == '1')).collect::<Vec<_>>())
}

fn text(b: &Bits) -> String {
    b.iter().map(|x| if x.0 { '1' } else { '0' }).collect()
}

#[test]
fn and_or_xor_combine_bit_by_bit() {
    assert_eq!(text(&(bits("1100") & bits("1010"))), "1000");
    assert_eq!(text(&(bits("1100") | bits("1010"))), "1110");
    assert_eq!(text(&(bits("1100") ^ bits("1010"))), "0110");
}

#[test]
fn assign_forms_match_the_plain_forms() {
    let mut a = bits("1100");
    a &= bits("1010");
    assert_eq!(text(&a), "1000");
    let mut o = bits("1100");
    o |= bits("1010");
    assert_eq!(text(&o), "1110");
    let mut x = bits("1100");
    x ^= bits("1010");
    assert_eq!(text(&x), "0110");
}

#[test]
fn not_inverts_every_bit() {
    assert_eq!(text(&!bits("1100")), "0011");
}

#[test]
fn shl_moves_bits_toward_index_zero_and_fills_with_zeros() {
    assert_eq!(text(&(bits("1011") << 1usize)), "0110");
    assert_eq!(text(&(bits("1011") << 2u32)), "1100");
    assert_eq!(text(&(&bits("1011") << 1u32)), "0110");
}

#[test]
fn shr_moves_bits_away_from_index_zero_and_fills_with_zeros() {
    assert_eq!(text(&(bits("1011") >> 1usize)), "0101");
    assert_eq!(text(&(bits("1011") >> 2u32)), "0010");
}

#[test]
fn checked_shl_refuses_a_shift_past_the_length() {
    use num_traits::CheckedShl;
    assert_eq!(
        bits("1011").checked_shl(1).map(|b| text(&b)),
        Some("0110".into())
    );
    assert!(bits("1011").checked_shl(9).is_none());
}

#[test]
fn zero_and_is_zero() {
    use num_traits::Zero;
    assert!(Bits::zero().is_zero());
    assert!(!bits("0100").is_zero());
}

#[test]
fn display_lists_the_bits() {
    assert_eq!(format!("{}", bits("10")), "Bits( Bit(true) Bit(false))");
}
