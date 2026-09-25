// src/traits/is_positive.rs : `IsPositive`

/// Trait defining instance method `is_positive() : bool` that indicates
/// whether the implementing type instance has a value that is deemed to be
/// positive.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-IsPositive-for-built_ins"`
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
pub trait IsPositive {
    fn is_positive(&self) -> bool;
}


#[cfg(any(test, not(feature = "nostd")))]
impl<T : IsPositive + ?Sized> IsPositive for Box<T> {
    fn is_positive(&self) -> bool {
        (**self).is_positive()
    }
}

#[cfg(any(test, not(feature = "nostd")))]
impl<T : IsPositive + ?Sized> IsPositive for std::rc::Rc<T> {
    fn is_positive(&self) -> bool {
        (**self).is_positive()
    }
}


#[cfg(feature = "implement-IsPositive-for-built_ins")]
mod impl_for_built_ins {
    #![allow(non_snake_case)]

    macro_rules! implement_IsPositive_fp_ {
        ($type:tt) => {
            impl super::IsPositive for $type {
                #[inline]
                fn is_positive(&self) -> bool {
                    $type::is_sign_positive(*self)
                }
            }
        };
    }

    macro_rules! implement_IsPositive_i_ {
        ($type:tt) => {
            impl super::IsPositive for $type {
                #[inline]
                fn is_positive(&self) -> bool {
                    *self > 0
                }
            }
        };
    }

    implement_IsPositive_fp_!(f32);
    implement_IsPositive_fp_!(f64);

    implement_IsPositive_i_!(i8);
    implement_IsPositive_i_!(i16);
    implement_IsPositive_i_!(i32);
    implement_IsPositive_i_!(i64);
    implement_IsPositive_i_!(i128);
    implement_IsPositive_i_!(isize);

    implement_IsPositive_i_!(u8);
    implement_IsPositive_i_!(u16);
    implement_IsPositive_i_!(u32);
    implement_IsPositive_i_!(u64);
    implement_IsPositive_i_!(u128);
    implement_IsPositive_i_!(usize);
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::IsPositive;

    use std::rc::Rc;


    #[allow(unused)]
    fn as_IsPositive<T : IsPositive>(t : &T) -> &impl IsPositive {
        t
    }

    fn assert_is_positive_<T : IsPositive + std::fmt::Debug>(
        value : &T,
        expected : bool,
    ) {
        assert_eq!(expected, value.is_positive());

        let via = as_IsPositive(value);

        assert_eq!(expected, via.is_positive());
    }


    mod TEST_CUSTOM_TYPE {
        #![allow(non_snake_case)]

        use super::*;


        #[derive(Debug)]
        struct CustomType {
            value : i32,
        }

        impl IsPositive for CustomType {
            fn is_positive(&self) -> bool {
                self.value > 0
            }
        }


        #[test]
        fn TEST_WHEN_NEGATIVE() {
            let ct = CustomType {
                value : -1
            };

            assert!(!ct.is_positive());

            let ct = &ct;

            assert!(!ct.is_positive());
        }

        #[test]
        fn TEST_WHEN_ZERO() {
            let ct = CustomType {
                value : 0
            };

            assert!(!ct.is_positive());

            let ct = &ct;

            assert!(!ct.is_positive());
        }

        #[test]
        fn TEST_WHEN_POSITIVE() {
            let ct = CustomType {
                value : 1
            };

            assert!(ct.is_positive());

            let ct = &ct;

            assert!(ct.is_positive());
        }

        #[test]
        fn TEST_WHEN_POSITIVE_IN_Box() {
            {
                let ct = Box::new(CustomType {
                    value : 1
                });

                assert!(ct.is_positive());

                let ct = &ct;

                assert!(ct.is_positive());
            }

            {
                let ct = &Box::new(CustomType {
                    value : 1
                });

                assert!(ct.is_positive());

                let ct = &ct;

                assert!(ct.is_positive());
            }

            {
                let ct = Box::new(&CustomType {
                    value : 1
                });

                assert!(ct.is_positive());

                let ct = &ct;

                assert!(ct.is_positive());
            }

            {
                let ct = &Box::new(&CustomType {
                    value : 1
                });

                assert!(ct.is_positive());

                let ct = &ct;

                assert!(ct.is_positive());
            }
        }

        #[test]
        fn TEST_WHEN_POSITIVE_IN_Rc() {
            {
                let ct = Rc::new(CustomType {
                    value : 1
                });

                assert!(ct.is_positive());

                let ct = &ct;

                assert!(ct.is_positive());
            }

            {
                let ct = &Rc::new(CustomType {
                    value : 1
                });

                assert!(ct.is_positive());

                let ct = &ct;

                assert!(ct.is_positive());
            }

            {
                let ct = Rc::new(&CustomType {
                    value : 1
                });

                assert!(ct.is_positive());

                let ct = &ct;

                assert!(ct.is_positive());
            }

            {
                let ct = &Rc::new(&CustomType {
                    value : 1
                });

                assert!(ct.is_positive());

                let ct = &ct;

                assert!(ct.is_positive());
            }
        }
    }


    #[cfg(feature = "implement-IsPositive-for-built_ins")]
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
            T : Copy + IsPositive + std::fmt::Debug,
        {
            assert_is_positive_(&zero, false);
            assert_is_positive_(&one, true);
            assert_is_positive_(&neg_one, false);
            assert_is_positive_(&min, false);
            assert_is_positive_(&max, true);
        }

        fn assert_unsigned_<T>(
            zero : T,
            one : T,
            max : T,
        ) where
            T : Copy + IsPositive + std::fmt::Debug,
        {
            assert_is_positive_(&zero, false);
            assert_is_positive_(&one, true);
            assert_is_positive_(&max, true);
        }

        fn assert_float_<T>(
            value : T,
            expected : bool,
        ) where
            T : Copy + IsPositive + std::fmt::Debug,
        {
            assert_is_positive_(&value, expected);

            let boxed = Box::new(value);

            assert_eq!(expected, boxed.is_positive());

            let boxed = &boxed;

            assert_eq!(expected, boxed.is_positive());

            let rc = Rc::new(value);

            assert_eq!(expected, rc.is_positive());

            let rc = &rc;

            assert_eq!(expected, rc.is_positive());
        }


        mod TEST_f32 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_SIGN_CASES() {
                assert_float_(0.0f32, true);
                assert_float_(-0.0f32, false);
                assert_float_(1.0f32, true);
                assert_float_(-1.0f32, false);
                assert_float_(f32::MIN_POSITIVE, true);
                assert_float_(-f32::MIN_POSITIVE, false);
                assert_float_(f32::MIN, false);
                assert_float_(f32::MAX, true);
                assert_float_(f32::INFINITY, true);
                assert_float_(f32::NEG_INFINITY, false);
                assert_float_(f32::from_bits(1), true);
                assert_float_(f32::from_bits(0x8000_0001), false);
                assert_float_(f32::NAN.copysign(1.0), true);
                assert_float_(f32::NAN.copysign(-1.0), false);
            }
        }


        mod TEST_f64 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_SIGN_CASES() {
                assert_float_(0.0f64, true);
                assert_float_(-0.0f64, false);
                assert_float_(1.0f64, true);
                assert_float_(-1.0f64, false);
                assert_float_(f64::MIN_POSITIVE, true);
                assert_float_(-f64::MIN_POSITIVE, false);
                assert_float_(f64::MIN, false);
                assert_float_(f64::MAX, true);
                assert_float_(f64::INFINITY, true);
                assert_float_(f64::NEG_INFINITY, false);
                assert_float_(f64::from_bits(1), true);
                assert_float_(f64::from_bits(0x8000_0000_0000_0001), false);
                assert_float_(f64::NAN.copysign(1.0), true);
                assert_float_(f64::NAN.copysign(-1.0), false);
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
