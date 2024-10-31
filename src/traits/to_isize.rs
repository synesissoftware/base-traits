// src/traits/to_isize.rs : `ToISize`

/// Trait defining instance method `to_isize() : isize` that provides a
/// potentially expensive conversion into `isize`.
///
/// It is expected that the implementing type "is-a" `isize` in a logical
/// manner.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-ToISize-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following type(s):
/// - [`isize`];
/// - [`i8`];
/// - [`i16`] - if architecture is 16+ bits;
/// - [`i32`] - if architecture is 32+ bits;
/// - [`i64`] - if architecture is 64+ bits;
/// - [`i128`] - if architecture is 128+ bits;
/// - [`u16`] - if architecture is 32+ bits;
/// - [`u32`] - if architecture is 64+ bits;
/// - [`u64`] - if architecture is 128+ bits;
pub trait ToISize {
    fn to_isize(&self) -> isize;
}


impl<T : ToISize + ?Sized> ToISize for Box<T> {
    fn to_isize(&self) -> isize {
        (**self).to_isize()
    }
}

impl<T : ToISize + ?Sized> ToISize for std::rc::Rc<T> {
    fn to_isize(&self) -> isize {
        (**self).to_isize()
    }
}


#[cfg(feature = "implement-ToISize-for-built_ins")]
#[rustfmt::skip]
mod impl_for_built_ins {
    #![allow(non_snake_case)]
    #![allow(unexpected_cfgs)]

    use super::ToISize;


    impl super::ToISize for isize {
        #[inline]
        fn to_isize(&self) -> isize {
            *self
        }
    }

    macro_rules! implement_ToISize_ {
        ($type:tt) => {
            impl super::ToISize for $type {
                #[inline]
                fn to_isize(&self) -> isize {
                    *self as isize
                }
            }
        };
    }

    implement_ToISize_!(u8);

    #[cfg(any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64",
        target_pointer_width = "128",
    ))]
    mod bits_16plus {
        implement_ToISize_!(i8);
        implement_ToISize_!(u16);
    }

    #[cfg(any(
        target_pointer_width = "32",
        target_pointer_width = "64",
        target_pointer_width = "128",
    ))]
    mod bits_32plus {
        implement_ToISize_!(i16);
        implement_ToISize_!(us32);
    }

    #[cfg(any(
        target_pointer_width = "64",
        target_pointer_width = "128",
    ))]
    mod bits_64plus {
        implement_ToISize_!(i32);
        implement_ToISize_!(u64);
    }

    #[cfg(any(
        target_pointer_width = "128",
    ))]
    mod bits_128plus {
        implement_ToISize_!(i64);
        implement_ToISize_!(u128);
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::ToISize;

    use std::rc as std_rc;


    mod TEST_CUSTOM_TYPE {
        #![allow(non_snake_case)]

        use super::ToISize;


        struct CustomType {
            value : u64,
        }

        impl ToISize for CustomType {
            fn to_isize(&self) -> isize {
                self.value as isize
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES() {

            const VALUES : &[isize] = &[
                // insert list:
                0,
                1,
                2, 4, 8, 16, 32, 64, 128, 256,
                u32::MAX as isize,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = CustomType { value: value as u64 };
                let actual = instance.to_isize();

                assert_eq!(expected, actual);
            }
        }
    }


    #[cfg(feature = "implement-ToISize-for-built_ins")]
    mod TEST_BUILTIN_TYPES {
        #![allow(non_snake_case)]
        #![allow(unexpected_cfgs)]

        use super::*;


        #[test]
        fn TEST_RANGE_OF_isize_VALUES() {

            const VALUES : &[isize] = &[
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
                isize::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let actual = value.to_isize();

                assert_eq!(expected, actual);
            }
        }

        #[cfg(any(
            target_pointer_width = "32",
            target_pointer_width = "64",
            target_pointer_width = "128",
        ))]
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
                let expected = value as isize;
                let actual = (&value).to_isize();

                assert_eq!(expected, actual);
            }
        }

        #[cfg(any(
            target_pointer_width = "16",
            target_pointer_width = "32",
            target_pointer_width = "64",
            target_pointer_width = "128",
        ))]
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
                let expected = value as isize;
                let actual = (&value).to_isize();

                assert_eq!(expected, actual);
            }
        }

        #[cfg(any(
            target_pointer_width = "32",
            target_pointer_width = "64",
            target_pointer_width = "128",
        ))]
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
                let expected = value as isize;
                let actual = (&value).to_isize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_isize_VALUES_REF() {

            const VALUES : &[isize] = &[
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
                isize::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let actual = (&value).to_isize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_isize_VALUES_IN_Box() {

            const VALUES : &[isize] = &[
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
                isize::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(value);
                let actual = instance.to_isize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_isize_VALUES_IN_REF_Box() {

            const VALUES : &[isize] = &[
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
                isize::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(value);
                let actual = (&instance).to_isize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_isize_VALUES_IN_Rc() {

            const VALUES : &[isize] = &[
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
                isize::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = std_rc::Rc::new(value);
                let actual = instance.to_isize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_isize_VALUES_IN_REF_Rc() {

            const VALUES : &[isize] = &[
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
                isize::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = std_rc::Rc::new(value);
                let actual = (&instance).to_isize();

                assert_eq!(expected, actual);
            }
        }
    }
}

// ///////////////////////////// end of file //////////////////////////// //


