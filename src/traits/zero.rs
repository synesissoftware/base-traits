// src/traits/zero.rs : `Zero`

/// Trait defining class method `zero() : T` that indicates whether
/// the implementing type instance is numerically zero.
pub trait Zero {
    fn zero() -> Self;
}


#[cfg(feature = "implement-Zero-for-built_ins")]
mod impl_for_built_ins {
    #![allow(non_snake_case)]

    macro_rules! implement_IsZero_ {
        ($type:tt, $zero_value:expr) => {
            impl super::Zero for $type {
                #[inline]
                fn zero() -> Self {
                    $zero_value
                }
            }
        };
    }

    implement_IsZero_!(i8, 0);
    implement_IsZero_!(i16, 0);
    implement_IsZero_!(i32, 0);
    implement_IsZero_!(i64, 0);
    implement_IsZero_!(i128, 0);

    implement_IsZero_!(u8, 0);
    implement_IsZero_!(u16, 0);
    implement_IsZero_!(u32, 0);
    implement_IsZero_!(u64, 0);
    implement_IsZero_!(u128, 0);

    implement_IsZero_!(isize, 0);
    implement_IsZero_!(usize, 0);

    implement_IsZero_!(f32, 0.0);
    implement_IsZero_!(f64, 0.0);

    implement_IsZero_!(char, '\0');
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::Zero;


    mod TEST_CUSTOM_TYPE {
        #![allow(non_snake_case)]

        use super::*;


        #[derive(Debug)]
        struct CustomType {
            pub value : i32,
        }

        impl Zero for CustomType {
            fn zero() -> Self {
                Self {
                    value : -1,
                }
            }
        }


        #[test]
        fn TEST_Zero() {
            let ct = CustomType::zero();

            assert_eq!(-1, ct.value);
        }
    }


    #[cfg(feature = "implement-Zero-for-built_ins")]
    mod TEST_BUILTIN_TYPES {
        #![allow(non_snake_case)]
        use super::*;


        #[test]
        fn TEST_INTEGERS() {
            assert_eq!(0, i8::zero());
            assert_eq!(0, i16::zero());
            assert_eq!(0, i32::zero());
            assert_eq!(0, i64::zero());
            assert_eq!(0, i128::zero());

            assert_eq!(0, u8::zero());
            assert_eq!(0, u16::zero());
            assert_eq!(0, u32::zero());
            assert_eq!(0, u64::zero());
            assert_eq!(0, u128::zero());

            assert_eq!(0, isize::zero());
            assert_eq!(0, usize::zero());

            assert_eq!(0.0f32, f32::zero());
            assert_eq!(0.0f64, f64::zero());

            assert_eq!('\0', char::zero());
        }
    }
}

