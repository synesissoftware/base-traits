// src/traits/to_u128.rs : `ToU128`

/// Trait defining instance method `to_u128() : u128` that provides a
/// no-cost or low-cost conversion into `u128`.
///
/// It is expected that the implementing type "is-a" `u128` in a logical
/// manner.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-ToU128-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following type(s):
/// - [`u8`];
/// - [`u16`];
/// - [`u32`];
/// - [`u64`];
/// - [`u128`];
pub trait ToU128 {
    fn to_u128(&self) -> u128;
}


impl<T : ToU128 + ?Sized> ToU128 for Box<T> {
    fn to_u128(&self) -> u128 {
        (**self).to_u128()
    }
}

impl<T : ToU128 + ?Sized> ToU128 for std::rc::Rc<T> {
    fn to_u128(&self) -> u128 {
        (**self).to_u128()
    }
}


#[cfg(feature = "implement-ToU128-for-built_ins")]
#[rustfmt::skip]
mod impl_for_built_ins {
    #![allow(non_snake_case)]


    impl super::ToU128 for u128 {
        #[inline]
        fn to_u128(&self) -> u128 {
            *self
        }
    }

    macro_rules! implement_ToU128_ {
        ($type:tt) => {
            impl super::ToU128 for $type {
                #[inline]
                fn to_u128(&self) -> u128 {
                    *self as u128
                }
            }
        };
    }

    implement_ToU128_!(u8);
    implement_ToU128_!(u16);
    implement_ToU128_!(u32);
    implement_ToU128_!(u64);
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::ToU128;

    use std::rc as std_rc;


    mod TEST_CUSTOM_TYPE {
        #![allow(non_snake_case)]

        use super::ToU128;


        struct CustomType {
            value : u128,
        }

        impl ToU128 for CustomType {
            fn to_u128(&self) -> u128 {
                self.value as u128
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES() {

            const VALUES : &[u128] = &[
                // insert list:
                0,
                1,
                2, 4, 8, 16, 32, 64, 128, 256,
                u32::MAX as u128,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = CustomType { value: value as u128 };
                let actual = instance.to_u128();

                assert_eq!(expected, actual);
            }
        }
    }


    #[cfg(feature = "implement-ToU128-for-built_ins")]
    mod TEST_BUILTIN_TYPES {
        #![allow(non_snake_case)]

        use super::*;


        #[test]
        fn TEST_RANGE_OF_u128_VALUES() {

            const VALUES : &[u128] = &[
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
                u128::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let actual = value.to_u128();

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
                let expected = value as u128;
                let actual = (&value).to_u128();

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
                let expected = value as u128;
                let actual = (&value).to_u128();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u128_VALUES_REF() {

            const VALUES : &[u128] = &[
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
                u128::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let actual = (&value).to_u128();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u128_VALUES_IN_Box() {

            const VALUES : &[u128] = &[
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
                u128::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(value);
                let actual = instance.to_u128();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u128_VALUES_IN_REF_Box() {

            const VALUES : &[u128] = &[
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
                u128::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(value);
                let actual = (&instance).to_u128();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u128_VALUES_IN_Rc() {

            const VALUES : &[u128] = &[
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
                u128::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = std_rc::Rc::new(value);
                let actual = instance.to_u128();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u128_VALUES_IN_REF_Rc() {

            const VALUES : &[u128] = &[
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
                u128::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = std_rc::Rc::new(value);
                let actual = (&instance).to_u128();

                assert_eq!(expected, actual);
            }
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //

