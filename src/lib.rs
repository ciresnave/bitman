//! Rips your variables to Bits!
//!
//! The *Bit* and *Bits* structures are the heart of *bitman*.
//!
#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

extern crate alloc;
use alloc::vec::Vec;

mod bit;
pub use bit::*;

mod bits;
pub use bits::*;

#[cfg(test)]
mod bitman_tests;

/// Read and write the individual bits of a value without converting it.
///
/// Bit 0 is the most significant bit. Out-of-range indices panic.
/// Arrays, slices and vectors of `BitMan` values are one long bit sequence:
/// the bits of element 0 come first. (Locating a bit in such a sequence walks
/// the elements, so it costs time linear in their number.)
pub trait BitMan {
    /// Number of bits in this value.
    fn bit_len(&self) -> usize;

    /// The bit at `index`.
    fn bit(&self, index: usize) -> Bit;

    /// Sets the bit at `index`.
    fn set_bit(&mut self, index: usize, bit: Bit);

    /// All the bits, in order.
    fn bits(&self) -> Bits {
        let all: Vec<Bit> = (0..self.bit_len()).map(|index| self.bit(index)).collect();
        Bits::new(&all)
    }

    /// Writes `bits` starting at bit `index`.
    fn set_bits(&mut self, index: usize, bits: &Bits) {
        for (offset, current_bit) in bits.iter().enumerate() {
            self.set_bit(index + offset, *current_bit);
        }
    }
}

macro_rules! impl_bitman_for_integers {
    ($($integer:ty),*) => {$(
        impl BitMan for $integer {
            #[inline]
            fn bit_len(&self) -> usize {
                <$integer>::BITS as usize
            }

            #[inline]
            fn bit(&self, index: usize) -> Bit {
                assert!(index < self.bit_len(), "bit index {index} out of range for {} bits", self.bit_len());
                Bit((*self >> (self.bit_len() - 1 - index)) & 1 == 1)
            }

            #[inline]
            fn set_bit(&mut self, index: usize, bit: Bit) {
                assert!(index < self.bit_len(), "bit index {index} out of range for {} bits", self.bit_len());
                let mask: $integer = 1 << (self.bit_len() - 1 - index);
                if bit.0 {
                    *self |= mask;
                } else {
                    *self &= !mask;
                }
            }
        }
    )*};
}

impl_bitman_for_integers!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

/// Finds the element holding bit `index` of a sequence and the bit's index inside it.
fn locate<T: BitMan>(elements: &[T], mut index: usize) -> (usize, usize) {
    for (position, element) in elements.iter().enumerate() {
        let width = element.bit_len();
        if index < width {
            return (position, index);
        }
        index -= width;
    }
    panic!("bit index out of range");
}

impl<T: BitMan> BitMan for [T] {
    fn bit_len(&self) -> usize {
        self.iter().map(BitMan::bit_len).sum()
    }

    fn bit(&self, index: usize) -> Bit {
        let (position, inner) = locate(self, index);
        self[position].bit(inner)
    }

    fn set_bit(&mut self, index: usize, bit: Bit) {
        let (position, inner) = locate(self, index);
        self[position].set_bit(inner, bit);
    }
}

impl<T: BitMan, const N: usize> BitMan for [T; N] {
    fn bit_len(&self) -> usize {
        self.as_slice().bit_len()
    }

    fn bit(&self, index: usize) -> Bit {
        self.as_slice().bit(index)
    }

    fn set_bit(&mut self, index: usize, bit: Bit) {
        self.as_mut_slice().set_bit(index, bit);
    }
}

impl<T: BitMan> BitMan for Vec<T> {
    fn bit_len(&self) -> usize {
        self.as_slice().bit_len()
    }

    fn bit(&self, index: usize) -> Bit {
        self.as_slice().bit(index)
    }

    fn set_bit(&mut self, index: usize, bit: Bit) {
        self.as_mut_slice().set_bit(index, bit);
    }
}
