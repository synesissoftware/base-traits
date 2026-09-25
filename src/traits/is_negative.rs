// src/traits/is_negative.rs : `IsNegative`

/// Trait defining instance method `is_negative() : bool` that indicates
/// whether the implementing type instance has a value that is deemed to be
/// negative.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-IsNegative-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following types:
/// - [`f32`];
/// - [`f64`];
/// - [`i8`];
/// - [`i16`];
/// - [`i32`];
/// - [`i64`];
/// - [`i128`];
/// - [`u8`];
/// - [`u16`];
/// - [`u32`];
/// - [`u64`];
/// - [`u128`];
/// - [`isize`];
/// - [`usize`];
pub trait IsNegative {
    fn is_negative(&self) -> bool;
}


#[cfg(any(test, not(feature = "nostd")))]
impl<T : IsNegative + ?Sized> IsNegative for Box<T> {
    fn is_negative(&self) -> bool {
        (**self).is_negative()
    }
}

#[cfg(any(test, not(feature = "nostd")))]
impl<T : IsNegative + ?Sized> IsNegative for std::rc::Rc<T> {
    fn is_negative(&self) -> bool {
        (**self).is_negative()
    }
}


#[cfg(feature = "implement-IsNegative-for-built_ins")]
mod impl_for_built_ins {
    #![allow(non_snake_case)]

    macro_rules! implement_IsNegative_fp_ {
        ($type:tt) => {
            impl super::IsNegative for $type {
                #[inline]
                fn is_negative(&self) -> bool {
                    $type::is_sign_negative(*self)
                }
            }
        };
    }

    macro_rules! implement_IsNegative_si_ {
        ($type:tt) => {
            impl super::IsNegative for $type {
                #[inline]
                fn is_negative(&self) -> bool {
                    *self < 0
                }
            }
        };
    }

    macro_rules! implement_IsNegative_su_ {
        ($type:tt) => {
            impl super::IsNegative for $type {
                #[inline]
                fn is_negative(&self) -> bool {
                    false
                }
            }
        };
    }

    implement_IsNegative_fp_!(f32);
    implement_IsNegative_fp_!(f64);

    implement_IsNegative_si_!(i8);
    implement_IsNegative_si_!(i16);
    implement_IsNegative_si_!(i32);
    implement_IsNegative_si_!(i64);
    implement_IsNegative_si_!(i128);
    implement_IsNegative_si_!(isize);

    implement_IsNegative_su_!(u8);
    implement_IsNegative_su_!(u16);
    implement_IsNegative_su_!(u32);
    implement_IsNegative_su_!(u64);
    implement_IsNegative_su_!(u128);
    implement_IsNegative_su_!(usize);
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::IsNegative;

    use std::rc::Rc;


    #[allow(unused)]
    fn as_IsNegative<T : IsNegative>(t : &T) -> &impl IsNegative {
        t
    }

    fn assert_is_negative_<T : IsNegative + std::fmt::Debug>(
        value : &T,
        expected : bool,
    ) {
        assert_eq!(expected, value.is_negative());

        let via = as_IsNegative(value);

        assert_eq!(expected, via.is_negative());
    }


    mod TEST_CUSTOM_TYPE {
        #![allow(non_snake_case)]

        use super::*;


        #[derive(Debug)]
        struct CustomType {
            value : i32,
        }

        impl IsNegative for CustomType {
            fn is_negative(&self) -> bool {
                self.value < 0
            }
        }


        #[test]
        fn TEST_WHEN_NEGATIVE() {
            let ct = CustomType {
                value : -1
            };

            assert!(ct.is_negative());

            let ct = &ct;

            assert!(ct.is_negative());
        }

        #[test]
        fn TEST_WHEN_ZERO() {
            let ct = CustomType {
                value : 0
            };

            assert!(!ct.is_negative());

            let ct = &ct;

            assert!(!ct.is_negative());
        }

        #[test]
        fn TEST_WHEN_POSITIVE() {
            let ct = CustomType {
                value : 1
            };

            assert!(!ct.is_negative());

            let ct = &ct;

            assert!(!ct.is_negative());
        }

        #[test]
        fn TEST_WHEN_NEGATIVE_IN_Box() {
            {
                let ct = Box::new(CustomType {
                    value : -1
                });

                assert!(ct.is_negative());

                let ct = &ct;

                assert!(ct.is_negative());
            }

            {
                let ct = &Box::new(CustomType {
                    value : -1
                });

                assert!(ct.is_negative());

                let ct = &ct;

                assert!(ct.is_negative());
            }

            {
                let ct = Box::new(&CustomType {
                    value : -1
                });

                assert!(ct.is_negative());

                let ct = &ct;

                assert!(ct.is_negative());
            }

            {
                let ct = &Box::new(&CustomType {
                    value : -1
                });

                assert!(ct.is_negative());

                let ct = &ct;

                assert!(ct.is_negative());
            }
        }

