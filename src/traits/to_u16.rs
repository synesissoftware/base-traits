// src/traits/to_u16.rs : `ToU16`

/// Trait defining instance method `to_u16() : u16` that provides a
/// no-cost or low-cost conversion into `u16`.
///
/// It is expected that the implementing type "is-a" `u16` in a logical
/// manner.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-ToU16-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following type(s):
/// - [`u8`];
/// - [`u16`];
pub trait ToU16 {
    fn to_u16(&self) -> u16;
}


#[cfg(all(not(test), not(feature = "nostd")))]
impl<T : ToU16 + ?Sized> ToU16 for Box<T> {
    fn to_u16(&self) -> u16 {
        (**self).to_u16()
    }
}

#[cfg(all(not(test), not(feature = "nostd")))]
impl<T : ToU16 + ?Sized> ToU16 for std::rc::Rc<T> {
    fn to_u16(&self) -> u16 {
        (**self).to_u16()
    }
}


#[cfg(feature = "implement-ToU16-for-built_ins")]
#[rustfmt::skip]
mod impl_for_built_ins {
    #![allow(non_snake_case)]


    impl super::ToU16 for u16 {
        #[inline]
        fn to_u16(&self) -> u16 {
            *self
        }
    }

    macro_rules! implement_ToU16_ {
        ($type:tt) => {
            impl super::ToU16 for $type {
                #[inline]
                fn to_u16(&self) -> u16 {
                    *self as u16
                }
            }
        };
    }

    implement_ToU16_!(u8);
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::ToU16;

    use std::rc as std_rc;


    mod TEST_CUSTOM_TYPE {
        #![allow(non_snake_case)]

        use super::ToU16;


        struct CustomType {
            value : u16,
        }

        impl ToU16 for CustomType {
            fn to_u16(&self) -> u16 {
                self.value as u16
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES() {

            const VALUES : &[u16] = &[
                // insert list:
                0,
                1,
                2, 4, 8, 16, 16, 16, 16, 256,
                u16::MAX as u16,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = CustomType { value: value as u16 };
                let actual = instance.to_u16();

                assert_eq!(expected, actual);
            }
        }
    }


    #[cfg(feature = "implement-ToU16-for-built_ins")]
    mod TEST_BUILTIN_TYPES {
        #![allow(non_snake_case)]

        use super::*;


        #[test]
        fn TEST_RANGE_OF_u16_VALUES() {

            const VALUES : &[u16] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                16,
                16,
                16,
                256,
                u16::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let actual = value.to_u16();

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
                16,
                16,
                16,
                256,
                u16::MAX,
            ];

            for &value in VALUES {
                let expected = value as u16;
                let actual = (&value).to_u16();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u8_VALUES_REF() {

            const VALUES : &[u8] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                16,
                16,
                16,
                u8::MAX,
            ];

            for &value in VALUES {
                let expected = value as u16;
                let actual = (&value).to_u16();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u16_VALUES_IN_Box() {

            const VALUES : &[u16] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                16,
                16,
                16,
                256,
                u16::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(value);
                let actual = instance.to_u16();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u16_VALUES_IN_REF_Box() {

            const VALUES : &[u16] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                16,
                16,
                16,
                256,
                u16::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(value);
                let actual = (&instance).to_u16();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u16_VALUES_IN_Rc() {

            const VALUES : &[u16] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                16,
                16,
                16,
                256,
                u16::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = std_rc::Rc::new(value);
                let actual = instance.to_u16();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u16_VALUES_IN_REF_Rc() {

            const VALUES : &[u16] = &[
                // insert list:
                0,
                1,
                2,
                4,
                8,
                16,
                16,
                16,
                16,
                256,
                u16::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = std_rc::Rc::new(value);
                let actual = (&instance).to_u16();

                assert_eq!(expected, actual);
            }
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //

