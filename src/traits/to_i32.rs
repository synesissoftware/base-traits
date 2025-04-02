// src/traits/to_i32.rs : `ToI32`

/// Trait defining instance method `to_i32() : i32` that provides a
/// no-cost or low-cost conversion into `i32`.
///
/// It is expected that the implementing type "is-a" `i32` in a logical
/// manner.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-ToI32-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following type(s):
/// - [`i8`];
/// - [`i16`];
/// - [`i32`];
/// - [`u8`];
/// - [`u16`];
pub trait ToI32 {
    fn to_i32(&self) -> i32;
}


#[cfg(all(not(test), not(feature = "nostd")))]
impl<T : ToI32 + ?Sized> ToI32 for Box<T> {
    fn to_i32(&self) -> i32 {
        (**self).to_i32()
    }
}

#[cfg(all(not(test), not(feature = "nostd")))]
impl<T : ToI32 + ?Sized> ToI32 for std::rc::Rc<T> {
    fn to_i32(&self) -> i32 {
        (**self).to_i32()
    }
}


#[cfg(feature = "implement-ToI32-for-built_ins")]
#[rustfmt::skip]
mod impl_for_built_ins {
    #![allow(non_snake_case)]


    impl super::ToI32 for i32 {
        #[inline]
        fn to_i32(&self) -> i32 {
            *self
        }
    }

    macro_rules! implement_ToI32_ {
        ($type:tt) => {
            impl super::ToI32 for $type {
                #[inline]
                fn to_i32(&self) -> i32 {
                    *self as i32
                }
            }
        };
    }

    implement_ToI32_!(i8);
    implement_ToI32_!(u8);
    implement_ToI32_!(i16);
    implement_ToI32_!(u16);
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::ToI32;

    use std::rc as std_rc;


    mod TEST_CUSTOM_TYPE {
        #![allow(non_snake_case)]

        use super::ToI32;


        struct CustomType {
            value : u32,
        }

        impl ToI32 for CustomType {
            fn to_i32(&self) -> i32 {
                self.value as i32
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES() {

            const VALUES : &[i32] = &[
                // insert list:
                0,
                1,
                2, 4, 8, 16, 32, 32, 32, 256,
                u32::MAX as i32,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = CustomType { value: value as u32 };
                let actual = instance.to_i32();

                assert_eq!(expected, actual);
            }
        }
    }


    #[cfg(feature = "implement-ToI32-for-built_ins")]
    mod TEST_BUILTIN_TYPES {
        #![allow(non_snake_case)]

        use super::*;


        #[test]
        fn TEST_RANGE_OF_i32_VALUES() {

            const VALUES : &[i32] = &[
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
                i32::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let actual = value.to_i32();

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
                32,
                32,
                256,
                i16::MAX,
            ];

            for &value in VALUES {
                let expected = value as i32;
                let actual = (&value).to_i32();

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
                let expected = value as i32;
                let actual = (&value).to_i32();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_i32_VALUES_REF() {

            const VALUES : &[i32] = &[
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
                i32::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let actual = (&value).to_i32();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_i32_VALUES_IN_Box() {

            const VALUES : &[i32] = &[
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
                i32::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(value);
                let actual = instance.to_i32();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_i32_VALUES_IN_REF_Box() {

            const VALUES : &[i32] = &[
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
                i32::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(value);
                let actual = (&instance).to_i32();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_i32_VALUES_IN_Rc() {

            const VALUES : &[i32] = &[
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
                i32::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = std_rc::Rc::new(value);
                let actual = instance.to_i32();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_i32_VALUES_IN_REF_Rc() {

            const VALUES : &[i32] = &[
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
                i32::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = std_rc::Rc::new(value);
                let actual = (&instance).to_i32();

                assert_eq!(expected, actual);
            }
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //

