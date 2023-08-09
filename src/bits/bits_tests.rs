use crate as bitman;
    use bitman::BitMan;
    use num_traits::{One, Zero};

    #[test]
    fn bit_method_test_on_u8() {
        assert_eq!(0u8.bit(&0), bitman::Bit(false));
        assert_eq!(0u8.bit(&1), bitman::Bit(false));
        assert_eq!(0u8.bit(&2), bitman::Bit(false));
        assert_eq!(0u8.bit(&3), bitman::Bit(false));
        assert_eq!(0u8.bit(&4), bitman::Bit(false));
        assert_eq!(0u8.bit(&5), bitman::Bit(false));
        assert_eq!(0u8.bit(&6), bitman::Bit(false));
        assert_eq!(0u8.bit(&7), bitman::Bit(false));
    }

    #[test]
    fn set_bit_method_test_on_u8() {
        let mut my_u8 = 0u8;
        my_u8.set_bit(&0, &bitman::Bit(true));
        assert_eq!(my_u8.bit(&0), bitman::Bit(true));
        my_u8.set_bit(&1, &bitman::Bit(true));
        assert_eq!(my_u8.bit(&1), bitman::Bit(true));
        let mut my_u8 = 0u8;
        my_u8.set_bit(&2, &bitman::Bit(true));
        assert_eq!(my_u8.bit(&2), bitman::Bit(true));
        let mut my_u8 = 0u8;
        my_u8.set_bit(&3, &bitman::Bit(true));
        assert_eq!(my_u8.bit(&3), bitman::Bit(true));
        let mut my_u8 = 0u8;
        my_u8.set_bit(&4, &bitman::Bit(true));
        assert_eq!(my_u8.bit(&4), bitman::Bit(true));
        let mut my_u8 = 0u8;
        my_u8.set_bit(&5, &bitman::Bit(true));
        assert_eq!(my_u8.bit(&5), bitman::Bit(true));
        let mut my_u8 = 0u8;
        my_u8.set_bit(&6, &bitman::Bit(true));
        assert_eq!(my_u8.bit(&6), bitman::Bit(true));
        let mut my_u8 = 0u8;
        my_u8.set_bit(&7, &bitman::Bit(true));
        assert_eq!(my_u8.bit(&7), bitman::Bit(true));
    }

    #[test]
    fn u8_zero_as_bits_compared_set_to_one_and_compared() {
        let mut my_u8_as_bits = 0u8.bits();
        assert_eq!(
            my_u8_as_bits[..],
            [
                bitman::Bit(false),
                bitman::Bit(false),
                bitman::Bit(false),
                bitman::Bit(false),
                bitman::Bit(false),
                bitman::Bit(false),
                bitman::Bit(false),
                bitman::Bit(false)
            ]
        );
        my_u8_as_bits = u8::one().bits();
        assert_eq!(
            my_u8_as_bits[..],
            [
                bitman::Bit(false),
                bitman::Bit(false),
                bitman::Bit(false),
                bitman::Bit(false),
                bitman::Bit(false),
                bitman::Bit(false),
                bitman::Bit(false),
                bitman::Bit(true)
            ]
        );
    }

    #[test]
    fn bits_method_test_on_u8() {
        let new_bits = 0u8.bits();
        assert_eq!(new_bits.bit_len(), 8);
        assert_eq!(new_bits[0], bitman::Bit(false));
        assert_eq!(0u8.bits(), bitman::Bits::new(&[bitman::Bit::zero(); 8]));
    }
    