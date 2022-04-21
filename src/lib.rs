//! Rips your primitive integers to Bits!
//!
//! The *Bits* structure is the heart of *bitman*.  It contains both the
//! primitive integer and a vector of booleans representing each bit within it
//! and both are kept in constant synchronization.  As a result, reading the
//! primitive integer or any of the bits within the vector of booleans is very
//! fast.  In an upcoming update we will complete multithreading of the whole
//! library making both reads and writes even faster still.
//!
//! We have striven to make *bitman* easy to use.  As such, you will find that
//! methods like u8::to_bits() mirror the functionality of Bits::from_integer()
//! so that you can feel free to use which ever you feel more comfortable with.
//!
//! Similarly, although the usual way to use *bitman* is to wrap a primitive
//! integer or Vec\<bool> into a Bits struct and use the Bits::set_bit() and
//! Bits::bit() methods to set and read individual bits or dereference the Bits
//! instance to access the primitive integer, we have also added set_bit() and
//! bit() methods to all of Rust's primitive integers via the AnyIntegerType
//! trait.  Please note that using those methods directly on the primitive
//! integers will not offer any of the speed advantages of the Bits struct but
//! may save a moment or two that would have been spent constructing a Bits
//! instance.
//!
//! For help getting started with *bitman*, see the *Usage* section
//! of our [*README.md*](https://github.com/ciresnave/bitman/blob/main/README.md).
//!
//! Note: All methods are big endian.  Bit zero is always the most significant.
//!
//! Additional Note: While signed primitive integers can be used, attempting to
//! change the sign bit is not possible due to the way Rust handles bit shifting
//! of signed integers.  All other bits within signed integers work as expected.
#![no_std]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::{
    fmt::{Binary, Debug, Display},
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, Index,
        IndexMut, Mul, Not, Rem, Shl, Sub,
    },
};
use core::{mem::size_of, slice::SliceIndex};
use num_traits::{One, Zero};

#[cfg(test)]
mod unit_tests {
    use alloc::vec;
    use alloc::vec::Vec;
    use core::fmt::Binary;
    use core::{
        fmt::{Debug, Display},
        ops::Not,
    };

    use crate::{AnyIntegerType, Bits, WrappedInteger};
    use num_traits::{One, Zero};

    fn type_of<T>(_: &T) -> &str {
        return core::any::type_name::<T>();
    }

    fn test_bits_from_integer_type<T>()
    where
        T: Zero + One + Debug + PartialEq + AnyIntegerType<T, IntegerType = T>,
    {
        let my_integer: T = T::one();
        let my_bits = my_integer.to_bits();
        let my_bits2: Bits<T> = Bits::new("unnamed Bits", T::one());
        assert!(
            type_of(&my_bits2) == type_of(&my_bits),
            "type_of my_bits2 is {} while type_of my_bits is {}",
            type_of(&my_bits2),
            type_of(&my_bits)
        );
        assert_eq!(
            my_bits.inner_integer, my_bits2.inner_integer,
            "my_bits.inner_integer is {:#?} while my_bits2.inner_integer is {:#?}",
            my_bits.inner_integer, my_bits2.inner_integer
        );
        assert_eq!(
            my_bits.inner_integer,
            T::one(),
            "my_bits.as_integer() is {:#?} while T::one() is {:#?}",
            my_bits.inner_integer,
            T::one()
        );
        assert_eq!(
            my_bits.as_vec_bool().len(),
            T::max_settable_bits(),
            "my_bits.as_vec_bool().len() is {} while T::size_in_bits() is {}",
            my_bits.as_vec_bool().len(),
            T::max_settable_bits()
        );
        assert!(my_bits.as_vec_bool()[0..(T::max_bit_index() - 1)]
            .iter()
            .all(|&x| x == false));
        assert_eq!(
            my_bits.as_vec_bool()[T::max_bit_index()],
            true,
            "my_bits.as_vec_bool() index {} is {} and should be true.",
            T::max_bit_index(),
            my_bits.as_vec_bool()[T::max_bit_index()]
        );
    }

    #[test]
    fn test_bits_from_u8() {
        test_bits_from_integer_type::<u8>();
    }

    #[test]
    fn test_bits_from_u16() {
        test_bits_from_integer_type::<u16>();
    }

    #[test]
    fn test_bits_from_u32() {
        test_bits_from_integer_type::<u32>();
    }

    #[test]
    fn test_bits_from_u64() {
        test_bits_from_integer_type::<u64>();
    }

    #[test]
    fn test_bits_from_u128() {
        test_bits_from_integer_type::<u128>();
    }

    #[test]
    fn test_bits_from_i8() {
        test_bits_from_integer_type::<i8>();
    }

    #[test]
    fn test_bits_from_i16() {
        test_bits_from_integer_type::<i16>();
    }

    #[test]
    fn test_bits_from_i32() {
        test_bits_from_integer_type::<i32>();
    }

    #[test]
    fn test_bits_from_i64() {
        test_bits_from_integer_type::<i64>();
    }

    #[test]
    fn test_bits_from_i128() {
        test_bits_from_integer_type::<i128>();
    }

    #[test]
    fn test_bits_from_usize() {
        test_bits_from_integer_type::<usize>();
    }

    #[test]
    fn test_bits_from_isize() {
        test_bits_from_integer_type::<isize>();
    }

    fn test_bits_from_default<T>()
    where
        T: Zero + One + Default + AnyIntegerType<T, IntegerType = T> + PartialEq + Debug,
    {
        let my_t: T = Default::default();
        let my_bits: Bits<T> = Bits::default();
        assert_eq!(
            my_bits.inner_integer, my_t,
            "my_bits.as_integer is {:#?} and my_t is {:#?}",
            my_bits.inner_integer, my_t
        );
        assert_eq!(
            my_bits.as_vec_bool().len(),
            T::max_settable_bits(),
            "Length of Vec<bool> does not match T::size_in_bits()."
        );
        let my_wrapped_t = WrappedInteger::<T>(my_t);
        let new_vec2 = Vec::<bool>::from(my_wrapped_t);
        assert_eq!(*my_bits.as_vec_bool(), new_vec2);
        for index in 0..T::max_bit_index() {
            assert_eq!(my_bits.bit(index), my_t.bit(index));
        }
    }

    #[test]
    fn test_default_u8() {
        test_bits_from_default::<u8>();
    }

    #[test]
    fn test_default_u16() {
        test_bits_from_default::<u16>();
    }

    #[test]
    fn test_default_u32() {
        test_bits_from_default::<u32>();
    }

    #[test]
    fn test_default_u64() {
        test_bits_from_default::<u64>();
    }

    #[test]
    fn test_default_u128() {
        test_bits_from_default::<u128>();
    }

    #[test]
    fn test_default_usize() {
        test_bits_from_default::<usize>();
    }

    #[test]
    fn test_default_i8() {
        test_bits_from_default::<i8>();
    }

    #[test]
    fn test_default_i16() {
        test_bits_from_default::<i16>();
    }

    #[test]
    fn test_default_i32() {
        test_bits_from_default::<i32>();
    }

    #[test]
    fn test_default_i64() {
        test_bits_from_default::<i64>();
    }

    #[test]
    fn test_default_i128() {
        test_bits_from_default::<i128>();
    }

    #[test]
    fn test_default_isize() {
        test_bits_from_default::<isize>();
    }

    fn test_bits_from_integer<T>()
    where
        T: Zero + One + Default + AnyIntegerType<T, IntegerType = T> + PartialEq + Debug,
    {
        let my_bits: Bits<T> = Bits::from_integer(T::one());
        assert_eq!(my_bits.inner_integer, T::one());
        assert_eq!(my_bits.as_vec_bool()[T::max_bit_index()], true);
        assert_eq!(my_bits.as_vec_bool()[T::max_bit_index() - 1], false);
    }

    #[test]
    fn test_from_integer_u8() {
        test_bits_from_integer::<u8>();
    }

    #[test]
    fn test_from_integer_u16() {
        test_bits_from_integer::<u16>();
    }

    #[test]
    fn test_from_integer_u32() {
        test_bits_from_integer::<u32>();
    }

    #[test]
    fn test_from_integer_u64() {
        test_bits_from_integer::<u64>();
    }

    #[test]
    fn test_from_integer_u128() {
        test_bits_from_integer::<u128>();
    }

    #[test]
    fn test_from_integer_usize() {
        test_bits_from_integer::<usize>();
    }

    #[test]
    fn test_from_integer_i8() {
        test_bits_from_integer::<i8>();
    }

    #[test]
    fn test_from_integer_i16() {
        test_bits_from_integer::<i16>();
    }

    #[test]
    fn test_from_integer_i32() {
        test_bits_from_integer::<i32>();
    }

    #[test]
    fn test_from_integer_i64() {
        test_bits_from_integer::<i64>();
    }

    #[test]
    fn test_from_integer_i128() {
        test_bits_from_integer::<i128>();
    }

    #[test]
    fn test_from_integer_isize() {
        test_bits_from_integer::<isize>();
    }

    fn test_bits_from_named_integer<T>()
    where
        T: Zero + One + Default + AnyIntegerType<T, IntegerType = T> + PartialEq + Debug,
    {
        let my_bits: Bits<T> = Bits::from_named_integer("unnamed Bits", T::one());
        assert_eq!(my_bits.inner_integer, T::one());
        assert_eq!(my_bits.as_vec_bool()[T::max_bit_index()], true);
        assert_eq!(my_bits.as_vec_bool()[T::max_bit_index() - 1], false);
        assert_eq!(my_bits.name, "unnamed Bits");
    }

    #[test]
    fn test_from_named_integer_u8() {
        test_bits_from_named_integer::<u8>();
    }

    #[test]
    fn test_from_named_integer_u16() {
        test_bits_from_named_integer::<u16>();
    }

    #[test]
    fn test_from_named_integer_u32() {
        test_bits_from_named_integer::<u32>();
    }

    #[test]
    fn test_from_named_integer_u64() {
        test_bits_from_named_integer::<u64>();
    }

    #[test]
    fn test_from_named_integer_u128() {
        test_bits_from_named_integer::<u128>();
    }

    #[test]
    fn test_from_named_integer_usize() {
        test_bits_from_named_integer::<usize>();
    }

    #[test]
    fn test_from_named_integer_i8() {
        test_bits_from_named_integer::<i8>();
    }

    #[test]
    fn test_from_named_integer_i16() {
        test_bits_from_named_integer::<i16>();
    }

    #[test]
    fn test_from_named_integer_i32() {
        test_bits_from_named_integer::<i32>();
    }

    #[test]
    fn test_from_named_integer_i64() {
        test_bits_from_named_integer::<i64>();
    }

    #[test]
    fn test_from_named_integer_i128() {
        test_bits_from_named_integer::<i128>();
    }

    #[test]
    fn test_from_named_integer_isize() {
        test_bits_from_named_integer::<isize>();
    }

    fn test_from_named_vec_bool<T>()
    where
        T: Zero
            + One
            + Not<Output = T>
            + Debug
            + AnyIntegerType<T, IntegerType = T>
            + PartialOrd
            + Display,
    {
        let mut my_vec = vec![false; T::max_settable_bits() - 1];
        my_vec.push(true);
        let my_bits: Bits<T> = Bits::from_named_vec_bool("unnamed Bits", my_vec.clone());
        assert_eq!(
            my_bits.inner_integer,
            T::one(),
            "my_vec is {:#?} and *my_bits.as_integer() is {:#?}",
            my_vec,
            my_bits.inner_integer
        );
    }

    #[test]
    fn test_from_named_vec_bool_u8() {
        test_from_named_vec_bool::<u8>();
    }

    #[test]
    fn test_from_named_vec_bool_u16() {
        test_from_named_vec_bool::<u16>();
    }

    #[test]
    fn test_from_named_vec_bool_u32() {
        test_from_named_vec_bool::<u32>();
    }

    #[test]
    fn test_from_named_vec_bool_u64() {
        test_from_named_vec_bool::<u64>();
    }

    #[test]
    fn test_from_named_vec_bool_u128() {
        test_from_named_vec_bool::<u128>();
    }

    #[test]
    fn test_from_named_vec_bool_usize() {
        test_from_named_vec_bool::<usize>();
    }

    #[test]
    fn test_from_named_vec_bool_i8() {
        test_from_named_vec_bool::<i8>();
    }

    #[test]
    fn test_from_named_vec_bool_i16() {
        test_from_named_vec_bool::<i16>();
    }

    #[test]
    fn test_from_named_vec_bool_i32() {
        test_from_named_vec_bool::<i32>();
    }

    #[test]
    fn test_from_named_vec_bool_i64() {
        test_from_named_vec_bool::<i64>();
    }

    #[test]
    fn test_from_named_vec_bool_i128() {
        test_from_named_vec_bool::<i128>();
    }

    #[test]
    fn test_from_named_vec_bool_isize() {
        test_from_named_vec_bool::<isize>();
    }

    fn test_from_vec_bool<T>()
    where
        T: Zero
            + One
            + Not<Output = T>
            + Debug
            + AnyIntegerType<T, IntegerType = T>
            + PartialOrd
            + Display,
    {
        let mut my_vec = vec![false; T::max_bit_index()];
        my_vec.push(true);
        let my_bits: Bits<T> = Bits::from_vec_bool(my_vec.clone());
        assert_eq!(
            my_bits.inner_integer,
            T::one(),
            "my_vec is {:#?} and *my_bits.as_integer() is {:#?}",
            my_vec,
            my_bits.inner_integer
        );
    }

    #[test]
    fn test_from_vec_bool_u8() {
        test_from_vec_bool::<u8>();
    }

    #[test]
    fn test_from_vec_bool_u16() {
        test_from_vec_bool::<u16>();
    }

    #[test]
    fn test_from_vec_bool_u32() {
        test_from_vec_bool::<u32>();
    }

    #[test]
    fn test_from_vec_bool_u64() {
        test_from_vec_bool::<u64>();
    }

    #[test]
    fn test_from_vec_bool_u128() {
        test_from_vec_bool::<u128>();
    }

    #[test]
    fn test_from_vec_bool_usize() {
        test_from_vec_bool::<usize>();
    }

    #[test]
    fn test_from_vec_bool_i8() {
        test_from_vec_bool::<i8>();
    }

    #[test]
    fn test_from_vec_bool_i16() {
        test_from_vec_bool::<i16>();
    }

    #[test]
    fn test_from_vec_bool_i32() {
        test_from_vec_bool::<i32>();
    }

    #[test]
    fn test_from_vec_bool_i64() {
        test_from_vec_bool::<i64>();
    }

    #[test]
    fn test_from_vec_bool_i128() {
        test_from_vec_bool::<i128>();
    }

    #[test]
    fn test_from_vec_bool_isize() {
        test_from_vec_bool::<isize>();
    }

    fn test_set_and_get_bit_in_bits<T>()
    where
        T: Zero
            + One
            + Not<Output = T>
            + Debug
            + AnyIntegerType<T, IntegerType = T>
            + PartialOrd
            + Display
            + Binary,
    {
        let mut my_bits = Bits::new("TestBits", T::zero());
        for index in 0..(T::max_bit_index()) {
            let mut test_bit = my_bits.bit(index);
            assert_eq!(
                test_bit, false,
                "Bit {:#?} of my_bits was not initially zero'd to false.",
                index
            );
            my_bits.set_bit(index, true);
            test_bit = my_bits.bit(index);
            assert_eq!(test_bit, true, "Bit {:#?} of my_bits was {:#?} when retrieved after being set to true.  my_bits: {:#?}", index, test_bit, my_bits.inner_vec_bool);
        }
    }

    #[test]
    fn test_set_and_get_bit_in_bits_u8() {
        test_set_and_get_bit_in_bits::<u8>();
    }

    #[test]
    fn test_set_and_get_bit_in_bits_u16() {
        test_set_and_get_bit_in_bits::<u16>();
    }

    #[test]
    fn test_set_and_get_bit_in_bits_u32() {
        test_set_and_get_bit_in_bits::<u32>();
    }

    #[test]
    fn test_set_and_get_bit_in_bits_u64() {
        test_set_and_get_bit_in_bits::<u64>();
    }

    #[test]
    fn test_set_and_get_bit_in_bits_u128() {
        test_set_and_get_bit_in_bits::<u128>();
    }

    #[test]
    fn test_set_and_get_bit_in_bits_usize() {
        test_set_and_get_bit_in_bits::<usize>();
    }

    #[test]
    fn test_set_and_get_bit_in_bits_i8() {
        test_set_and_get_bit_in_bits::<i8>();
    }

    #[test]
    fn test_set_and_get_bit_in_bits_i16() {
        test_set_and_get_bit_in_bits::<i16>();
    }

    #[test]
    fn test_set_and_get_bit_in_bits_i32() {
        test_set_and_get_bit_in_bits::<i32>();
    }

    #[test]
    fn test_set_and_get_bit_in_bits_i64() {
        test_set_and_get_bit_in_bits::<i64>();
    }

    #[test]
    fn test_set_and_get_bit_in_bits_i128() {
        test_set_and_get_bit_in_bits::<i128>();
    }

    #[test]
    fn test_set_and_get_bit_in_bits_isize() {
        test_set_and_get_bit_in_bits::<isize>();
    }

    fn test_set_and_get_bit_in_t<T>()
    where
        T: Zero
            + One
            + Not<Output = T>
            + Debug
            + AnyIntegerType<T, IntegerType = T>
            + PartialOrd
            + Display
            + Binary,
    {
        let mut my_t = T::zero();
        for index in 0..(T::max_bit_index()) {
            assert_eq!(
                my_t.bit(index),
                false,
                "Bit {:#?} of my_t was not initially zero'd to false.",
                index
            );
            my_t = my_t.set_bit(index, true);
            assert_eq!(
                my_t.bit(index),
                true,
                "Bit {:#?} of my_t was {:#?} when retrieved after being set to true.  my_t: {:b}",
                index,
                my_t.bit(index),
                my_t
            );
        }
    }

    #[test]
    fn test_set_and_get_bit_in_t_u8() {
        test_set_and_get_bit_in_t::<u8>();
    }

    #[test]
    fn test_set_and_get_bit_in_t_u16() {
        test_set_and_get_bit_in_t::<u16>();
    }

    #[test]
    fn test_set_and_get_bit_in_t_u32() {
        test_set_and_get_bit_in_t::<u32>();
    }

    #[test]
    fn test_set_and_get_bit_in_t_u64() {
        test_set_and_get_bit_in_t::<u64>();
    }

    #[test]
    fn test_set_and_get_bit_in_t_u128() {
        test_set_and_get_bit_in_t::<u128>();
    }

    #[test]
    fn test_set_and_get_bit_in_t_usize() {
        test_set_and_get_bit_in_t::<usize>();
    }

    #[test]
    fn test_set_and_get_bit_in_t_i8() {
        test_set_and_get_bit_in_t::<i8>();
    }

    #[test]
    fn test_set_and_get_bit_in_t_i16() {
        test_set_and_get_bit_in_t::<i16>();
    }

    #[test]
    fn test_set_and_get_bit_in_t_i32() {
        test_set_and_get_bit_in_t::<i32>();
    }

    #[test]
    fn test_set_and_get_bit_in_t_i64() {
        test_set_and_get_bit_in_t::<i64>();
    }

    #[test]
    fn test_set_and_get_bit_in_t_i128() {
        test_set_and_get_bit_in_t::<i128>();
    }

    #[test]
    fn test_set_and_get_bit_in_t_isize() {
        test_set_and_get_bit_in_t::<isize>();
    }

    fn test_index_bit_in_bits<T>()
    where
        T: Debug + One + AnyIntegerType<T, IntegerType = T> + PartialOrd + Display + Binary,
    {
        let my_bits = Bits::<T>::new("Test Bits", T::one());
        assert_eq!(my_bits[0], false);
        assert_eq!(my_bits[T::max_bit_index() - 1], false);
        assert_eq!(my_bits[T::max_bit_index()], true);
    }

    #[test]
    fn test_index_bit_in_bits_u8() {
        test_index_bit_in_bits::<u8>();
    }

    #[test]
    fn test_index_bit_in_bits_u16() {
        test_index_bit_in_bits::<u16>();
    }

    #[test]
    fn test_index_bit_in_bits_u32() {
        test_index_bit_in_bits::<u32>();
    }

    #[test]
    fn test_index_bit_in_bits_u64() {
        test_index_bit_in_bits::<u64>();
    }

    #[test]
    fn test_index_bit_in_bits_u128() {
        test_index_bit_in_bits::<u128>();
    }

    #[test]
    fn test_index_bit_in_bits_usize() {
        test_index_bit_in_bits::<usize>();
    }

    #[test]
    fn test_index_bit_in_bits_i8() {
        test_index_bit_in_bits::<i8>();
    }

    #[test]
    fn test_index_bit_in_bits_i16() {
        test_index_bit_in_bits::<i16>();
    }

    #[test]
    fn test_index_bit_in_bits_i32() {
        test_index_bit_in_bits::<i32>();
    }

    #[test]
    fn test_index_bit_in_bits_i64() {
        test_index_bit_in_bits::<i64>();
    }

    #[test]
    fn test_index_bit_in_bits_i128() {
        test_index_bit_in_bits::<i128>();
    }

    #[test]
    fn test_index_bit_in_bits_isize() {
        test_index_bit_in_bits::<isize>();
    }

    fn test_slice_of_bits_in_bits<T>()
    where
        T: Debug + AnyIntegerType<T, IntegerType = T> + PartialEq + Display,
    {
        let mut my_bits = Bits::<T>::new("Test Bits", T::one());
        assert_eq!(my_bits.inner_vec_bool, my_bits[0..T::max_settable_bits()]);
        assert_eq!(my_bits.inner_vec_bool[1..3], my_bits[1..3]);
        my_bits[0..2].copy_from_slice(&[true, true]);
        assert_eq!(my_bits[0], true);
        assert_eq!(my_bits[1], true);
    }

    #[test]
    fn test_slice_of_bits_in_bits_u8() {
        test_slice_of_bits_in_bits::<u8>();
    }

    #[test]
    fn test_slice_of_bits_in_bits_u16() {
        test_slice_of_bits_in_bits::<u16>();
    }

    #[test]
    fn test_slice_of_bits_in_bits_u32() {
        test_slice_of_bits_in_bits::<u32>();
    }

    #[test]
    fn test_slice_of_bits_in_bits_u64() {
        test_slice_of_bits_in_bits::<u64>();
    }

    #[test]
    fn test_slice_of_bits_in_bits_u128() {
        test_slice_of_bits_in_bits::<u128>();
    }

    #[test]
    fn test_slice_of_bits_in_bits_usize() {
        test_slice_of_bits_in_bits::<usize>();
    }

    #[test]
    fn test_slice_of_bits_in_bits_i8() {
        test_slice_of_bits_in_bits::<i8>();
    }

    #[test]
    fn test_slice_of_bits_in_bits_i16() {
        test_slice_of_bits_in_bits::<i16>();
    }

    #[test]
    fn test_slice_of_bits_in_bits_i32() {
        test_slice_of_bits_in_bits::<i32>();
    }

    #[test]
    fn test_slice_of_bits_in_bits_i64() {
        test_slice_of_bits_in_bits::<i64>();
    }

    #[test]
    fn test_slice_of_bits_in_bits_i128() {
        test_slice_of_bits_in_bits::<i128>();
    }

    #[test]
    fn test_slice_of_bits_in_bits_isize() {
        test_slice_of_bits_in_bits::<isize>();
    }

    fn test_deref_bits<T>()
    where
        T: AnyIntegerType<T, IntegerType = T>,
    {
        let my_t = T::default();
        let my_bits = Bits::new("TestBits", my_t);
        assert_eq!(my_t, *my_bits);
    }

    #[test]
    fn test_deref_bits_u8() {
        test_deref_bits::<u8>();
    }

    #[test]
    fn test_deref_bits_u16() {
        test_deref_bits::<u16>();
    }

    #[test]
    fn test_deref_bits_u32() {
        test_deref_bits::<u32>();
    }

    #[test]
    fn test_deref_bits_u64() {
        test_deref_bits::<u64>();
    }

    #[test]
    fn test_deref_bits_u128() {
        test_deref_bits::<u128>();
    }

    #[test]
    fn test_deref_bits_usize() {
        test_deref_bits::<usize>();
    }

    #[test]
    fn test_deref_bits_i8() {
        test_deref_bits::<i8>();
    }

    #[test]
    fn test_deref_bits_i16() {
        test_deref_bits::<i16>();
    }

    #[test]
    fn test_deref_bits_i32() {
        test_deref_bits::<i32>();
    }

    #[test]
    fn test_deref_bits_i64() {
        test_deref_bits::<i64>();
    }

    #[test]
    fn test_deref_bits_i128() {
        test_deref_bits::<i128>();
    }

    #[test]
    fn test_deref_bits_isize() {
        test_deref_bits::<isize>();
    }

    fn test_deref_mut_bits<T>()
    where
        T: AnyIntegerType<T, IntegerType = T>,
    {
        let mut my_bits = Bits::new("TestBits", T::zero());
        *my_bits = T::one();
        assert_eq!(T::one(), *my_bits);
    }

    #[test]
    fn test_deref_mut_bits_u8() {
        test_deref_mut_bits::<u8>();
    }

    #[test]
    fn test_deref_mut_bits_u16() {
        test_deref_mut_bits::<u16>();
    }

    #[test]
    fn test_deref_mut_bits_u32() {
        test_deref_mut_bits::<u32>();
    }

    #[test]
    fn test_deref_mut_bits_u64() {
        test_deref_mut_bits::<u64>();
    }

    #[test]
    fn test_deref_mut_bits_u128() {
        test_deref_mut_bits::<u128>();
    }

    #[test]
    fn test_deref_mut_bits_usize() {
        test_deref_mut_bits::<usize>();
    }

    #[test]
    fn test_deref_mut_bits_i8() {
        test_deref_mut_bits::<i8>();
    }

    #[test]
    fn test_deref_mut_bits_i16() {
        test_deref_mut_bits::<i16>();
    }

    #[test]
    fn test_deref_mut_bits_i32() {
        test_deref_mut_bits::<i32>();
    }

    #[test]
    fn test_deref_mut_bits_i64() {
        test_deref_mut_bits::<i64>();
    }

    #[test]
    fn test_deref_mut_bits_i128() {
        test_deref_mut_bits::<i128>();
    }

    #[test]
    fn test_deref_mut_bits_isize() {
        test_deref_mut_bits::<isize>();
    }