        #[test]
        fn TEST_WHEN_NEGATIVE_IN_Rc() {
            {
                let ct = Rc::new(CustomType {
                    value : -1
                });

                assert!(ct.is_negative());

                let ct = &ct;

                assert!(ct.is_negative());
            }

            {
                let ct = &Rc::new(CustomType {
                    value : -1
                });

                assert!(ct.is_negative());

                let ct = &ct;

                assert!(ct.is_negative());
            }

            {
                let ct = Rc::new(&CustomType {
                    value : -1
                });

                assert!(ct.is_negative());

                let ct = &ct;

                assert!(ct.is_negative());
            }

            {
                let ct = &Rc::new(&CustomType {
                    value : -1
                });

                assert!(ct.is_negative());

                let ct = &ct;

                assert!(ct.is_negative());
            }
        }
    }


    #[cfg(feature = "implement-IsNegative-for-built_ins")]
    mod TEST_BUILTIN_TYPES {
        #![allow(non_snake_case)]

        use super::*;


        fn assert_signed_<T>(
            zero : T,
            one : T,
            neg_one : T,
            min : T,
            max : T,
        ) where
            T : Copy + IsNegative + std::fmt::Debug,
        {
            assert_is_negative_(&zero, false);
            assert_is_negative_(&one, false);
            assert_is_negative_(&neg_one, true);
            assert_is_negative_(&min, true);
            assert_is_negative_(&max, false);
        }

        fn assert_unsigned_<T>(
            zero : T,
            one : T,
            max : T,
        ) where
            T : Copy + IsNegative + std::fmt::Debug,
        {
            assert_is_negative_(&zero, false);
            assert_is_negative_(&one, false);
            assert_is_negative_(&max, false);
        }

        fn assert_float_<T>(
            value : T,
            expected : bool,
        ) where
            T : Copy + IsNegative + std::fmt::Debug,
        {
            assert_is_negative_(&value, expected);

            let boxed = Box::new(value);

            assert_eq!(expected, boxed.is_negative());

            let boxed = &boxed;

            assert_eq!(expected, boxed.is_negative());

            let rc = Rc::new(value);

            assert_eq!(expected, rc.is_negative());

            let rc = &rc;

            assert_eq!(expected, rc.is_negative());
        }


        mod TEST_f32 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_SIGN_CASES() {
                assert_float_(0.0f32, false);
                assert_float_(-0.0f32, true);
                assert_float_(1.0f32, false);
                assert_float_(-1.0f32, true);
                assert_float_(f32::MIN_POSITIVE, false);
                assert_float_(-f32::MIN_POSITIVE, true);
                assert_float_(f32::MIN, true);
                assert_float_(f32::MAX, false);
                assert_float_(f32::INFINITY, false);
                assert_float_(f32::NEG_INFINITY, true);
                assert_float_(f32::from_bits(1), false);
                assert_float_(f32::from_bits(0x8000_0001), true);
                assert_float_(f32::NAN.copysign(1.0), false);
                assert_float_(f32::NAN.copysign(-1.0), true);
            }
        }


        mod TEST_f64 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_SIGN_CASES() {
                assert_float_(0.0f64, false);
                assert_float_(-0.0f64, true);
                assert_float_(1.0f64, false);
                assert_float_(-1.0f64, true);
                assert_float_(f64::MIN_POSITIVE, false);
                assert_float_(-f64::MIN_POSITIVE, true);
                assert_float_(f64::MIN, true);
                assert_float_(f64::MAX, false);
                assert_float_(f64::INFINITY, false);
                assert_float_(f64::NEG_INFINITY, true);
                assert_float_(f64::from_bits(1), false);
                assert_float_(f64::from_bits(0x8000_0000_0000_0001), true);
                assert_float_(f64::NAN.copysign(1.0), false);
                assert_float_(f64::NAN.copysign(-1.0), true);
            }
        }


        mod TEST_i8 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_SIGN_CASES() {
                assert_signed_(0i8, 1, -1, i8::MIN, i8::MAX);
            }
        }


        mod TEST_i16 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_SIGN_CASES() {
                assert_signed_(0i16, 1, -1, i16::MIN, i16::MAX);
            }
        }


        mod TEST_i32 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_SIGN_CASES() {
                assert_signed_(0i32, 1, -1, i32::MIN, i32::MAX);
            }
        }


        mod TEST_i64 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_SIGN_CASES() {
                assert_signed_(0i64, 1, -1, i64::MIN, i64::MAX);
            }
        }


        mod TEST_i128 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_SIGN_CASES() {
                assert_signed_(0i128, 1, -1, i128::MIN, i128::MAX);
            }
        }


        mod TEST_isize {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_SIGN_CASES() {
                assert_signed_(0isize, 1, -1, isize::MIN, isize::MAX);
            }
        }


        mod TEST_u8 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_SIGN_CASES() {
                assert_unsigned_(0u8, 1, u8::MAX);
            }
        }


        mod TEST_u16 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_SIGN_CASES() {
                assert_unsigned_(0u16, 1, u16::MAX);
            }
        }


        mod TEST_u32 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_SIGN_CASES() {
                assert_unsigned_(0u32, 1, u32::MAX);
            }
        }


        mod TEST_u64 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_SIGN_CASES() {
                assert_unsigned_(0u64, 1, u64::MAX);
            }
        }


        mod TEST_u128 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_SIGN_CASES() {
                assert_unsigned_(0u128, 1, u128::MAX);
            }
        }


        mod TEST_usize {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_SIGN_CASES() {
                assert_unsigned_(0usize, 1, usize::MAX);
            }
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //
