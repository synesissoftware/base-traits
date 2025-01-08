// src/traits/to_i128.rs : `ToI128`

/// Trait defining instance method `to_i128() : i128` that provides a
/// no-cost or low-cost conversion into `i128`.
///
/// It is expected that the implementing type "is-a" `i128` in a logical
/// manner.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-ToI128-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following type(s):
/// - [`i8`];
/// - [`i16`];
/// - [`i32`];
/// - [`i64`];
/// - [`i128`];
/// - [`u8`];
/// - [`u16`];
/// - [`u32`];
/// - [`u64`];
pub trait ToI128 {
    fn to_i128(&self) -> i128;
}


impl<T : ToI128 + ?Sized> ToI128 for Box<T> {
    fn to_i128(&self) -> i128 {
        (**self).to_i128()
    }
}

impl<T : ToI128 + ?Sized> ToI128 for std::rc::Rc<T> {
    fn to_i128(&self) -> i128 {
        (**self).to_i128()
    }
}


#[cfg(feature = "implement-ToI128-for-built_ins")]
#[rustfmt::skip]
mod impl_for_built_ins {
    #![allow(non_snake_case)]


    impl super::ToI128 for i128 {
        #[inline]
        fn to_i128(&self) -> i128 {
            *self
        }
    }

    macro_rules! implement_ToI128_ {
        ($type:tt) => {
            impl super::ToI128 for $type {
                #[inline]
                fn to_i128(&self) -> i128 {
                    *self as i128
                }
            }
        };
    }

    implement_ToI128_!(i8);
    implement_ToI128_!(i16);
    implement_ToI128_!(u16);
    implement_ToI128_!(i32);
    implement_ToI128_!(u32);
    implement_ToI128_!(i64);
    implement_ToI128_!(u64);
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::ToI128;

    use std::rc as std_rc;


    mod TEST_CUSTOM_TYPE {
        #![allow(non_snake_case)]

        use super::ToI128;


        struct CustomType {
            value : u64,
        }

        impl ToI128 for CustomType {
            fn to_i128(&self) -> i128 {
                self.value as i128
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES() {

            const VALUES : &[i128] = &[
                // insert list:
                0,
                1,
                2, 4, 8, 16, 32, 64, 128, 256,
                u32::MAX as i128,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = CustomType { value: value as u64 };
                let actual = instance.to_i128();

                assert_eq!(expected, actual);
            }
        }
    }


    #[cfg(feature = "implement-ToI128-for-built_ins")]
    mod TEST_BUILTIN_TYPES {
        #![allow(non_snake_case)]

        use super::*;


        #[test]
        fn TEST_RANGE_OF_i128_VALUES() {

            const VALUES : &[i128] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                32,
                64,
                128,
                256,
                i128::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let actual = value.to_i128();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_i16_VALUES_REF() {

            const VALUES : &[i16] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                32,
                64,
                128,
                256,
                i16::MAX,
            ];

            for &value in VALUES {
                let expected = value as i128;
                let actual = (&value).to_i128();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u16_VALUES_REF() {

            const VALUES : &[u16] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                32,
                64,
                128,
                256,
                u16::MAX,
            ];

            for &value in VALUES {
                let expected = value as i128;
                let actual = (&value).to_i128();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u32_VALUES_REF() {

            const VALUES : &[u32] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                32,
                64,
                128,
                256,
                u32::MAX,
            ];

            for &value in VALUES {
                let expected = value as i128;
                let actual = (&value).to_i128();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_i128_VALUES_REF() {

            const VALUES : &[i128] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                32,
                64,
                128,
                256,
                i128::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let actual = (&value).to_i128();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_i128_VALUES_IN_Box() {

            const VALUES : &[i128] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                32,
                64,
                128,
                256,
                i128::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(value);
                let actual = instance.to_i128();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_i128_VALUES_IN_REF_Box() {

            const VALUES : &[i128] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                32,
                64,
                128,
                256,
                i128::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(value);
                let actual = (&instance).to_i128();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_i128_VALUES_IN_Rc() {

            const VALUES : &[i128] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                32,
                64,
                128,
                256,
                i128::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = std_rc::Rc::new(value);
                let actual = instance.to_i128();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_i128_VALUES_IN_REF_Rc() {

            const VALUES : &[i128] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                32,
                64,
                128,
                256,
                i128::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = std_rc::Rc::new(value);
                let actual = (&instance).to_i128();

                assert_eq!(expected, actual);
            }
        }
    }
}

// ///////////////////////////// end of file //////////////////////////// //