    fn test_integer_to_vec_bool_to_integer<T>()
    where
        T: AnyIntegerType<T, IntegerType = T> + One,
    {
        let my_t = T::one();
        let wrapped_t = WrappedInteger::<T>(my_t);
        let my_vec_bool = Vec::<bool>::from(wrapped_t);
        let rewrapped_t = WrappedInteger::<T>::from(my_vec_bool);
        assert_eq!(my_t, *rewrapped_t);
    }

    #[test]
    fn test_integer_to_vec_bool_to_integer_u8() {
        test_integer_to_vec_bool_to_integer::<u8>();
    }

    #[test]
    fn test_integer_to_vec_bool_to_integer_u16() {
        test_integer_to_vec_bool_to_integer::<u16>();
    }

    #[test]
    fn test_integer_to_vec_bool_to_integer_u32() {
        test_integer_to_vec_bool_to_integer::<u32>();
    }

    #[test]
    fn test_integer_to_vec_bool_to_integer_u64() {
        test_integer_to_vec_bool_to_integer::<u64>();
    }

    #[test]
    fn test_integer_to_vec_bool_to_integer_u128() {
        test_integer_to_vec_bool_to_integer::<u128>();
    }

    #[test]
    fn test_integer_to_vec_bool_to_integer_usize() {
        test_integer_to_vec_bool_to_integer::<usize>();
    }

    #[test]
    fn test_integer_to_vec_bool_to_integer_i8() {
        test_integer_to_vec_bool_to_integer::<i8>();
    }

    #[test]
    fn test_integer_to_vec_bool_to_integer_i16() {
        test_integer_to_vec_bool_to_integer::<i16>();
    }

    #[test]
    fn test_integer_to_vec_bool_to_integer_i32() {
        test_integer_to_vec_bool_to_integer::<i32>();
    }

    #[test]
    fn test_integer_to_vec_bool_to_integer_i64() {
        test_integer_to_vec_bool_to_integer::<i64>();
    }

    #[test]
    fn test_integer_to_vec_bool_to_integer_i128() {
        test_integer_to_vec_bool_to_integer::<i128>();
    }

    #[test]
    fn test_integer_to_vec_bool_to_integer_isize() {
        test_integer_to_vec_bool_to_integer::<isize>();
    }

    fn test_anyinteger_to_bits<T>()
    where
        T: AnyIntegerType<T, IntegerType = T>,
    {
        let my_t = T::one();
        let my_bits = my_t.to_bits();
        assert_eq!(my_bits.inner_integer, my_t);
    }

    #[test]
    fn test_anyinteger_to_bits_u8() {
        test_anyinteger_to_bits::<u8>();
    }

    #[test]
    fn test_anyinteger_to_bits_u16() {
        test_anyinteger_to_bits::<u16>();
    }

    #[test]
    fn test_anyinteger_to_bits_u32() {
        test_anyinteger_to_bits::<u32>();
    }

    #[test]
    fn test_anyinteger_to_bits_u64() {
        test_anyinteger_to_bits::<u64>();
    }

    #[test]
    fn test_anyinteger_to_bits_u128() {
        test_anyinteger_to_bits::<u128>();
    }

    #[test]
    fn test_anyinteger_to_bits_usize() {
        test_anyinteger_to_bits::<usize>();
    }

    #[test]
    fn test_anyinteger_to_bits_i8() {
        test_anyinteger_to_bits::<i8>();
    }

    #[test]
    fn test_anyinteger_to_bits_i16() {
        test_anyinteger_to_bits::<i16>();
    }

    #[test]
    fn test_anyinteger_to_bits_i32() {
        test_anyinteger_to_bits::<i32>();
    }

    #[test]
    fn test_anyinteger_to_bits_i64() {
        test_anyinteger_to_bits::<i64>();
    }

    #[test]
    fn test_anyinteger_to_bits_i128() {
        test_anyinteger_to_bits::<i128>();
    }

    #[test]
    fn test_anyinteger_to_bits_isize() {
        test_anyinteger_to_bits::<isize>();
    }

    fn test_anyinteger_to_named_bits<T>()
    where
        T: AnyIntegerType<T, IntegerType = T>,
    {
        let my_t = T::one();
        let my_bits = my_t.to_named_bits("TestBits");
        assert_eq!(my_bits.name, "TestBits");
    }

    #[test]
    fn test_anyinteger_to_named_bits_u8() {
        test_anyinteger_to_named_bits::<u8>();
    }

    #[test]
    fn test_anyinteger_to_named_bits_u16() {
        test_anyinteger_to_named_bits::<u16>();
    }

    #[test]
    fn test_anyinteger_to_named_bits_u32() {
        test_anyinteger_to_named_bits::<u32>();
    }

    #[test]
    fn test_anyinteger_to_named_bits_u64() {
        test_anyinteger_to_named_bits::<u64>();
    }

    #[test]
    fn test_anyinteger_to_named_bits_u128() {
        test_anyinteger_to_named_bits::<u128>();
    }

    #[test]
    fn test_anyinteger_to_named_bits_usize() {
        test_anyinteger_to_named_bits::<usize>();
    }

    #[test]
    fn test_anyinteger_to_named_bits_i8() {
        test_anyinteger_to_named_bits::<i8>();
    }

    #[test]
    fn test_anyinteger_to_named_bits_i16() {
        test_anyinteger_to_named_bits::<i16>();
    }

    #[test]
    fn test_anyinteger_to_named_bits_i32() {
        test_anyinteger_to_named_bits::<i32>();
    }

    #[test]
    fn test_anyinteger_to_named_bits_i64() {
        test_anyinteger_to_named_bits::<i64>();
    }

    #[test]
    fn test_anyinteger_to_named_bits_i128() {
        test_anyinteger_to_named_bits::<i128>();
    }

    #[test]
    fn test_anyinteger_to_named_bits_isize() {
        test_anyinteger_to_named_bits::<isize>();
    }

    #[test]
    fn test_anyinteger_get_max() {
        assert_eq!(u8::MAX, u8::get_max());
        assert_eq!(u16::MAX, u16::get_max());
        assert_eq!(u32::MAX, u32::get_max());
        assert_eq!(u64::MAX, u64::get_max());
        assert_eq!(u128::MAX, u128::get_max());
        assert_eq!(usize::MAX, usize::get_max());
        assert_eq!(i8::MAX, i8::get_max());
        assert_eq!(i16::MAX, i16::get_max());
        assert_eq!(i32::MAX, i32::get_max());
        assert_eq!(i64::MAX, i64::get_max());
        assert_eq!(i128::MAX, i128::get_max());
        assert_eq!(isize::MAX, isize::get_max());
    }

    #[test]
    fn test_anyinteger_get_min() {
        assert_eq!(u8::MIN, u8::get_min());
        assert_eq!(u16::MIN, u16::get_min());
        assert_eq!(u32::MIN, u32::get_min());
        assert_eq!(u64::MIN, u64::get_min());
        assert_eq!(u128::MIN, u128::get_min());
        assert_eq!(usize::MIN, usize::get_min());
        assert_eq!(i8::MIN, i8::get_min());
        assert_eq!(i16::MIN, i16::get_min());
        assert_eq!(i32::MIN, i32::get_min());
        assert_eq!(i64::MIN, i64::get_min());
        assert_eq!(i128::MIN, i128::get_min());
        assert_eq!(isize::MIN, isize::get_min());
    }

    #[test]
    fn test_anyinteger_is_signed() {
        assert_eq!(<u8 as AnyIntegerType<u8>>::is_signed(), false);
        assert_eq!(<u16 as AnyIntegerType<u16>>::is_signed(), false);
        assert_eq!(<u32 as AnyIntegerType<u32>>::is_signed(), false);
        assert_eq!(<u64 as AnyIntegerType<u64>>::is_signed(), false);
        assert_eq!(<u128 as AnyIntegerType<u128>>::is_signed(), false);
        assert_eq!(<usize as AnyIntegerType<usize>>::is_signed(), false);
        assert_eq!(<i8 as AnyIntegerType<i8>>::is_signed(), true);
        assert_eq!(<i16 as AnyIntegerType<i16>>::is_signed(), true);
        assert_eq!(<i32 as AnyIntegerType<i32>>::is_signed(), true);
        assert_eq!(<i64 as AnyIntegerType<i64>>::is_signed(), true);
        assert_eq!(<i128 as AnyIntegerType<i128>>::is_signed(), true);
        assert_eq!(<isize as AnyIntegerType<isize>>::is_signed(), true);
    }

    #[test]
    fn test_anyinteger_max_settable_bits() {
        assert_eq!(<u8 as AnyIntegerType<u8>>::max_settable_bits(), 8);
        assert_eq!(<u16 as AnyIntegerType<u16>>::max_settable_bits(), 16);
        assert_eq!(<u32 as AnyIntegerType<u32>>::max_settable_bits(), 32);
        assert_eq!(<u64 as AnyIntegerType<u64>>::max_settable_bits(), 64);
        assert_eq!(<u128 as AnyIntegerType<u128>>::max_settable_bits(), 128);
        assert_eq!(<i8 as AnyIntegerType<i8>>::max_settable_bits(), 7);
        assert_eq!(<i16 as AnyIntegerType<i16>>::max_settable_bits(), 15);
        assert_eq!(<i32 as AnyIntegerType<i32>>::max_settable_bits(), 31);
        assert_eq!(<i64 as AnyIntegerType<i64>>::max_settable_bits(), 63);
        assert_eq!(<i128 as AnyIntegerType<i128>>::max_settable_bits(), 127);
    }

    #[test]
    fn test_anyinteger_max_bit_index() {
        assert_eq!(<u8 as AnyIntegerType<u8>>::max_bit_index(), 7);
        assert_eq!(<u16 as AnyIntegerType<u16>>::max_bit_index(), 15);
        assert_eq!(<u32 as AnyIntegerType<u32>>::max_bit_index(), 31);
        assert_eq!(<u64 as AnyIntegerType<u64>>::max_bit_index(), 63);
        assert_eq!(<u128 as AnyIntegerType<u128>>::max_bit_index(), 127);
        assert_eq!(<i8 as AnyIntegerType<i8>>::max_bit_index(), 6);
        assert_eq!(<i16 as AnyIntegerType<i16>>::max_bit_index(), 14);
        assert_eq!(<i32 as AnyIntegerType<i32>>::max_bit_index(), 30);
        assert_eq!(<i64 as AnyIntegerType<i64>>::max_bit_index(), 62);
        assert_eq!(<i128 as AnyIntegerType<i128>>::max_bit_index(), 126);
    }

    fn test_anyinteger_set_and_get_bit<T>()
    where
        T: AnyIntegerType<T, IntegerType = T>,
    {
        let mut my_t = T::zero();
        for index in 0..T::max_bit_index() {
            assert_eq!(my_t.bit(index), false);
            my_t.set_bit(index, true);
            assert_eq!(my_t.bit(index), true);
        }
    }

    #[test]
    fn test_anyinteger_set_and_get_bit_u8() {
        test_anyinteger_set_and_get_bit::<u8>();
    }

    #[test]
    fn test_anyinteger_set_and_get_bit_u16() {
        test_anyinteger_set_and_get_bit::<u16>();
    }

    #[test]
    fn test_anyinteger_set_and_get_bit_u32() {
        test_anyinteger_set_and_get_bit::<u32>();
    }

    #[test]
    fn test_anyinteger_set_and_get_bit_u64() {
        test_anyinteger_set_and_get_bit::<u64>();
    }

    #[test]
    fn test_anyinteger_set_and_get_bit_u128() {
        test_anyinteger_set_and_get_bit::<u128>();
    }

    #[test]
    fn test_anyinteger_set_and_get_bit_usize() {
        test_anyinteger_set_and_get_bit::<usize>();
    }

    #[test]
    fn test_anyinteger_set_and_get_bit_i8() {
        test_anyinteger_set_and_get_bit::<i8>();
    }

    #[test]
    fn test_anyinteger_set_and_get_bit_i16() {
        test_anyinteger_set_and_get_bit::<i16>();
    }

    #[test]
    fn test_anyinteger_set_and_get_bit_i32() {
        test_anyinteger_set_and_get_bit::<i32>();
    }

    #[test]
    fn test_anyinteger_set_and_get_bit_i64() {
        test_anyinteger_set_and_get_bit::<i64>();
    }

    #[test]
    fn test_anyinteger_set_and_get_bit_i128() {
        test_anyinteger_set_and_get_bit::<i128>();
    }

    #[test]
    fn test_anyinteger_set_and_get_bit_isize() {
        test_anyinteger_set_and_get_bit::<isize>();
    }
}

/// Encapsulates a primitive integer and a vec\<bool> representing its bits.
///
/// Instances of the Bits contain within them a primitive integer and a vector of booleans
/// each representing a bit within the primitive integer.  Those instances can be created
/// from a primitive integer or a vector of booleans.  Both the primitive integer and the
/// vector of booleans within the Bits are kept in constant sync.  As a result, reads from
/// both the primitive integer and the vector of boolean bits within it are very fast.
#[derive(PartialEq, Debug)]
pub struct Bits<T>
where
    T: Zero
        + One
        + Not<Output = T>
        + BitAndAssign
        + Debug
        + AnyIntegerType<T, IntegerType = T>
        + PartialOrd
        + Binary
        + Display,
{
    name: String,
    inner_integer: T,
    inner_vec_bool: Vec<bool>,
}

impl<T> Default for Bits<T>
where
    T: Zero
        + One
        + Not<Output = T>
        + BitAndAssign
        + Debug
        + AnyIntegerType<T, IntegerType = T>
        + PartialOrd
        + Binary
        + Display,
{
    /// Creates a default Bits instance in which the value of the primitive integer is
    /// the default value of that primitive integer type and the optional name of the
    /// Bits value is "unnamed Bits".
    ///
    /// ```
    /// /// To create a Bits<u8> instance containing 0u8 (the default value for a u8), the
    /// /// following example is all you need:
    /// use bitman::Bits;
    /// let my_bits = Bits::<u8>::default();
    /// ```
    fn default() -> Bits<T> {
        let name: String = "unnamed Bits".to_string();
        let inner_vec_bool: Vec<bool> = vec![false; T::max_bit_index()];
        let inner_integer: T = Default::default();
        let mut new_bits = Bits {
            name,
            inner_integer,
            inner_vec_bool,
        };
        new_bits.inner_vec_bool = Vec::<bool>::from(WrappedInteger::<T>(new_bits.inner_integer));
        new_bits
    }
}

/// The methods implemented for Bits form the tools needed to create, read, update and
/// delete both instances of Bits and the bits stored in the vector of booleans within.
impl<T> Bits<T>
where
    T: AnyIntegerType<T, IntegerType = T>
        + One
        + Zero
        + Not<Output = T>
        + BitAndAssign
        + PartialOrd
        + Binary
        + Display
        + Debug,
{
    /// Creates a Bits instance containing the name and primitive integer provided.
    ///
    /// ```
    /// /// Creating a named Bits instance containing a u8 is easy.  The following example
    /// /// shows how:
    /// use bitman::Bits;
    /// let my_bits = Bits::<u8>::new("NameOfBits", 0u8);
    /// ```
    pub fn new(name: &str, inner_integer: T) -> Bits<T> {
        let mut new_bits = Bits::<T>::default();
        new_bits.name = name.to_string();
        new_bits.inner_integer = inner_integer;
        new_bits.inner_vec_bool = Vec::<bool>::from(WrappedInteger::<T>(new_bits.inner_integer));
        new_bits
    }

    /// Creates a Bits instance containing the primitive integer provided.
    ///
    /// ```
    /// /// Bits instances can be easily created from existing primitive integers.  To make
    /// /// a Bits instance from a u8 is as simple as this:
    /// use bitman::Bits;
    /// let my_u8 = 0u8;
    /// let my_bits = Bits::<u8>::from_integer(my_u8);
    /// ```
    pub fn from_integer(integer: T) -> Bits<T> {
        Bits::<T>::new("unnamed Bits", integer)
    }

    /// Creates a Bits instance containing the name and primitive integer provided.
    ///
    /// Note: This is directly wraps Bits::new() and does the same thing.  We liked this name
    /// better but the accepted standard for creating new instances is new().
    /// ```
    /// /// Creating a named Bits instance containing a u8 is easy.  The following example
    /// /// shows how:
    /// use bitman::Bits;
    /// let my_bits = Bits::<u8>::from_named_integer("Name Of Bits", 0u8);
    /// ```
    pub fn from_named_integer(name: &str, integer: T) -> Bits<T> {
        Bits::<T>::new(name, integer)
    }

    /// Creates a Bits instance containing the name and vector of booleans provided.
    ///
    /// ```
    /// /// Creating a named Bits instance from a vector of boolean values describing its
    /// /// bits is incredibly easy.  The following example shows how:
    /// use bitman::Bits;
    /// let my_vec_bool = vec![false, false, false, false, false, false, false, true];
    /// let my_bits = Bits::<u8>::from_named_vec_bool("Name Of Bits", my_vec_bool);
    /// ```
    pub fn from_named_vec_bool(name: &str, vec_bool: Vec<bool>) -> Bits<T> {
        if vec_bool.len() == T::max_settable_bits() {
            let mut new_bits = Bits::<T>::new(name, T::zero());
            new_bits.inner_vec_bool = vec_bool;
            new_bits.inner_integer = *(WrappedInteger::<T>::from(new_bits.inner_vec_bool.clone()));
            return new_bits;
        } else {
            panic!(
                "vec_bool len is {} which is not bit length of T: {}",
                vec_bool.len(),
                T::max_settable_bits()
            );
        }
    }

    /// Creates a Bits instance containing the vector of booleans provided.
    ///
    /// ```
    /// /// Creating a Bits instance from a vector of boolean values describing its
    /// /// bits is incredibly easy.  The following example shows how:
    /// use bitman::Bits;
    /// let my_vec_bool = vec![false, false, false, false, false, false, false, true];
    /// let my_bits = Bits::<u8>::from_vec_bool(my_vec_bool);
    /// assert_eq!(my_bits.as_integer(), 1);
    /// ```
    pub fn from_vec_bool(vec_bool: Vec<bool>) -> Bits<T> {
        Bits::from_named_vec_bool("unnamed Bits", vec_bool)
    }

    /// Updates the internal integer representation to match the interal vector of booleans.
    ///
    /// This method is only for internal use within Bits' implementation.
    fn update_integer_from_vec_bool_bit(&mut self, bit_number: usize, bit_value: &bool) {
        let mut mask = T::one() << (T::max_bit_index() - bit_number);
        if *bit_value {
            self.inner_integer |= mask;
        } else {
            mask = !mask;
            self.inner_integer &= mask;
        }
    }

    /// Sets a single bit within the Bits instance.
    ///
    /// Note: true sets the bit to 1.  false sets the bit to 0.
    ///
    /// ```
    /// /// Setting a single bit within a Bits couldn't be simpler.  The following example
    /// /// shows how:
    /// use bitman::Bits;
    /// let mut my_bits = Bits::<u8>::default();
    /// my_bits.set_bit(7, true);
    /// ```
    pub fn set_bit(&mut self, bit_number: usize, bit_value: bool) {
        self.inner_vec_bool[bit_number] = bit_value;
        self.update_integer_from_vec_bool_bit(bit_number, &bit_value);
    }

    /// Gets the value of a single bit within the Bits instance.
    ///
    /// Note: A 1 value returns true.  A 0 value returns false.
    ///
    /// ```
    /// /// Retrieving the value of a single bit from a Bits is easy.  Here's how:
    /// use bitman::Bits;
    /// let my_bits = Bits::<u8>::from_integer(1u8);
    /// assert_eq!(my_bits.bit(7), true);
    /// ```
    pub fn bit(&self, bit_number: usize) -> bool {
        self.inner_vec_bool[bit_number]
    }

    /// Returns a borrowed instance of the internal vector of booleans.
    ///
    /// ```
    /// /// Need to read the internal vector of booleans from your Bits? Here's how:
    /// use bitman::Bits;
    /// let my_bits = Bits::<u8>::from_integer(1u8);
    /// let my_vec_bool = my_bits.as_vec_bool();
    /// assert_eq!(my_vec_bool[7], my_bits.bit(7));
    /// ```
    pub fn as_vec_bool(&self) -> &Vec<bool> {
        &self.inner_vec_bool
    }

    /// Returns a borrowed mutable instance of the internal vector of booleans.
    ///
    /// ```
    /// /// Need to change the internal vector of booleans in your Bits? Here's how:
    /// use bitman::Bits;
    /// let mut my_bits = Bits::<u8>::from_integer(1u8);
    /// let mut my_vec_bool = my_bits.as_mut_vec_bool();
    /// my_vec_bool[2] = true;
    /// assert_eq!(my_bits.bit(2), true);
    /// ```
    pub fn as_mut_vec_bool(&mut self) -> &mut Vec<bool> {
        &mut self.inner_vec_bool
    }

    /// Returns a borrowed instance of the internal integer.
    ///
    /// ```
    /// /// Need to read the internal integer?  Here's how:
    /// use bitman::Bits;
    /// let my_bits = Bits::<u8>::from_integer(1u8);
    /// let my_u8 = my_bits.as_integer();
    /// assert_eq!(my_u8, 1u8);
    /// ```
    pub fn as_integer(&self) -> T {
        return self.inner_integer;
    }

    /// Returns a borrowed mutable instance of the internal integer.
    ///
    /// ```
    /// /// Need to modify the internal integer?  Here's how:
    /// use bitman::Bits;
    /// let mut my_bits = Bits::<u8>::from_integer(1u8);
    /// let mut my_u8 = my_bits.as_mut_integer();
    /// *my_u8 += 1;
    /// assert_eq!(my_bits.as_integer(), 2u8);
    /// ```
    pub fn as_mut_integer(&mut self) -> &mut T {
        return &mut self.inner_integer;
    }

    /// Get a reference to the Bits instance's name.
    ///
    /// ```
    /// /// Need to know what your Bits instance is named?  Here's how:
    /// use bitman::Bits;
    /// let my_bits = Bits::<u8>::from_integer(1u8);
    /// assert_eq!(my_bits.name(), "unnamed Bits");
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Set the bits's name.
    ///
    /// ```
    /// /// Need to change what your Bits instance is named?  Here's how:
    /// use bitman::Bits;
    /// let mut my_bits = Bits::<u8>::from_integer(1u8);
    /// my_bits.set_name("George");
    /// assert_eq!(my_bits.name(), "George");
    /// ```
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl<Idx, T> Index<Idx> for Bits<T>
where
    Idx: SliceIndex<[bool]>,
    T: AnyIntegerType<T, IntegerType = T> + BitAndAssign,
{
    type Output = Idx::Output;
    /// Returns a borrowed boolean representation of the indexed bits.
    ///
    /// ```
    /// /// You can retrieve a borrowed slice of boolean values representing
    /// /// some of the bits from your Bits instance.  Here's how:
    /// use bitman::Bits;
    /// let mut my_bits = Bits::<u8>::from_integer(1u8);
    /// let my_bit_slice = &my_bits[5..8];
    /// assert_eq!(my_bit_slice[0], false);
    /// assert_eq!(my_bit_slice[2], true);
    /// ```
    fn index(&self, index: Idx) -> &Idx::Output {
        &self.inner_vec_bool[index]
    }
}

impl<Idx, T> IndexMut<Idx> for Bits<T>
where
    Idx: SliceIndex<[bool], Output = [bool]>,
    T: Zero + One + AnyIntegerType<T, IntegerType = T>,
{
    /// Returns a borrowed mutable slice of the indexed bits.
    ///
    /// Note: While the slice is mutable, it does not change the bits in the Bits instance.
    /// If you need to modify bits in your Bits instance, use Bits::set_bit().
    ///
    /// ```
    /// /// You can retrieve a mutable borrowed slice of boolean values representing
    /// /// some of the bits from your Bits instance.  Here's how:
    /// use bitman::Bits;
    /// let mut my_bits = Bits::<u8>::from_integer(0u8);
    /// let my_bit_slice = &mut my_bits[0..8];
    /// my_bit_slice[0] = true;
    /// assert_eq!(my_bit_slice, [true, false, false, false, false, false, false, false]);
    /// ```
    fn index_mut(&mut self, index: Idx) -> &mut Idx::Output {
        &mut self.inner_vec_bool[index]
    }
}

impl<T> Deref for Bits<T>
where
    T: Zero + One + BitAndAssign + Debug + AnyIntegerType<T, IntegerType = T>,
{
    type Target = T;

    /// Returns a borrowed representation of the internal primitive integer.
    fn deref(&self) -> &T {
        &self.inner_integer
    }
}

impl<T> DerefMut for Bits<T>
where
    T: Zero + One + Debug + BitAndAssign + AnyIntegerType<T, IntegerType = T>,
{
    /// Returns a borrowed mutable representation of the internal primitive integer.
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner_integer
    }
}

/// A generic wrapper around any primitive integer.
///
/// WrappedInteger was a necessary evil to allow us to implement the From trait
/// to convert primitive integers to vectors of booleans and vectors of booleans
/// to primitive integers.
#[derive(Default)]
pub struct WrappedInteger<T>(pub T)
where
    T: AnyIntegerType<T, IntegerType = T>;

impl<T> From<WrappedInteger<T>> for Vec<bool>
where
    T: AnyIntegerType<T, IntegerType = T>,
{
    /// Converts a WrappedInteger to a Vec\<bool> of it's bits.
    ///
    /// ```
    /// /// Converting an integer to a Vec<bool> is easy! Here's how:
    /// use bitman::WrappedInteger;
    /// let my_u8 = 1u8;
    /// let my_vec_bool = Vec::<bool>::from(WrappedInteger::<u8>(my_u8));
    /// assert_eq!(my_vec_bool[6], false);
    /// assert_eq!(my_vec_bool[7], true);
    /// ```
    fn from(item: WrappedInteger<T>) -> Vec<bool> {
        let mut new_vec: Vec<bool> = Vec::new();
        for index in 0..T::max_settable_bits() {
            if index >= new_vec.len() {
                new_vec.push((*item).bit(index));
            } else {
                new_vec[index] = (*item).bit(index);
            }
        }
        return new_vec;
    }
}

impl<T> From<Vec<bool>> for WrappedInteger<T>
where
    T: AnyIntegerType<T, IntegerType = T> + Default,
{
    /// Converts a Vec\<bool> to a WrappedInteger
    ///
    /// ```
    /// /// Need to convery a Vec<bool> to an integer?  Here's how:
    /// use bitman::WrappedInteger;
    /// let my_vec_bool = vec![false, false, false, false, false, false, false, true];
    /// let my_u8 = *WrappedInteger::<u8>::from(my_vec_bool);
    /// assert_eq!(my_u8, 1u8);
    /// ```
    fn from(item: Vec<bool>) -> WrappedInteger<T> {
        let mut new_integer: WrappedInteger<T> = Default::default();
        for (index, item) in item.iter().enumerate().take(T::max_settable_bits()) {
            (*new_integer).set_bit(index, *item);
        }
        return new_integer;
    }
}

pub trait AnyIntegerType<T>:
    Shl<usize, Output = T>
    + BitOr<Output = T>
    + BitOrAssign
    + BitAnd<Output = T>
    + BitXor<Output = T>
    + BitXorAssign
    + BitAndAssign
    + Sub<Output = T>
    + Mul
    + Rem<Output = T>
    + Default
    + Copy
    + From<bool>
    + PartialOrd
    + TryFrom<usize>
    + Binary
    + Display
    + Zero
    + One
    + Not<Output = T>
    + Debug
where
    T: AnyIntegerType<T, IntegerType = T>,
{
    type IntegerType: Shl<usize, Output = T>
        + BitOr<Output = T>
        + BitOrAssign
        + BitAnd<Output = T>
        + BitXor<Output = T>
        + BitXorAssign
        + BitAndAssign
        + Sub<Output = T>
        + Mul
        + Rem<Output = T>
        + Default
        + AnyIntegerType<T>
        + From<bool>
        + PartialOrd
        + TryFrom<usize>
        + Binary
        + Display
        + Zero
        + One
        + Not<Output = T>
        + Debug;

    /// Creates a Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a Bits instance?  Here's how:
    /// use bitman::{Bits, AnyIntegerType};
    /// let my_u8 = 1u8;
    /// let my_bits = my_u8.to_bits();
    /// assert_eq!(*my_bits, my_u8);
    /// ```
    fn to_bits(&self) -> Bits<T>;

    /// Creates a named Bits instance from self.
    ///
    /// ```
    /// /// Have an integer and need a named Bits instance?  Here's how:
    /// use bitman::{Bits, AnyIntegerType};
    /// let my_u8 = 1u8;
    /// let my_bits = my_u8.to_named_bits("TestBits");
    /// assert_eq!(my_bits.name(), "TestBits");
    /// assert_eq!(*my_bits, my_u8);
    /// ```
    fn to_named_bits(&self, name: &str) -> Bits<T>;

    /// Returns the maximum value of self.
    ///
    /// ```
    /// /// This method returns the maximum value of the integer.  This was
    /// /// necessary since Rust does not appear to provide a way for a trait
    /// /// to depend on a constant value.  As such, for the primitive
    /// /// integers included with Rust, this simply returns their ::MAX
    /// /// value.  It can be used like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(u8::get_max(), u8::MAX);
    /// ```
    fn get_max() -> T;

    /// Returns the minimum value of self.
    ///
    /// ```
    /// /// This method returns the minimum value of the integer.  This was
    /// /// necessary since Rust does not appear to provide a way for a trait
    /// /// to depend on a constant value.  As such, for the primitive
    /// /// integers included with Rust, this simply returns their ::MIN
    /// /// value.  It can be used like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(u8::get_min(), u8::MIN);
    /// ```
    fn get_min() -> T;

    /// Returns whether this integer is signed.
    ///
    /// ```
    /// /// Here's how to determine if your integer is signed:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(u8::is_signed(), false);
    /// ```
    fn is_signed() -> bool {
        return T::get_min() != T::zero();
    }

    /// Returns the number of settable bits in your integer.
    ///
    /// ```
    /// /// Here's how to determine the number of settable bits:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(u8::max_settable_bits(), 8);
    /// ```
    fn max_settable_bits() -> usize {
        if T::is_signed() {
            return (size_of::<T>() * 8) - 1;
        } else {
            return size_of::<T>() * 8;
        }
    }

    /// Returns the highest possible bit index in self.
    ///
    /// ```
    /// /// Need to know the highest possible bit index for your integer?
    /// /// Here's how:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(u8::max_bit_index(), 7);
    /// ```
    fn max_bit_index() -> usize {
        T::max_settable_bits() - 1
    }

    /// Sets a bit within the integer to bit_value.
    ///
    /// ```
    /// /// Need to set a single bit in an integer without creating a Bits?
    /// /// Here's how:
    /// use bitman::AnyIntegerType;
    /// let mut my_u8 = 0u8;
    /// my_u8.set_bit(7, true);
    /// assert_eq!(my_u8, 1u8);
    /// ```
    fn set_bit(&mut self, bit_index: usize, bit_value: bool) -> Self
    where
        Self: BitAnd<T, Output = Self> + BitOr<T, Output = Self>,
    {
        if T::max_bit_index() < bit_index {
            panic!("bit_number is greater than the number of bits in t");
        }
        let left_shift_amount_to_bit: usize = T::max_bit_index() - bit_index;
        assert!(left_shift_amount_to_bit <= T::max_bit_index());
        let mut mask: T = T::one() << left_shift_amount_to_bit;
        assert!(mask != T::zero());
        if bit_value {
            *self = *self | mask;
        } else {
            mask = !mask;
            assert!(mask != T::zero());
            *self = *self & mask;
        }
        return *self;
    }

    /// Returns the value of a bit from within the integer.
    ///
    /// ```
    /// /// Need to know the value of a single bit in your integer?
    /// /// Here's how:
    /// use bitman::AnyIntegerType;
    /// let my_u8 = 1u8;
    /// assert_eq!(my_u8.bit(6), false);
    /// assert_eq!(my_u8.bit(7), true);
    /// ```
    fn bit(&self, bit_number: usize) -> bool
    where
        Self: BitAnd<T, Output = T>,
    {
        let left_shift_amount_to_bit: usize;
        if T::max_bit_index() < bit_number {
            panic!("bit_number is greater than the number of bits in T.");
        } else {
            left_shift_amount_to_bit = T::max_bit_index() - bit_number;
            let mask = T::one() << left_shift_amount_to_bit;
            return *self & mask > T::zero();
        }
    }
}

impl<T> Deref for WrappedInteger<T>
where
    T: AnyIntegerType<T, IntegerType = T>,
{
    type Target = T;

    /// Returns a borrowed representation of the integer within the WrappedInteger.
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for WrappedInteger<T>
where
    T: AnyIntegerType<T, IntegerType = T>,
{
    /// Returns a borrowed mutable representation of the integer within WrappedInteger.
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl AnyIntegerType<u8> for u8 {
    type IntegerType = u8;

    /// Creates a Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: u8 = 1;
    /// let my_bits = my_integer.to_bits();
    /// assert_eq!(my_bits.as_integer(), 1);
    /// ```
    fn to_bits(&self) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new("unnamed Bits", *self)
    }

    /// Creates a named Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a named Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: u8 = 1;
    /// let my_bits = my_integer.to_named_bits("TestBits");
    /// assert_eq!(my_bits.as_integer(), 1);
    /// assert_eq!(my_bits.name(), "TestBits");
    /// ```
    fn to_named_bits(&self, name: &str) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new(name, *self)
    }

    /// Returns the maximum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MAX value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(u8::get_max(), u8::MAX);
    /// ```
    fn get_max() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MAX;
    }

    /// Returns the minimum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MIN value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(u8::get_min(), u8::MIN);
    /// ```
    fn get_min() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MIN;
    }
}

impl AnyIntegerType<u16> for u16 {
    type IntegerType = u16;

    /// Creates a Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: u16 = 1;
    /// let my_bits = my_integer.to_bits();
    /// assert_eq!(my_bits.as_integer(), 1);
    /// ```
    fn to_bits(&self) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new("unnamed Bits", *self)
    }

    /// Creates a named Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a named Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: u16 = 1;
    /// let my_bits = my_integer.to_named_bits("TestBits");
    /// assert_eq!(my_bits.as_integer(), 1);
    /// assert_eq!(my_bits.name(), "TestBits");
    /// ```
    fn to_named_bits(&self, name: &str) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new(name, *self)
    }

    /// Returns the maximum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MAX value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(u16::get_max(), u16::MAX);
    /// ```
    fn get_max() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MAX;
    }

    /// Returns the minimum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MIN value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(u16::get_min(), u16::MIN);
    /// ```
    fn get_min() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MIN;
    }
}

impl AnyIntegerType<u32> for u32 {
    type IntegerType = u32;

    /// Creates a Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: u32 = 1;
    /// let my_bits = my_integer.to_bits();
    /// assert_eq!(my_bits.as_integer(), 1);
    /// ```
    fn to_bits(&self) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new("unnamed Bits", *self)
    }

    /// Creates a named Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a named Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: u32 = 1;
    /// let my_bits = my_integer.to_named_bits("TestBits");
    /// assert_eq!(my_bits.as_integer(), 1);
    /// assert_eq!(my_bits.name(), "TestBits");
    /// ```
    fn to_named_bits(&self, name: &str) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new(name, *self)
    }

    /// Returns the maximum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MAX value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(u32::get_max(), u32::MAX);
    /// ```
    fn get_max() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MAX;
    }

    /// Returns the minimum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MIN value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(u32::get_min(), u32::MIN);
    /// ```
    fn get_min() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MIN;
    }
}

impl AnyIntegerType<u64> for u64 {
    type IntegerType = u64;

    /// Creates a Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: u64 = 1;
    /// let my_bits = my_integer.to_bits();
    /// assert_eq!(my_bits.as_integer(), 1);
    /// ```
    fn to_bits(&self) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new("unnamed Bits", *self)
    }

    /// Creates a named Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a named Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: u64 = 1;
    /// let my_bits = my_integer.to_named_bits("TestBits");
    /// assert_eq!(my_bits.as_integer(), 1);
    /// assert_eq!(my_bits.name(), "TestBits");
    /// ```
    fn to_named_bits(&self, name: &str) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new(name, *self)
    }

    /// Returns the maximum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MAX value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(u64::get_max(), u64::MAX);
    /// ```
    fn get_max() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MAX;
    }

    /// Returns the minimum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MIN value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(u64::get_min(), u64::MIN);
    /// ```
    fn get_min() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MIN;
    }
}

impl AnyIntegerType<u128> for u128 {
    type IntegerType = u128;

    /// Creates a Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: u128 = 1;
    /// let my_bits = my_integer.to_bits();
    /// assert_eq!(my_bits.as_integer(), 1);
    /// ```
    fn to_bits(&self) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new("unnamed Bits", *self)
    }

    /// Creates a named Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a named Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: u128 = 1;
    /// let my_bits = my_integer.to_named_bits("TestBits");
    /// assert_eq!(my_bits.as_integer(), 1);
    /// assert_eq!(my_bits.name(), "TestBits");
    /// ```
    fn to_named_bits(&self, name: &str) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new(name, *self)
    }

    /// Returns the maximum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MAX value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(u128::get_max(), u128::MAX);
    /// ```
    fn get_max() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MAX;
    }

    /// Returns the minimum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MIN value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(u128::get_min(), u128::MIN);
    /// ```
    fn get_min() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MIN;
    }
}

impl AnyIntegerType<i8> for i8 {
    type IntegerType = i8;

    /// Creates a Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: i8 = 1;
    /// let my_bits = my_integer.to_bits();
    /// assert_eq!(my_bits.as_integer(), 1);
    /// ```
    fn to_bits(&self) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new("unnamed Bits", *self)
    }

    /// Creates a named Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a named Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: i8 = 1;
    /// let my_bits = my_integer.to_named_bits("TestBits");
    /// assert_eq!(my_bits.as_integer(), 1);
    /// assert_eq!(my_bits.name(), "TestBits");
    /// ```
    fn to_named_bits(&self, name: &str) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new(name, *self)
    }

    /// Returns the maximum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MAX value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(i8::get_max(), i8::MAX);
    /// ```
    fn get_max() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MAX;
    }

    /// Returns the minimum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MIN value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(i8::get_min(), i8::MIN);
    /// ```
    fn get_min() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MIN;
    }
}

impl AnyIntegerType<i16> for i16 {
    type IntegerType = i16;

    /// Creates a Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: i16 = 1;
    /// let my_bits = my_integer.to_bits();
    /// assert_eq!(my_bits.as_integer(), 1);
    /// ```
    fn to_bits(&self) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new("unnamed Bits", *self)
    }

    /// Creates a named Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a named Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: i16 = 1;
    /// let my_bits = my_integer.to_named_bits("TestBits");
    /// assert_eq!(my_bits.as_integer(), 1);
    /// assert_eq!(my_bits.name(), "TestBits");
    /// ```
    fn to_named_bits(&self, name: &str) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new(name, *self)
    }

    /// Returns the maximum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MAX value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(i16::get_max(), i16::MAX);
    /// ```
    fn get_max() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MAX;
    }

    /// Returns the minimum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MIN value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(i16::get_min(), i16::MIN);
    /// ```
    fn get_min() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MIN;
    }
}

impl AnyIntegerType<i32> for i32 {
    type IntegerType = i32;

    /// Creates a Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: i32 = 1;
    /// let my_bits = my_integer.to_bits();
    /// assert_eq!(my_bits.as_integer(), 1);
    /// ```
    fn to_bits(&self) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new("unnamed Bits", *self)
    }

    /// Creates a named Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a named Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: i32 = 1;
    /// let my_bits = my_integer.to_named_bits("TestBits");
    /// assert_eq!(my_bits.as_integer(), 1);
    /// assert_eq!(my_bits.name(), "TestBits");
    /// ```
    fn to_named_bits(&self, name: &str) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new(name, *self)
    }

    /// Returns the maximum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MAX value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(i32::get_max(), i32::MAX);
    /// ```
    fn get_max() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MAX;
    }

    /// Returns the minimum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MIN value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(i32::get_min(), i32::MIN);
    /// ```
    fn get_min() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MIN;
    }
}

impl AnyIntegerType<i64> for i64 {
    type IntegerType = i64;

    /// Creates a Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: i64 = 1;
    /// let my_bits = my_integer.to_bits();
    /// assert_eq!(my_bits.as_integer(), 1);
    /// ```
    fn to_bits(&self) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new("unnamed Bits", *self)
    }

    /// Creates a named Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a named Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: i64 = 1;
    /// let my_bits = my_integer.to_named_bits("TestBits");
    /// assert_eq!(my_bits.as_integer(), 1);
    /// assert_eq!(my_bits.name(), "TestBits");
    /// ```
    fn to_named_bits(&self, name: &str) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new(name, *self)
    }

    /// Returns the maximum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MAX value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(i64::get_max(), i64::MAX);
    /// ```
    fn get_max() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MAX;
    }

    /// Returns the minimum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MIN value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(i64::get_min(), i64::MIN);
    /// ```
    fn get_min() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MIN;
    }
}

impl AnyIntegerType<i128> for i128 {
    type IntegerType = i128;

    /// Creates a Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: i128 = 1;
    /// let my_bits = my_integer.to_bits();
    /// assert_eq!(my_bits.as_integer(), 1);
    /// ```
    fn to_bits(&self) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new("unnamed Bits", *self)
    }

    /// Creates a named Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a named Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: i128 = 1;
    /// let my_bits = my_integer.to_named_bits("TestBits");
    /// assert_eq!(my_bits.as_integer(), 1);
    /// assert_eq!(my_bits.name(), "TestBits");
    /// ```
    fn to_named_bits(&self, name: &str) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new(name, *self)
    }

    /// Returns the maximum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MAX value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(i128::get_max(), i128::MAX);
    /// ```
    fn get_max() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MAX;
    }

    /// Returns the minimum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MIN value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(i128::get_min(), i128::MIN);
    /// ```
    fn get_min() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MIN;
    }
}

impl AnyIntegerType<usize> for usize {
    type IntegerType = usize;

    /// Creates a Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: usize = 1;
    /// let my_bits = my_integer.to_bits();
    /// assert_eq!(my_bits.as_integer(), 1);
    /// ```
    fn to_bits(&self) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new("unnamed Bits", *self)
    }

    /// Creates a named Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a named Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: usize = 1;
    /// let my_bits = my_integer.to_named_bits("TestBits");
    /// assert_eq!(my_bits.as_integer(), 1);
    /// assert_eq!(my_bits.name(), "TestBits");
    /// ```
    fn to_named_bits(&self, name: &str) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new(name, *self)
    }

    /// Returns the maximum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MAX value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(usize::get_max(), usize::MAX);
    /// ```
    fn get_max() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MAX;
    }

    /// Returns the minimum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MIN value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(usize::get_min(), usize::MIN);
    /// ```
    fn get_min() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MIN;
    }
}

impl AnyIntegerType<isize> for isize {
    type IntegerType = isize;

    /// Creates a Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: isize = 1;
    /// let my_bits = my_integer.to_bits();
    /// assert_eq!(my_bits.as_integer(), 1);
    /// ```
    fn to_bits(&self) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new("unnamed Bits", *self)
    }

    /// Creates a named Bits instance from an integer.
    ///
    /// ```
    /// /// Have an integer and need a named Bits?  Here's one way:
    /// use bitman::AnyIntegerType;
    /// let my_integer: isize = 1;
    /// let my_bits = my_integer.to_named_bits("TestBits");
    /// assert_eq!(my_bits.as_integer(), 1);
    /// assert_eq!(my_bits.name(), "TestBits");
    /// ```
    fn to_named_bits(&self, name: &str) -> Bits<Self::IntegerType>
    where
        Self: AnyIntegerType<Self::IntegerType>,
    {
        Bits::new(name, *self)
    }

    /// Returns the maximum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MAX value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(isize::get_max(), isize::MAX);
    /// ```
    fn get_max() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MAX;
    }

    /// Returns the minimum value of an integer.
    ///
    /// ```
    /// /// This is a wrapper around the ::MIN value from the integer
    /// /// but can also be used as a function like this:
    /// use bitman::AnyIntegerType;
    /// assert_eq!(isize::get_min(), isize::MIN);
    /// ```
    fn get_min() -> <Self as AnyIntegerType<Self>>::IntegerType {
        return <Self as AnyIntegerType<Self>>::IntegerType::MIN;
    }
}
