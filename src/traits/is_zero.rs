// src/traits/is_zero.rs : `IsZero`

/// Trait defining instance method `is_zero() : bool` that indicates whether
/// the implementing type instance is numerically zero.
pub trait IsZero {
    fn is_zero(&self) -> bool;
}


#[cfg(feature = "implement-IsZero-for-built_ins")]
mod impl_for_built_ins {
    #![allow(non_snake_case)]

    macro_rules! implement_IsZero_ {
        ($type:tt, $zero_value:expr) => {
            impl super::IsZero for $type {
                #[inline]
                fn is_zero(&self) -> bool {
                    $zero_value == *self
                }
            }
        };
    }

    implement_IsZero_!(i8, 0);
    implement_IsZero_!(i16, 0);
    implement_IsZero_!(i32, 0);
    implement_IsZero_!(i64, 0);
    implement_IsZero_!(i128, 0);

    implement_IsZero_!(u8, 0);
    implement_IsZero_!(u16, 0);
    implement_IsZero_!(u32, 0);
    implement_IsZero_!(u64, 0);
    implement_IsZero_!(u128, 0);

    implement_IsZero_!(isize, 0);
    implement_IsZero_!(usize, 0);

    implement_IsZero_!(f32, 0.0);
    implement_IsZero_!(f64, 0.0);

    implement_IsZero_!(char, '\0');
}


#[cfg(feature = "implement-IsZero-for-standard_num_types")]
mod impl_for_std_num_types {
    #![allow(non_snake_case)]

    /*
    use std::num as std_num;
     */


    // NonZero<T>

    /*
    impl<T> super::IsZero for std_num::NonZero<T>
    {
        fn is_zero(&self) -> bool {
            false
        }
    }
     */
}


#[cfg(feature = "implement-IsZero-for-standard_process_types")]
mod impl_for_std_process_types {
    #![allow(non_snake_case)]

    use std::process as std_process;


    // ExitStatus

    impl super::IsZero for std_process::ExitStatus
    {
        fn is_zero(&self) -> bool {
            match self.code() {
                Some(ec) => {
                    0 == ec
                },
                None => false,
            }
        }
    }
}


#[cfg(feature = "implement-IsZero-for-standard_time_types")]
mod impl_for_std_time_types {
    #![allow(non_snake_case)]

    use std::time as std_time;


    mod isolate_{
        #![allow(non_snake_case)]

        use std::time as std_time;


        #[inline]
        pub(super) fn get_is_zero_Duration_(d : &std_time::Duration) -> bool {
            d.is_zero()
        }
    }


    // Duration

    impl super::IsZero for std_time::Duration {
        fn is_zero(&self) -> bool {
            isolate_::get_is_zero_Duration_(self)
        }
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::IsZero;

    use std::rc::Rc;


    #[allow(unused)]
    fn as_IsZero<T : IsZero>(t : &T) -> &impl IsZero {
        t
    }


    mod TEST_CUSTOM_TYPE {
        #![allow(non_snake_case)]

        use super::*;


        #[derive(Debug)]
        struct CustomType {
            value : i32,
        }

        impl IsZero for CustomType {
            fn is_zero(&self) -> bool {
                0 == self.value
            }
        }


        #[test]
        fn TEST_WHEN_ZERO_ELEMENTS() {
            let ct = CustomType { value : 0 };

            assert!(ct.is_zero());

            let ct = &ct;

            assert!(ct.is_zero());
        }

        #[test]
        fn TEST_WHEN_HAVE_ELEMENTS() {
            let ct = CustomType { value : 1 };

            assert!(!ct.is_zero());

            let ct = &ct;

            assert!(!ct.is_zero());
        }

        #[test]
        fn TEST_WHEN_ZERO_ELEMENTS_IN_Box() {
            {
                let ct = Box::new(CustomType { value : 0 });

                assert!(ct.is_zero());

                let ct = &ct;

                assert!(ct.is_zero());
            }

            {
                let ct = &Box::new(CustomType { value : 0 });

                assert!(ct.is_zero());

                let ct = &ct;

                assert!(ct.is_zero());
            }

            {
                let ct = Box::new(&CustomType { value : 0 });

                assert!(ct.is_zero());

                let ct = &ct;

                assert!(ct.is_zero());
            }

            {
                let ct = &Box::new(&CustomType { value : 0 });

                assert!(ct.is_zero());

                let ct = &ct;

                assert!(ct.is_zero());
            }
        }

        #[test]
        fn TEST_WHEN_ZERO_ELEMENTS_IN_Rc() {
            {
                let ct = Rc::new(CustomType { value : 0 });

                assert!(ct.is_zero());

                let ct = &ct;

                assert!(ct.is_zero());
            }

            {
                let ct = &Rc::new(CustomType { value : 0 });

                assert!(ct.is_zero());

                let ct = &ct;

                assert!(ct.is_zero());
            }

            {
                let ct = Rc::new(&CustomType { value : 0 });

                assert!(ct.is_zero());

                let ct = &ct;

                assert!(ct.is_zero());
            }

            {
                let ct = &Rc::new(&CustomType { value : 0 });

                assert!(ct.is_zero());

                let ct = &ct;

                assert!(ct.is_zero());
            }
        }
    }


    #[cfg(feature = "implement-IsZero-for-built_ins")]
    mod TEST_BUILTIN_TYPES {
        #![allow(non_snake_case)]

        use super::*;


        mod TEST_char {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_ZERO() {

                {
                    let c = '\0';

                    assert!(c.is_zero());

                    let ie = as_IsZero(&c);

                    assert!(ie.is_zero());
                }

                {
                    let c = '\u{0000}';

                    assert!(c.is_zero());

                    let ie = as_IsZero(&c);

                    assert!(ie.is_zero());
                }
            }

            #[test]
            fn TEST_NONZERO() {
                let c = 'a';

                assert!(!c.is_zero());

                let ie = as_IsZero(&c);

                assert!(!ie.is_zero());
            }
        }


        mod TEST_f64 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_ZERO() {
                let v = 0.0f64;

                assert!(v.is_zero());

                let ie = as_IsZero(&v);

                assert!(ie.is_zero());
            }

            #[test]
            fn TEST_NONZERO() {
                let v = -123.456f64;

                assert!(!v.is_zero());

                let ie = as_IsZero(&v);

                assert!(!ie.is_zero());
            }
        }


        mod TEST_i32 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_ZERO() {
                let v = 0i32;

                assert!(v.is_zero());

                let ie = as_IsZero(&v);

                assert!(ie.is_zero());
            }

            #[test]
            fn TEST_NONZERO() {
                let v = 123i32;

                assert!(!v.is_zero());

                let ie = as_IsZero(&v);

                assert!(!ie.is_zero());
            }
        }


        mod TEST_usize {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_ZERO() {
                let v = 0usize;

                assert!(v.is_zero());

                let ie = as_IsZero(&v);

                assert!(ie.is_zero());
            }

            #[test]
            fn TEST_NONZERO() {
                let v = usize::MAX;

                assert!(!v.is_zero());

                let ie = as_IsZero(&v);

                assert!(!ie.is_zero());
            }
        }
    }


    #[cfg(feature = "implement-IsZero-for-standard_process_types")]
    mod TEST_PROCESS_TYPES {
        #![allow(non_snake_case)]

    }


    #[cfg(feature = "implement-IsZero-for-standard_time_types")]
    mod TEST_TIME_TYPES {
        #![allow(non_snake_case)]

        use super::*;

        use std::time::{
            Duration,
        };


        mod TEST_Duration {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_ZERO() {
                let d = Duration::from_micros(0);

                assert!(d.is_zero());

                let ie = as_IsZero(&d);

                assert!(ie.is_zero());
            }

            #[test]
            fn TEST_NONZERO() {
                let d = Duration::from_micros(1);

                assert!(!d.is_zero());

                let ie = as_IsZero(&d);

                assert!(!ie.is_zero());
            }
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //

