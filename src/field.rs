use alloc::vec;
use alloc::vec::Vec;

use crate::{Bit, BitMan, Bits};

/// A run of bits inside a [`BitMan`] value, defined once and reused.
///
/// `offset` counts from the most significant bit (bit 0). Reading converts the
/// bits to an integer, right-aligned (so a 4-bit field `1011` reads as 11);
/// writing stores the low `len` bits of an integer, leaving every other bit alone.
///
/// ```
/// use bitman::Field;
///
/// let mut value = 0xFFu8;
/// let middle = Field::new(2, 3);
/// middle.write(&mut value, 0b010u8);
/// assert_eq!(value, 0b1101_0111);
/// assert_eq!(middle.read::<u8, _>(&value), 0b010);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Field {
    offset: usize,
    len: usize,
}

impl Field {
    /// A field of `len` bits starting at bit `offset`.
    #[must_use]
    pub const fn new(offset: usize, len: usize) -> Self {
        Self { offset, len }
    }

    /// Index of the field's first bit.
    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Number of bits in the field.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Whether the field has no bits.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn check<B: BitMan + ?Sized>(&self, target: &B) {
        assert!(
            self.offset + self.len <= target.bit_len(),
            "field of {} bits at offset {} does not fit in {} bits",
            self.len,
            self.offset,
            target.bit_len()
        );
    }

    /// Reads the field as an integer. If the type is narrower than the field,
    /// the low-order bits are kept. Panics if the field does not fit in `target`.
    pub fn read<T, B>(&self, target: &B) -> T
    where
        T: for<'a> From<&'a Bits>,
        B: BitMan + ?Sized,
    {
        self.check(target);
        let bits: Vec<Bit> = (self.offset..self.offset + self.len)
            .map(|index| target.bit(index))
            .collect();
        T::from(&Bits::new(&bits))
    }

    /// Writes the low `len` bits of `value` into the field. Panics if the field
    /// does not fit in `target`.
    pub fn write<T, B>(&self, target: &mut B, value: T)
    where
        Bits: From<T>,
        B: BitMan + ?Sized,
    {
        self.check(target);
        let value_bits = Bits::from(value);
        let mut field_bits: Vec<Bit> = vec![Bit(false); self.len.saturating_sub(value_bits.len())];
        let skip = value_bits.len().saturating_sub(self.len);
        field_bits.extend(value_bits.iter().skip(skip).copied());
        target.set_bits(self.offset, &Bits::new(&field_bits));
    }
}
