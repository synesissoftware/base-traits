// src/traits/as_f64.rs : `AsF64`

/// Trait defining instance method `as_f64() : f64` that provides a
/// cost-free conversion into `f64`.
///
/// It is expected that the implementing type "is-a" `f64` in a direct
/// manner as well as in a logical manner.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-AsF64-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following type(s):
/// - [`f64`];
pub trait AsF64 {
    fn as_f64(&self) -> f64;
}


#[cfg(all(not(test), not(feature = "nostd")))]
impl<T : AsF64 + ?Sized> AsF64 for Box<T> {
    fn as_f64(&self) -> f64 {
        (**self).as_f64()
    }
}

#[cfg(all(not(test), not(feature = "nostd")))]
impl<T : AsF64 + ?Sized> AsF64 for std::rc::Rc<T> {
    fn as_f64(&self) -> f64 {
        (**self).as_f64()
    }
}


#[cfg(feature = "implement-AsF64-for-built_ins")]
mod impl_for_built_ins {
    #![allow(non_snake_case)]


    impl super::AsF64 for f64 {
        #[inline]
        fn as_f64(&self) -> f64 {
            *self
        }
    }

    impl super::AsF64 for &f64 {
        #[inline]
        fn as_f64(&self) -> f64 {
            **self
        }
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::AsF64;

    use std::rc::Rc;


    #[allow(unused)]
    fn as_AsF64<T : AsF64>(t : &T) -> &impl AsF64 {
        t
    }


    mod TEST_CUSTOM_TYPE {
        #![allow(non_snake_case)]

        use super::*;


        #[derive(Debug)]
        struct CustomType {
            value : f64,
        }

        impl AsF64 for CustomType {
            fn as_f64(&self) -> f64 {
                self.value
            }
        }


        #[test]
        fn TEST_WHEN_ZERO() {
            let ct = CustomType { value : 0.0 };

            assert_eq!(0.0, ct.as_f64());

            let ct = &ct;

            assert_eq!(0.0, ct.as_f64());
        }

        #[test]
        fn TEST_WHEN_NONZERO() {
            let ct = CustomType { value : 1.0 };

            assert_ne!(0.0, ct.as_f64());

            let ct = &ct;

            assert_ne!(0.0, ct.as_f64());
        }

        #[test]
        fn TEST_WHEN_ZERO_IN_Box() {
            {
                let ct = Box::new(CustomType { value : 0.0 });

                assert_eq!(0.0, ct.as_f64());

                let ct = &ct;

                assert_eq!(0.0, ct.as_f64());
            }

            {
                let ct = &Box::new(CustomType { value : 0.0 });

                assert_eq!(0.0, ct.as_f64());

                let ct = &ct;

                assert_eq!(0.0, ct.as_f64());
            }

            {
                let ct = Box::new(&CustomType { value : 0.0 });

                assert_eq!(0.0, ct.as_f64());

                let ct = &ct;

                assert_eq!(0.0, ct.as_f64());
            }

            {
                let ct = &Box::new(&CustomType { value : 0.0 });

                assert_eq!(0.0, ct.as_f64());

                let ct = &ct;

                assert_eq!(0.0, ct.as_f64());
            }
        }

        #[test]
        fn TEST_WHEN_ZERO_IN_Rc() {
            {
                let ct = Rc::new(CustomType { value : 0.0 });

                assert_eq!(0.0, ct.as_f64());

                let ct = &ct;

                assert_eq!(0.0, ct.as_f64());
            }

            {
                let ct = &Rc::new(CustomType { value : 0.0 });

                assert_eq!(0.0, ct.as_f64());

                let ct = &ct;

                assert_eq!(0.0, ct.as_f64());
            }

            {
                let ct = Rc::new(&CustomType { value : 0.0 });

                assert_eq!(0.0, ct.as_f64());

                let ct = &ct;

                assert_eq!(0.0, ct.as_f64());
            }

            {
                let ct = &Rc::new(&CustomType { value : 0.0 });

                assert_eq!(0.0, ct.as_f64());

                let ct = &ct;

                assert_eq!(0.0, ct.as_f64());
            }
        }
    }


    #[cfg(feature = "implement-AsF64-for-built_ins")]
    mod TEST_BUILTIN_TYPES {
        #![allow(non_snake_case)]

        use super::*;


        mod TEST_f64 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_ZERO() {
                let v = 0.0f64;

                assert_eq!(0.0, v.as_f64());

                let ie = as_AsF64(&v);

                assert_eq!(0.0, ie.as_f64());
            }

            #[test]
            fn TEST_NONZERO() {
                let v = -123.456f64;

                assert_ne!(0.0, v.as_f64());

                let ie = as_AsF64(&v);

                assert_ne!(0.0, ie.as_f64());
            }
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //

