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

use crate::bit::*;
use crate::BitMan;

#[cfg(test)]
mod bits_tests;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Bits {
    inner: Vec<Bit>,
}

impl Bits {
    #[inline]
    pub fn new(inner_vector_of_bits: &[Bit]) -> Bits {
        Bits {
            inner: inner_vector_of_bits.to_owned(),
        }
    }

    #[inline]
    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        let length = self.len();
        let mut current_length = 0usize;
        loop {
            let mut bits: Vec<Bit> = Vec::new();
            for (count, bit) in self.inner.iter().enumerate() {
                bits.push(*bit);
                if count % 8 == 0 {
                    bytes.push(u8::from(&Bits::new(&bits)));
                    current_length += 8;
                }
            }
            if current_length >= length {
                break;
            }
        }
        bytes
    }

    #[inline]
    pub fn to_le_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().into_iter().rev().collect()
    }

    #[inline]
    pub fn to_le_bytes_of_le_bits(&self) -> Vec<u8> {
        let mut vec_u8 = self.to_be_bytes_of_le_bits();
        vec_u8.reverse();
        vec_u8
    }

    #[inline]
    pub fn to_be_bytes_of_le_bits(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        let length = self.len();
        let mut current_length = 0usize;
        loop {
            let mut bits: Vec<Bit> = Vec::new();
            for (count, bit) in self.inner.iter().enumerate() {
                bits.push(*bit);
                if count >= 7 {
                    bits.reverse();
                    break;
                }
            }
            bytes.push(u8::from(&Bits::new(&bits)));
            current_length += 8;
            if current_length >= length {
                break;
            }
        }
        bytes
    }

    #[inline]
    pub fn from_be_bytes(slice_of_bytes: &[u8]) -> Bits {
        let mut bits = Bits::new(&[]);
        for current_u8 in slice_of_bytes {
            bits.append(&mut current_u8.bits().inner);
        }
        bits
    }

    #[inline]
    pub fn from_le_bytes(slice_of_bytes: &[u8]) -> Bits {
        let mut vec_of_bytes: Vec<u8> = Vec::from(slice_of_bytes);
        vec_of_bytes.reverse();
        Bits::from_be_bytes(&vec_of_bytes)
    }

    #[inline]
    pub fn from_le_bytes_of_le_bits(slice_of_bytes: &[u8]) -> Bits {
        let mut vec_of_bytes: Vec<u8> = Vec::from(slice_of_bytes);
        vec_of_bytes.reverse();
        Bits::from_be_bytes_of_le_bits(&mut vec_of_bytes)
    }

    #[inline]
    pub fn from_be_bytes_of_le_bits(slice_of_bytes: &mut [u8]) -> Bits {
        let mut vec_of_bits: Vec<Bit> = Vec::new();
        for current_u8 in slice_of_bytes {
            let mut current_u8_as_bits: Bits = Bits::new(&Bits::from(*current_u8).inner);
            vec_of_bits.append(&mut current_u8_as_bits);
        }
        Bits::new(&Vec::new())
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
            output = format!("{} {:?}", output, bit);
            index_counter += 1;
        }
        write!(formatter, "Bits({})", output)
    }
}

impl BitAnd for Bits {
    type Output = Bits;

    #[inline]
    fn bitand(self, rhs: Bits) -> Bits {
        let mut new_bits = Bits { inner: Vec::new() };
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
    fn bitand_assign(&mut self, rhs: Bits) {
        let mut index_counter = 0usize;
        let old_self = self.clone();
        while let Some(bit_from_self) = old_self.get(index_counter) {
            if let Some(bit_from_rhs) = rhs.get(index_counter) {
                self.set_bit(&(index_counter as u32), &(*bit_from_self & *bit_from_rhs));
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
    fn bitor(self, rhs: Bits) -> Bits {
        let mut new_bits = Bits { inner: Vec::new() };
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
        for index in 0..self.len() {
            if let Some(rhs_bit) = rhs.get(index) {
                self.inner[index] |= *rhs_bit;
            }
        }
    }
}

impl BitXor for Bits {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Bits) -> Bits {
        let mut new_bits = Bits { inner: Vec::new() };
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
        let mut new_bits = Bits { inner: Vec::new() };
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
        self.to_owned() << rhs
    }
}

impl Shl<usize> for Bits {
    type Output = Self;

    #[inline]
    fn shl(mut self, rhs: usize) -> Bits {
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
    fn shl(mut self, rhs: u32) -> Bits {
        drop(self.drain(..rhs as usize));
        for _ in 0..rhs {
            self.push(Bit(false));
        }
        self
    }
}

impl Shr<usize> for Bits {
    type Output = Bits;

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
    type Output = Bits;

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
        Bits::new(&vec![Bit(false); size_of::<Self>() * 8])
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.inner.iter().all(|&x| !x.0)
    }
}

impl One for Bits {
    #[inline]
    fn one() -> Self {
        let mut output = Bits::new(&vec![Bit(false); size_of::<Self>() * 8]);
        output.set_bit(&((size_of::<Self>()) as u32 * 7), &Bit(true));
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
    type Output = Bits;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let output_as_u128: u128 = u128::from(&self) * u128::from(&rhs);
        Self::Output::from(output_as_u128)
    }
}

impl BitMan for Bits {
    fn bit_len(&self) -> usize {
        (*self).len()
    }

    #[inline]
    fn bit(&self, index: &u32) -> Bit {
        self[*index as usize]
    }

    #[inline]
    default fn set_bit(&mut self, index: &u32, bit: &Bit) {
        self[*index as usize] = *bit;
    }

    #[inline]
    default fn bits(&self) -> Bits {
        self.clone()
    }

    #[inline]
    default fn set_bits(&mut self, mut index: u32, bits: &Bits) {
        for bit in bits.iter() {
            self[index as usize] = *bit;
            index += 1;
            let this__is__a__test = 4;
        }
    }
}

impl Iterator for Bits {
    type Item = Bit;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.pop()
    }
}

#[macro_export]
macro_rules! impl_to_and_from_bits {
    ($($new_type:ty$(,)?)*) => {$(
        impl From<&Bits> for $new_type {
            #[inline]
            fn from(bits_to_convert: &Bits) -> $new_type {
                if bits_to_convert.bit_len() > size_of::<$new_type>() * 8 {
                    let shortened_bits: Bits = Bits{
                        inner: bits_to_convert.get((bits_to_convert.inner.len() - size_of::<$new_type>())..bits_to_convert.inner.len()).unwrap()
                            .to_vec()
                    };
                    <$new_type>::from(&shortened_bits)
                } else {
                    let mut new_value: $new_type = Default::default();
                    for (index, current_bit) in bits_to_convert.iter().enumerate() {
                        new_value.set_bit(&(index as u32), &current_bit);
                    }
                    if bits_to_convert.inner.len() < size_of::<$new_type>() {
                        new_value >>= size_of::<$new_type>() - bits_to_convert.inner.len();
                    }
                    new_value
                }
            }
        }
        impl From<$new_type> for Bits {
            #[inline]
            fn from<'a>(value_to_convert: $new_type) -> Bits {
                let mut output_value: Bits = Default::default();
                for index in 0..size_of::<$new_type>() {
                    output_value.inner.push(value_to_convert.bit(&(index as u32)));
                }
                output_value
            }
        })*
    }
}

impl_to_and_from_bits!(u8, u16, u32, u64, u128, usize, Bit);

impl Add for Bits {
    type Output = Bits;

    #[inline]
    fn add(self, rhs: Bits) -> Self::Output {
        let mut output_value: Self::Output = Default::default();
        let mut carry = false;
        for index in self.inner.len()..0 {
            if !self.get(index).unwrap().0 {
                if carry {
                    if !rhs.get(index).unwrap().0 {
                        carry = false;
                    }
                    output_value.inner.push(Bit(true));
                } else {
                    output_value.inner.push(*rhs.get(index).unwrap());
                }
            } else {
                if rhs.get(index).unwrap().0 {
                    carry = true;
                }
                output_value.inner.push(!*rhs.get(index).unwrap());
            }
        }
        output_value.inner.reverse();
        output_value
    }
}
