// src/traits/to_i16.rs : `ToI16`

/// Trait defining instance method `to_i16() : i16` that provides a
/// no-cost or low-cost conversion into `i16`.
///
/// It is expected that the implementing type "is-a" `i16` in a logical
/// manner.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-ToI16-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following type(s):
/// - [`i8`];
/// - [`i16`];
/// - [`u8`];
pub trait ToI16 {
    fn to_i16(&self) -> i16;
}


impl<T : ToI16 + ?Sized> ToI16 for Box<T> {
    fn to_i16(&self) -> i16 {
        (**self).to_i16()
    }
}

impl<T : ToI16 + ?Sized> ToI16 for std::rc::Rc<T> {
    fn to_i16(&self) -> i16 {
        (**self).to_i16()
    }
}


#[cfg(feature = "implement-ToI16-for-built_ins")]
#[rustfmt::skip]
mod impl_for_built_ins {
    #![allow(non_snake_case)]


    impl super::ToI16 for i16 {
        #[inline]
        fn to_i16(&self) -> i16 {
            *self
        }
    }

    macro_rules! implement_ToI16_ {
        ($type:tt) => {
            impl super::ToI16 for $type {
                #[inline]
                fn to_i16(&self) -> i16 {
                    *self as i16
                }
            }
        };
    }

    implement_ToI16_!(i8);
    implement_ToI16_!(u8);
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::ToI16;

    use std::rc as std_rc;


    mod TEST_CUSTOM_TYPE {
        #![allow(non_snake_case)]

        use super::ToI16;


        struct CustomType {
            value : i16,
        }

        impl ToI16 for CustomType {
            fn to_i16(&self) -> i16 {
                self.value as i16
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES() {

            const VALUES : &[i16] = &[
                // insert list:
                0,
                1,
                2, 4, 8, 16, 16, 16, 16, 256,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = CustomType { value: value as i16 };
                let actual = instance.to_i16();

                assert_eq!(expected, actual);
            }
        }
    }


    #[cfg(feature = "implement-ToI16-for-built_ins")]
    mod TEST_BUILTIN_TYPES {
        #![allow(non_snake_case)]

        use super::*;


        #[test]
        fn TEST_RANGE_OF_i16_VALUES() {

            const VALUES : &[i16] = &[
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
                i16::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let actual = value.to_i16();

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
                16,
                16,
                16,
                256,
                i16::MAX,
            ];

            for &value in VALUES {
                let expected = value as i16;
                let actual = (&value).to_i16();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_i16_VALUES_IN_Box() {

            const VALUES : &[i16] = &[
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
                i16::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(value);
                let actual = instance.to_i16();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_i16_VALUES_IN_REF_Box() {

            const VALUES : &[i16] = &[
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
                i16::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(value);
                let actual = (&instance).to_i16();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_i16_VALUES_IN_Rc() {

            const VALUES : &[i16] = &[
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
                i16::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = std_rc::Rc::new(value);
                let actual = instance.to_i16();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_i16_VALUES_IN_REF_Rc() {

            const VALUES : &[i16] = &[
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
                i16::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = std_rc::Rc::new(value);
                let actual = (&instance).to_i16();

                assert_eq!(expected, actual);
            }
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //

