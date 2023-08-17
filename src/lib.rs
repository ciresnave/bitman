//! Rips your variables to Bits!
//!
//! The *Bit* and *Bits* structures are the heart of *bitman*.
//!
#![allow(incomplete_features)]
#![feature(specialization)]
#![cfg_attr(not(test), no_std)]

use core::fmt::Debug;
use core::{
    mem::size_of,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not},
};
extern crate alloc;
use alloc::borrow::ToOwned;
use alloc::vec;
use num_traits::{CheckedShl, One, Zero};

mod bit;
pub use bit::*;

mod bits;
pub use bits::*;

#[cfg(test)]
mod bitman_tests;
#[cfg(test)]
pub use bitman_tests::*;

trait BitMan
where
    Self: Sized
        + BitAnd<Output = Self>
        + BitAndAssign
        + BitOr
        + BitOrAssign
        + Not<Output = Self>
        + Debug
        + PartialEq
        + ToOwned<Owned = Self>
        + CheckedShl
        + Zero
        + One,
{
    #[inline]
    fn bit_len(&self) -> usize {
        size_of::<Self>() * 8
    }

    #[inline]
    fn bit(&self, index: &u32) -> Bit {
        let mut mask = Self::one();
        let offset: u32 = (self.bit_len() as u32 - 1) - *index;
        if let Some(new_mask) = CheckedShl::checked_shl(&mask, offset) {
            mask = new_mask;
        } else {
            panic!(
                "Index {:?} passed to bit() is beyond the bounds of {:?}",
                index, self
            );
        }
        mask = self.to_owned() & mask;
        Bit(mask != Self::zero())
    }

    #[inline]
    fn set_bit(&mut self, index: &u32, bit: &Bit) {
        assert!(*index <= ((self.bit_len()) - 1).try_into().unwrap(), "Index out of range in call to set_bit()");
        if **bit {
            let mut mask: Self = Self::one();
            if let Some(new_mask) = mask.checked_shl((self.bit_len() as u32 - 1) - index) {
                mask = new_mask;
                *self |= mask;
            }
        } else {
            let mut mask: Self = Self::one();
            if let Some(new_mask) = mask.checked_shl((self.bit_len() as u32 - 1) - index) {
                mask = !new_mask;
                *self &= mask;
            }
        }
    }

    #[inline]
    fn bits(&self) -> Bits {
        let mut output_value: Bits = Bits::new(&vec![Bit(false); self.bit_len()]);
        for current_index in 0..self.bit_len() as u32 {
            (*output_value)[current_index as usize] = self.bit(&current_index);
        }
        output_value
    }

    #[inline]
    fn set_bits(&mut self, mut index: u32, bits: &Bits) {
        for current_bit in bits.iter() {
            if current_bit.0 {
                let mut mask: Self = Self::one();
                if let Some(new_mask) =
                    CheckedShl::checked_shl(&mask, self.bit_len() as u32 - index)
                {
                    mask = new_mask;
                } else {
                    mask = Self::zero();
                }
                *self &= !mask;
            } else {
                let mut mask: Self = Self::one();
                if let Some(new_mask) =
                    CheckedShl::checked_shl(&mask, self.bit_len() as u32 - index)
                {
                    mask = new_mask;
                } else {
                    mask = Self::zero();
                }
                *self |= mask;
            }
            index += 1;
        }
    }
}

impl BitMan for u8 {}

impl BitMan for u16 {}

impl BitMan for u32 {}

impl BitMan for u64 {}

impl BitMan for u128 {}

impl BitMan for usize {}

impl BitMan for i8 {}

impl BitMan for i16 {}

impl BitMan for i32 {}

impl BitMan for i64 {}

impl BitMan for i128 {}

impl BitMan for isize {}
