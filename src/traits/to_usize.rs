// src/traits/to_usize.rs : `ToUSize`

/// Trait defining instance method `to_usize() : usize` that provides a
/// no-cost or low-cost conversion into `usize`.
///
/// It is expected that the implementing type "is-a" `usize` in a logical
/// manner.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-ToUSize-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following type(s):
/// - [`usize`];
/// - [`u8`];
/// - [`u16`] - if architecture is 16+ bits;
/// - [`u32`] - if architecture is 32+ bits;
/// - [`u64`] - if architecture is 64+ bits;
/// - [`u128`] - if architecture is 128+ bits;
pub trait ToUSize {
    fn to_usize(&self) -> usize;
}


impl<T : ToUSize + ?Sized> ToUSize for Box<T> {
    fn to_usize(&self) -> usize {
        (**self).to_usize()
    }
}

impl<T : ToUSize + ?Sized> ToUSize for std::rc::Rc<T> {
    fn to_usize(&self) -> usize {
        (**self).to_usize()
    }
}


#[cfg(feature = "implement-ToUSize-for-built_ins")]
#[rustfmt::skip]
mod impl_for_built_ins {
    #![allow(non_snake_case)]
    #![allow(unexpected_cfgs)]


    impl super::ToUSize for usize {
        #[inline]
        fn to_usize(&self) -> usize {
            *self
        }
    }

    macro_rules! implement_ToUSize_ {
        ($type:tt) => {
            impl super::ToUSize for $type {
                #[inline]
                fn to_usize(&self) -> usize {
                    *self as usize
                }
            }
        };
    }

    implement_ToUSize_!(u8);

    #[cfg(any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64",
        target_pointer_width = "128",
    ))]
    implement_ToUSize_!(u16);

    #[cfg(any(
        target_pointer_width = "32",
        target_pointer_width = "64",
        target_pointer_width = "128",
    ))]
    implement_ToUSize_!(u32);

    #[cfg(any(
        target_pointer_width = "64",
        target_pointer_width = "128",
    ))]
    implement_ToUSize_!(u64);

    #[cfg(any(
        target_pointer_width = "128",
    ))]
    implement_ToUSize_!(u128);
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::ToUSize;

    use std::rc as std_rc;


    mod TEST_CUSTOM_TYPE {
        #![allow(non_snake_case)]

        use super::ToUSize;


        struct CustomType {
            value : u64,
        }

        impl ToUSize for CustomType {
            fn to_usize(&self) -> usize {
                self.value as usize
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES() {

            const VALUES : &[usize] = &[
                // insert list:
                0,
                1,
                2, 4, 8, 16, 32, 64, 128, 256,
                u32::MAX as usize,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = CustomType { value: value as u64 };
                let actual = instance.to_usize();

                assert_eq!(expected, actual);
            }
        }
    }


    #[cfg(feature = "implement-ToUSize-for-built_ins")]
    mod TEST_BUILTIN_TYPES {
        #![allow(non_snake_case)]
        #![allow(unexpected_cfgs)]

        use super::*;


        #[test]
        fn TEST_RANGE_OF_usize_VALUES() {

            const VALUES : &[usize] = &[
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
                usize::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let actual = value.to_usize();

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
                let expected = value as usize;
                let actual = (&value).to_usize();

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
                let expected = value as usize;
                let actual = (&value).to_usize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_usize_VALUES_REF() {

            const VALUES : &[usize] = &[
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
                usize::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let actual = (&value).to_usize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_usize_VALUES_IN_Box() {

            const VALUES : &[usize] = &[
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
                usize::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(value);
                let actual = instance.to_usize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_usize_VALUES_IN_REF_Box() {

            const VALUES : &[usize] = &[
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
                usize::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(value);
                let actual = (&instance).to_usize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_usize_VALUES_IN_Rc() {

            const VALUES : &[usize] = &[
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
                usize::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = std_rc::Rc::new(value);
                let actual = instance.to_usize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_usize_VALUES_IN_REF_Rc() {

            const VALUES : &[usize] = &[
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
                usize::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = std_rc::Rc::new(value);
                let actual = (&instance).to_usize();

                assert_eq!(expected, actual);
            }
        }
    }
}

// ///////////////////////////// end of file //////////////////////////// //


