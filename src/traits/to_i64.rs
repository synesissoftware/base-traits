// src/traits/to_i64.rs : `ToI64`

/// Trait defining instance method `to_i64() : i64` that provides a
/// no-cost or low-cost conversion into `i64`.
///
/// It is expected that the implementing type "is-a" `i64` in a logical
/// manner.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-ToI64-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following type(s):
/// - [`i8`];
/// - [`i16`];
/// - [`i32`];
/// - [`i64`];
/// - [`u8`];
/// - [`u16`];
/// - [`u32`];
pub trait ToI64 {
    fn to_i64(&self) -> i64;
}


#[cfg(all(not(test), not(feature = "nostd")))]
impl<T : ToI64 + ?Sized> ToI64 for Box<T> {
    fn to_i64(&self) -> i64 {
        (**self).to_i64()
    }
}

#[cfg(all(not(test), not(feature = "nostd")))]
impl<T : ToI64 + ?Sized> ToI64 for std::rc::Rc<T> {
    fn to_i64(&self) -> i64 {
        (**self).to_i64()
    }
}


#[cfg(feature = "implement-ToI64-for-built_ins")]
#[rustfmt::skip]
mod impl_for_built_ins {
    #![allow(non_snake_case)]


    impl super::ToI64 for i64 {
        #[inline]
        fn to_i64(&self) -> i64 {
            *self
        }
    }

    macro_rules! implement_ToI64_ {
        ($type:tt) => {
            impl super::ToI64 for $type {
                #[inline]
                fn to_i64(&self) -> i64 {
                    *self as i64
                }
            }
        };
    }

    implement_ToI64_!(i8);
    implement_ToI64_!(u8);
    implement_ToI64_!(i16);
    implement_ToI64_!(u16);
    implement_ToI64_!(i32);
    implement_ToI64_!(u32);
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::ToI64;

    use std::rc as std_rc;


    mod TEST_CUSTOM_TYPE {
        #![allow(non_snake_case)]

        use super::ToI64;


        struct CustomType {
            value : i64,
        }

        impl ToI64 for CustomType {
            fn to_i64(&self) -> i64 {
                self.value as i64
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES() {

            const VALUES : &[i64] = &[
                // insert list:
                0,
                1,
                2, 4, 8, 16, 32, 64, 64, 256,
                u32::MAX as i64,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = CustomType { value: value as i64 };
                let actual = instance.to_i64();

                assert_eq!(expected, actual);
            }
        }
    }


    #[cfg(feature = "implement-ToI64-for-built_ins")]
    mod TEST_BUILTIN_TYPES {
        #![allow(non_snake_case)]

        use super::*;


        #[test]
        fn TEST_RANGE_OF_i64_VALUES() {

            const VALUES : &[i64] = &[
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
                i64::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let actual = value.to_i64();

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
                64,
                256,
                i16::MAX,
            ];

            for &value in VALUES {
                let expected = value as i64;
                let actual = (&value).to_i64();

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
                let expected = value as i64;
                let actual = (&value).to_i64();

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
                let expected = value as i64;
                let actual = (&value).to_i64();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_i64_VALUES_REF() {

            const VALUES : &[i64] = &[
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
                i64::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let actual = (&value).to_i64();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_i64_VALUES_IN_Box() {

            const VALUES : &[i64] = &[
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
                i64::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(value);
                let actual = instance.to_i64();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_i64_VALUES_IN_REF_Box() {

            const VALUES : &[i64] = &[
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
                i64::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(value);
                let actual = (&instance).to_i64();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_i64_VALUES_IN_Rc() {

            const VALUES : &[i64] = &[
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
                i64::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = std_rc::Rc::new(value);
                let actual = instance.to_i64();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_i64_VALUES_IN_REF_Rc() {

            const VALUES : &[i64] = &[
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
                i64::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = std_rc::Rc::new(value);
                let actual = (&instance).to_i64();

                assert_eq!(expected, actual);
            }
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //

