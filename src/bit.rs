use core::{
    fmt::{self, Display},
    ops::{
        Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, Div,
        Mul, Not, Shl, ShlAssign, Shr, ShrAssign, Sub,
    },
};

use crate::{BitMan, Bits};
use num_traits::{CheckedShl, One, Zero};

#[cfg(test)]
mod bit_tests;
#[cfg(test)]
pub use bit_tests::*;

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Bit(pub bool);

impl Deref for Bit {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Bit {
    fn fmt(&self, bit: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(bit, "Bit({})", self.0)
    }
}

impl BitAnd for Bit {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bit {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for Bit {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        return Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bit {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for Bit {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        if self.0 {
            return Self(!(rhs.0))
        } else {
            return Self(rhs.0)
        }
    }
}

impl BitXorAssign for Bit {
    fn bitxor_assign(&mut self, rhs: Self) {
        if self.0 {
            self.0 = !(rhs.0);
        } else {
            self.0 = rhs.0;
        }
    }
}

impl Not for Bit {
    type Output = Self;

    fn not(self) -> Self::Output {
        return Self(!self.0)
    }
}

impl Shl<usize> for Bit {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        if rhs == 0 {
            return self;
        }
        return Self(false)
    }
}

impl ShlAssign<usize> for Bit {
    fn shl_assign(&mut self, rhs: usize) {
        if rhs != 0 {
            self.0 = false;
        }
    }
}

impl Shl<u32> for Bit {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        if rhs == 0 {
            return self;
        }
        return Self(false)
    }
}

impl ShlAssign<u32> for Bit {
    fn shl_assign(&mut self, rhs: u32) {
        if rhs != 0 {
            self.0 = false;
        }
    }
}

impl CheckedShl for Bit {
    fn checked_shl(&self, rhs: u32) -> Option<Self> {
        if rhs == 0 {
            Some(*self)
        } else {
            Some(Self::zero())
        }
    }
}

impl Shr<usize> for Bit {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        if rhs == 0 {
            return self;
        }
        return Self(false)
    }
}

impl ShrAssign<usize> for Bit {
    fn shr_assign(&mut self, rhs: usize) {
        if rhs != 0 {
            self.0 = false;
        }
    }
}

impl Mul for Bit {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        if *self {
            rhs
        } else {
            return Self(false)
        }
    }
}

impl Div for Bit {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        if *rhs {
            self
        } else {
            panic!("Divide by Zero")
        }
    }
}

impl Add for Bit {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if *self {
            if *rhs {
                return Self(false);
            } else {
                return Self(true);
            }
        }
        if *rhs {
            return Self(true)
        } else {
            return Self(false)
        }
    }
}

impl Sub for Bit {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if *self {
            if *rhs {
                return Self(false)
            } else {
                self
            }
        } else {
            return Self(false)
        }
    }
}

impl One for Bit {
    fn one() -> Self {
        return Self(true)
    }
}

impl Zero for Bit {
    fn zero() -> Self {
        return Self(false)
    }

    fn is_zero(&self) -> bool {
        return *self == Self(false)
    }
}

impl BitMan for Bit {
    fn bit_len(&self) -> usize {
        1
    }

    #[inline]
    fn bit(&self, _index: &u32) -> Bit {
        *self
    }

    #[inline]
    default fn set_bit(&mut self, _index: &u32, bit: &Bit) {
        self.0 = bit.0;
    }

    #[inline]
    default fn bits(&self) -> Bits {
        Bits::new(&[*self])
    }

    #[inline]
    default fn set_bits(&mut self, index: u32, bits: &Bits) {
        assert_eq!(index, 0);
        self.0 = bits.get(&bits.len() - 1).unwrap().0;
    }
}
