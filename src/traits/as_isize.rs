// src/traits/as_isize.rs : `AsISize`

/// Trait defining instance method `as_isize() : isize` that provides a
/// cost-free conversion into `isize`.
///
/// It is expected that the implementing type "is-a" `isize` in a direct
/// manner as well as in a logical manner.
pub trait AsISize {
    fn as_isize(&self) -> isize;
}


impl<T : AsISize + ?Sized> AsISize for Box<T> {
    fn as_isize(&self) -> isize {
        (**self).as_isize()
    }
}

impl<T : AsISize + ?Sized> AsISize for std::rc::Rc<T> {
    fn as_isize(&self) -> isize {
        (**self).as_isize()
    }
}


#[cfg(feature = "implement-AsISize-for-built_ins")]
mod impl_for_built_ins {
    #![allow(non_snake_case)]


    impl super::AsISize for isize {
        fn as_isize(&self) -> isize {
            *self
        }
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::AsISize;

    use std::rc as std_rc;


    mod TEST_CUSTOM_TYPE {
        #![allow(non_snake_case)]

        use super::*;


        struct CustomType {
            value : u64,
        }

        impl AsISize for CustomType {
            fn as_isize(&self) -> isize {
                self.value as isize
            }
        }


        #[test]
        fn TEST_RANGE_OF_VALUES() {

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
                u32::MAX as isize,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = CustomType { value: value as u64 };
                let actual = instance.as_isize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES_IN_Box() {

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
                u32::MAX as isize,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(CustomType { value: value as u64 });
                let actual = instance.as_isize();

                assert_eq!(expected, actual);
            }
        }
    }


    #[cfg(feature = "implement-AsISize-for-built_ins")]
    mod TEST_BUILTIN_TYPES {
        #![allow(non_snake_case)]

        use super::*;


        #[test]
        fn TEST_RANGE_OF_VALUES() {

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
                let actual = value.as_isize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES_REF() {

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
                let actual = (&value).as_isize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES_IN_Box() {

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
                let actual = instance.as_isize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES_IN_REF_Box() {

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
                let actual = (&instance).as_isize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES_IN_Rc() {

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
                let actual = instance.as_isize();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES_IN_REF_Rc() {

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
                let actual = (&instance).as_isize();

                assert_eq!(expected, actual);
            }
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //

