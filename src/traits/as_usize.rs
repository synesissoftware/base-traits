// src/traits/as_usize.rs : `AsUSize`

/// Trait defining instance method `as_usize() : usize` that provides a
/// cost-free conversion into `usize`.
///
/// It is expected that the implementing type "is-a" `usize` in a direct
/// manner as well as in a logical manner.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-AsUSize-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following type(s):
/// - [`usize`];
pub trait AsUSize {
    fn as_usize(&self) -> usize;
}


#[cfg(all(not(test), not(feature = "nostd")))]
impl<T : AsUSize + ?Sized> AsUSize for Box<T> {
    fn as_usize(&self) -> usize {
        (**self).as_usize()
    }
}

#[cfg(all(not(test), not(feature = "nostd")))]
impl<T : AsUSize + ?Sized> AsUSize for std::rc::Rc<T> {
    fn as_usize(&self) -> usize {
        (**self).as_usize()
    }
}


#[cfg(feature = "implement-AsUSize-for-built_ins")]
mod impl_for_built_ins {
    #![allow(non_snake_case)]


    impl super::AsUSize for usize {
        #[inline]
        fn as_usize(&self) -> usize {
            *self
        }
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::AsUSize;

    use std::rc as std_rc;


    mod TEST_CUSTOM_TYPE {
        #![allow(non_snake_case)]

        use super::*;


        struct CustomType {
            value : u64,
        }

        impl AsUSize for CustomType {
            fn as_usize(&self) -> usize {
                self.value as usize
            }
        }


        #[test]
        fn TEST_RANGE_OF_VALUES() {

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
                u32::MAX as usize,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = CustomType { value: value as u64 };
                let actual = instance.as_usize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES_IN_Box() {

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
                u32::MAX as usize,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(CustomType { value: value as u64 });
                let actual = instance.as_usize();

                assert_eq!(expected, actual);
            }
        }
    }


    #[cfg(feature = "implement-AsUSize-for-built_ins")]
    mod TEST_BUILTIN_TYPES {
        #![allow(non_snake_case)]

        use super::*;


        #[test]
        fn TEST_RANGE_OF_VALUES() {

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
                let actual = value.as_usize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES_REF() {

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
                let actual = (&value).as_usize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES_IN_Box() {

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
                let actual = instance.as_usize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES_IN_REF_Box() {

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
                let actual = (&instance).as_usize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES_IN_Rc() {

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
                let actual = instance.as_usize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES_IN_REF_Rc() {

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
                let actual = (&instance).as_usize();

                assert_eq!(expected, actual);
            }
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //

