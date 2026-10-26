use core::{
    fmt::{self, Debug, Display},
    ops::{Deref, DerefMut},
    slice::SliceIndex,
};
use core::{
    mem::size_of,
    ops::{
        Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, IndexMut, Mul,
        Not, Shl, Shr,
    },
};
extern crate alloc;
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use num_traits::{CheckedShl, One, Zero};

use crate::BitMan;
use crate::bit::Bit;

#[cfg(test)]
mod bits_tests;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Bits {
    inner: Vec<Bit>,
}

impl Bits {
    #[inline]
    #[must_use]
    pub fn new(inner_vector_of_bits: &[Bit]) -> Self {
        Self {
            inner: inner_vector_of_bits.to_owned(),
        }
    }

    /// Bits packed into whole bytes, first bit most significant; a partial
    /// first byte is padded with leading zero bits.
    #[inline]
    #[must_use]
    pub fn to_be_bytes(&self) -> Vec<u8> {
        self.pack_bytes(false)
    }

    /// `to_be_bytes` with the byte order reversed.
    #[inline]
    #[must_use]
    pub fn to_le_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().into_iter().rev().collect()
    }

    /// `to_be_bytes_of_le_bits` with the byte order reversed.
    #[inline]
    #[must_use]
    pub fn to_le_bytes_of_le_bits(&self) -> Vec<u8> {
        self.to_be_bytes_of_le_bits().into_iter().rev().collect()
    }

    /// Bits packed into whole bytes, first bit of each byte least significant.
    #[inline]
    #[must_use]
    pub fn to_be_bytes_of_le_bits(&self) -> Vec<u8> {
        self.pack_bytes(true)
    }

    fn pack_bytes(&self, least_significant_first: bool) -> Vec<u8> {
        let padding = (8 - self.inner.len() % 8) % 8;
        let mut padded: Vec<Bit> = vec![Bit(false); padding];
        padded.extend_from_slice(&self.inner);
        padded
            .chunks(8)
            .map(|chunk| {
                chunk.iter().enumerate().fold(0u8, |byte, (position, bit)| {
                    let shift = if least_significant_first {
                        position
                    } else {
                        7 - position
                    };
                    byte | (u8::from(bit.0) << shift)
                })
            })
            .collect()
    }

    fn unpack_bytes(slice_of_bytes: &[u8], least_significant_first: bool) -> Self {
        let mut inner: Vec<Bit> = Vec::with_capacity(slice_of_bytes.len() * 8);
        for byte in slice_of_bytes {
            for position in 0..8 {
                let shift = if least_significant_first {
                    position
                } else {
                    7 - position
                };
                inner.push(Bit((byte >> shift) & 1 == 1));
            }
        }
        Self { inner }
    }

    #[inline]
    #[must_use]
    pub fn from_be_bytes(slice_of_bytes: &[u8]) -> Self {
        Self::unpack_bytes(slice_of_bytes, false)
    }

    #[inline]
    #[must_use]
    pub fn from_le_bytes(slice_of_bytes: &[u8]) -> Self {
        let reversed: Vec<u8> = slice_of_bytes.iter().rev().copied().collect();
        Self::unpack_bytes(&reversed, false)
    }

    #[inline]
    #[must_use]
    pub fn from_le_bytes_of_le_bits(slice_of_bytes: &[u8]) -> Self {
        let reversed: Vec<u8> = slice_of_bytes.iter().rev().copied().collect();
        Self::unpack_bytes(&reversed, true)
    }

    #[inline]
    pub fn from_be_bytes_of_le_bits(slice_of_bytes: &mut [u8]) -> Self {
        Self::unpack_bytes(slice_of_bytes, true)
    }
}

impl Deref for Bits {
    type Target = Vec<Bit>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Bits {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Display for Bits {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        let mut index_counter = 0usize;
        while let Some(bit) = self.get(index_counter) {
            output = format!("{output} {bit:?}");
            index_counter += 1;
        }
        write!(formatter, "Bits({output})")
    }
}

impl BitAnd for Bits {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        let mut new_bits = Self { inner: Vec::new() };
        let mut index_counter = 0usize;
        while let Some(bit_from_self) = self.get(index_counter) {
            if let Some(bit_from_rhs) = rhs.get(index_counter) {
                new_bits.push(*bit_from_self & *bit_from_rhs);
                index_counter += 1;
            } else {
                break;
            }
        }
        new_bits
    }
}

impl BitAndAssign for Bits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        let mut index_counter = 0usize;
        let old_self = self.clone();
        while let Some(bit_from_self) = old_self.get(index_counter) {
            if let Some(bit_from_rhs) = rhs.get(index_counter) {
                self.set_bit(index_counter, *bit_from_self & *bit_from_rhs);
                index_counter += 1;
            } else {
                break;
            }
        }
    }
}

impl BitOr for Bits {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        let mut new_bits = Self { inner: Vec::new() };
        let mut index_counter = 0usize;
        while let Some(bit_from_self) = self.get(index_counter) {
            if let Some(bit_from_rhs) = rhs.get(index_counter) {
                new_bits.push(*bit_from_self | *bit_from_rhs);
                index_counter += 1;
            } else {
                break;
            }
        }
        new_bits
    }
}

impl BitOrAssign for Bits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        for (index, bit) in self.inner.iter_mut().enumerate() {
            if let Some(rhs_bit) = rhs.get(index) {
                *bit |= *rhs_bit;
            }
        }
    }
}

impl BitXor for Bits {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        let mut new_bits = Self { inner: Vec::new() };
        let mut index_counter = 0usize;
        while let Some(bit_from_self) = self.get(index_counter) {
            if let Some(bit_from_rhs) = rhs.get(index_counter) {
                new_bits.push(*bit_from_self ^ *bit_from_rhs);
                index_counter += 1;
            } else {
                break;
            }
        }
        new_bits
    }
}

impl BitXorAssign for Bits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        for index in 0..self.len() {
            if let Some(rhs_bit) = rhs.get(index) {
                self[index] ^= *rhs_bit;
            }
        }
    }
}

impl<Idx> Index<Idx> for Bits
where
    Idx: SliceIndex<[Bit]>,
{
    type Output = Idx::Output;

    #[inline]
    fn index(&self, index: Idx) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<Idx> IndexMut<Idx> for Bits
where
    Idx: SliceIndex<[Bit]>,
{
    #[inline]
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl Not for Bits {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        let mut new_bits = Self { inner: Vec::new() };
        for index in 0..self.len() {
            if self.get(index).unwrap().0 {
                new_bits.push(Bit(true));
            } else {
                new_bits.push(Bit(false));
            }
        }
        new_bits
    }
}

impl Shl<u32> for &Bits {
    type Output = Bits;

    fn shl(self, rhs: u32) -> Self::Output {
        self.clone() << rhs
    }
}

impl Shl<usize> for Bits {
    type Output = Self;

    #[inline]
    fn shl(mut self, rhs: usize) -> Self {
        drop(self.drain(..rhs));
        for _ in 0..rhs {
            self.push(Bit(false));
        }
        self
    }
}

impl Shl<u32> for Bits {
    type Output = Self;

    #[inline]
    fn shl(mut self, rhs: u32) -> Self {
        drop(self.drain(..rhs as usize));
        for _ in 0..rhs {
            self.push(Bit(false));
        }
        self
    }
}

impl Shr<usize> for Bits {
    type Output = Self;

    #[inline]
    fn shr(mut self, rhs: usize) -> Self::Output {
        drop(self.inner.drain(..rhs));
        for _ in 0..rhs {
            self.inner.push(Bit(false));
        }
        self
    }
}

impl Shr<u32> for Bits {
    type Output = Self;

    #[inline]
    fn shr(mut self, rhs: u32) -> Self::Output {
        drop(self.inner.drain(..rhs as usize));
        for _ in 0..rhs {
            self.inner.push(Bit(false));
        }
        self
    }
}

impl CheckedShl for Bits {
    #[inline]
    fn checked_shl(&self, rhs: u32) -> Option<Self> {
        if rhs > self.bit_len() as u32 {
            None
        } else {
            Some(self << rhs)
        }
    }
}

impl Zero for Bits {
    #[inline]
    fn zero() -> Self {
        Self::new(&vec![Bit(false); size_of::<Self>() * 8])
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.inner.iter().all(|&x| !x.0)
    }
}

impl One for Bits {
    #[inline]
    fn one() -> Self {
        let mut output = Self::new(&vec![Bit(false); size_of::<Self>() * 8]);
        output.set_bit(size_of::<Self>() * 7, Bit(true));
        output
    }

    #[inline]
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        (self
            .get(0..self.inner.len() - 1)
            .unwrap()
            .iter()
            .all(|&x| !x.0)
            || self.inner.len() == 1)
            && self.get(self.inner.len()).unwrap().0
    }
}

impl Mul for Bits {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let output_as_u128: u128 = u128::from(&self) * u128::from(&rhs);
        Self::Output::from(output_as_u128)
    }
}

impl BitMan for Bits {
    #[inline]
    fn bit_len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn bit(&self, index: usize) -> Bit {
        self[index]
    }

    #[inline]
    fn set_bit(&mut self, index: usize, bit: Bit) {
        self[index] = bit;
    }

    #[inline]
    fn bits(&self) -> Bits {
        self.clone()
    }
}

impl IntoIterator for Bits {
    type Item = Bit;
    type IntoIter = alloc::vec::IntoIter<Bit>;

    /// Yields the bits in index order (index 0 first).
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a Bits {
    type Item = &'a Bit;
    type IntoIter = core::slice::Iter<'a, Bit>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a mut Bits {
    type Item = &'a mut Bit;
    type IntoIter = core::slice::IterMut<'a, Bit>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

#[macro_export]
macro_rules! impl_to_and_from_bits {
    ($($new_type:ty$(,)?)*) => {$(
        impl From<&Bits> for $new_type {
            /// Reads the bits as a number, first bit most significant. Fewer bits
            /// than the type holds are right-aligned; more keep the low-order bits.
            #[inline]
            fn from(bits_to_convert: &Bits) -> $new_type {
                let mut new_value: $new_type = Default::default();
                let type_bits = new_value.bit_len();
                let kept = bits_to_convert.inner.len().min(type_bits);
                let source = &bits_to_convert.inner[bits_to_convert.inner.len() - kept..];
                let offset = type_bits - kept;
                for (index, current_bit) in source.iter().enumerate() {
                    new_value.set_bit(offset + index, *current_bit);
                }
                new_value
            }
        }
        impl From<$new_type> for Bits {
            #[inline]
            fn from(value_to_convert: $new_type) -> Bits {
                let mut output_value: Bits = Default::default();
                for index in 0..value_to_convert.bit_len() {
                    output_value.inner.push(value_to_convert.bit(index));
                }
                output_value
            }
        })*
    }
}

impl_to_and_from_bits!(u8, u16, u32, u64, u128, usize, Bit);

impl Add for Bits {
    type Output = Self;

    /// Binary addition at the wider operand's width; a carry out of the top is dropped.
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let width = self.inner.len().max(rhs.inner.len());
        let mut output_value: Vec<Bit> = vec![Bit(false); width];
        let mut carry = false;
        for position in 0..width {
            let from_end = |bits: &Bits| {
                bits.inner
                    .len()
                    .checked_sub(position + 1)
                    .is_some_and(|index| bits.inner[index].0)
            };
            let (left, right) = (from_end(&self), from_end(&rhs));
            output_value[width - 1 - position] = Bit(left ^ right ^ carry);
            carry = (left & right) | (carry & (left ^ right));
        }
        Self {
            inner: output_value,
        }
    }
}
