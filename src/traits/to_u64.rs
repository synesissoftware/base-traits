// src/traits/to_u64.rs : `ToU64`

/// Trait defining instance method `to_u64() : u64` that provides a
/// no-cost or low-cost conversion into `u64`.
///
/// It is expected that the implementing type "is-a" `u64` in a logical
/// manner.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-ToU64-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following type(s):
/// - [`u8`];
/// - [`u16`];
/// - [`u32`];
/// - [`u64`];
pub trait ToU64 {
    fn to_u64(&self) -> u64;
}


impl<T : ToU64 + ?Sized> ToU64 for Box<T> {
    fn to_u64(&self) -> u64 {
        (**self).to_u64()
    }
}

impl<T : ToU64 + ?Sized> ToU64 for std::rc::Rc<T> {
    fn to_u64(&self) -> u64 {
        (**self).to_u64()
    }
}


#[cfg(feature = "implement-ToU64-for-built_ins")]
#[rustfmt::skip]
mod impl_for_built_ins {
    #![allow(non_snake_case)]


    impl super::ToU64 for u64 {
        #[inline]
        fn to_u64(&self) -> u64 {
            *self
        }
    }

    macro_rules! implement_ToU64_ {
        ($type:tt) => {
            impl super::ToU64 for $type {
                #[inline]
                fn to_u64(&self) -> u64 {
                    *self as u64
                }
            }
        };
    }

    implement_ToU64_!(u8);
    implement_ToU64_!(u16);
    implement_ToU64_!(u32);
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::ToU64;

    use std::rc as std_rc;


    mod TEST_CUSTOM_TYPE {
        #![allow(non_snake_case)]

        use super::ToU64;


        struct CustomType {
            value : u64,
        }

        impl ToU64 for CustomType {
            fn to_u64(&self) -> u64 {
                self.value as u64
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES() {

            const VALUES : &[u64] = &[
                // insert list:
                0,
                1,
                2, 4, 8, 16, 32, 64, 64, 256,
                u16::MAX as u64,
                u32::MAX as u64,
                u64::MAX as u64,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = CustomType { value: value as u64 };
                let actual = instance.to_u64();

                assert_eq!(expected, actual);
            }
        }
    }


    #[cfg(feature = "implement-ToU64-for-built_ins")]
    mod TEST_BUILTIN_TYPES {
        #![allow(non_snake_case)]

        use super::*;


        #[test]
        fn TEST_RANGE_OF_u64_VALUES() {

            const VALUES : &[u64] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                32,
                64,
                64,
                256,
                u64::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let actual = value.to_u64();

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
                64,
                256,
                u16::MAX,
            ];

            for &value in VALUES {
                let expected = value as u64;
                let actual = (&value).to_u64();

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
                64,
                256,
                u32::MAX,
            ];

            for &value in VALUES {
                let expected = value as u64;
                let actual = (&value).to_u64();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u64_VALUES_REF() {

            const VALUES : &[u64] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                32,
                64,
                64,
                256,
                u64::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let actual = (&value).to_u64();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u64_VALUES_IN_Box() {

            const VALUES : &[u64] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                32,
                64,
                64,
                256,
                u64::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(value);
                let actual = instance.to_u64();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u64_VALUES_IN_REF_Box() {

            const VALUES : &[u64] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                32,
                64,
                64,
                256,
                u64::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(value);
                let actual = (&instance).to_u64();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u64_VALUES_IN_Rc() {

            const VALUES : &[u64] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                32,
                64,
                64,
                256,
                u64::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = std_rc::Rc::new(value);
                let actual = instance.to_u64();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u64_VALUES_IN_REF_Rc() {

            const VALUES : &[u64] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                32,
                64,
                64,
                256,
                u64::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = std_rc::Rc::new(value);
                let actual = (&instance).to_u64();

                assert_eq!(expected, actual);
            }
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //

