// src/traits/to_u32.rs : `ToU32`

/// Trait defining instance method `to_u32() : u32` that provides a
/// no-cost or low-cost conversion into `u32`.
///
/// It is expected that the implementing type "is-a" `u32` in a logical
/// manner.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-ToU32-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following type(s):
/// - [`u8`];
/// - [`u16`];
/// - [`u32`];
pub trait ToU32 {
    fn to_u32(&self) -> u32;
}


impl<T : ToU32 + ?Sized> ToU32 for Box<T> {
    fn to_u32(&self) -> u32 {
        (**self).to_u32()
    }
}

impl<T : ToU32 + ?Sized> ToU32 for std::rc::Rc<T> {
    fn to_u32(&self) -> u32 {
        (**self).to_u32()
    }
}


#[cfg(feature = "implement-ToU32-for-built_ins")]
#[rustfmt::skip]
mod impl_for_built_ins {
    #![allow(non_snake_case)]


    impl super::ToU32 for u32 {
        #[inline]
        fn to_u32(&self) -> u32 {
            *self
        }
    }

    macro_rules! implement_ToU32_ {
        ($type:tt) => {
            impl super::ToU32 for $type {
                #[inline]
                fn to_u32(&self) -> u32 {
                    *self as u32
                }
            }
        };
    }

    implement_ToU32_!(u8);
    implement_ToU32_!(u16);
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::ToU32;

    use std::rc as std_rc;


    mod TEST_CUSTOM_TYPE {
        #![allow(non_snake_case)]

        use super::ToU32;


        struct CustomType {
            value : u32,
        }

        impl ToU32 for CustomType {
            fn to_u32(&self) -> u32 {
                self.value as u32
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES() {

            const VALUES : &[u32] = &[
                // insert list:
                0,
                1,
                2, 4, 8, 16, 32, 32, 32, 256,
                u32::MAX as u32,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = CustomType { value: value as u32 };
                let actual = instance.to_u32();

                assert_eq!(expected, actual);
            }
        }
    }


    #[cfg(feature = "implement-ToU32-for-built_ins")]
    mod TEST_BUILTIN_TYPES {
        #![allow(non_snake_case)]

        use super::*;


        #[test]
        fn TEST_RANGE_OF_u32_VALUES() {

            const VALUES : &[u32] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                32,
                32,
                32,
                256,
                u32::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let actual = value.to_u32();

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
                32,
                32,
                256,
                u16::MAX,
            ];

            for &value in VALUES {
                let expected = value as u32;
                let actual = (&value).to_u32();

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
                32,
                32,
                256,
                u32::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let actual = (&value).to_u32();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u32_VALUES_IN_Box() {

            const VALUES : &[u32] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                32,
                32,
                32,
                256,
                u32::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(value);
                let actual = instance.to_u32();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u32_VALUES_IN_REF_Box() {

            const VALUES : &[u32] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                32,
                32,
                32,
                256,
                u32::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(value);
                let actual = (&instance).to_u32();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u32_VALUES_IN_Rc() {

            const VALUES : &[u32] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                32,
                32,
                32,
                256,
                u32::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = std_rc::Rc::new(value);
                let actual = instance.to_u32();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u32_VALUES_IN_REF_Rc() {

            const VALUES : &[u32] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                32,
                32,
                32,
                256,
                u32::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = std_rc::Rc::new(value);
                let actual = (&instance).to_u32();

                assert_eq!(expected, actual);
            }
        }
    }
}

// ///////////////////////////// end of file //////////////////////////// //


