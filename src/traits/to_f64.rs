// src/traits/to_f64.rs : `ToF64`

/// Trait defining instance method `to_f64() : f64` that provides a
/// no-cost or low-cost conversion into `f64`.
///
/// It is expected that the implementing type "is-a" `f64` in a logical
/// manner.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-ToF64-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following type(s):
/// - [`i8`];
/// - [`i16`];
/// - [`i32`];
/// - [`i64`];
/// - [`i128`];
/// - [`u8`];
/// - [`u16`];
/// - [`u32`];
/// - [`u64`];
/// - [`u128`];
/// - [`isize`];
/// - [`usize`];
/// - [`f32`];
/// - [`f64`];
pub trait ToF64 {
    fn to_f64(&self) -> f64;
}


#[cfg(all(not(test), not(feature = "nostd")))]
impl<T : ToF64 + ?Sized> ToF64 for Box<T> {
    fn to_f64(&self) -> f64 {
        (**self).to_f64()
    }
}

#[cfg(all(not(test), not(feature = "nostd")))]
impl<T : ToF64 + ?Sized> ToF64 for std::rc::Rc<T> {
    fn to_f64(&self) -> f64 {
        (**self).to_f64()
    }
}


#[cfg(feature = "implement-ToF64-for-built_ins")]
mod impl_for_built_ins {
    #![allow(non_snake_case)]

    macro_rules! implement_ToF64_ {
        ($type:tt) => {
            impl super::ToF64 for $type {
                #[inline]
                fn to_f64(&self) -> f64 {
                    *self as f64
                }
            }
        };
    }

    implement_ToF64_!(i8);
    implement_ToF64_!(i16);
    implement_ToF64_!(i32);
    implement_ToF64_!(i64);
    implement_ToF64_!(i128);

    implement_ToF64_!(u8);
    implement_ToF64_!(u16);
    implement_ToF64_!(u32);
    implement_ToF64_!(u64);
    implement_ToF64_!(u128);

    implement_ToF64_!(isize);
    implement_ToF64_!(usize);

    implement_ToF64_!(f32);
    implement_ToF64_!(f64);
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::ToF64;

    use std::rc::Rc;


    #[allow(unused)]
    fn as_ToF64<T : ToF64>(t : &T) -> &impl ToF64 {
        t
    }


    mod TEST_CUSTOM_TYPE {
        #![allow(non_snake_case)]

        use super::*;


        #[derive(Debug)]
        struct CustomType {
            value : String,
        }

        impl ToF64 for CustomType {
            fn to_f64(&self) -> f64 {
                self.value.parse::<f64>().unwrap()
            }
        }


        #[test]
        fn TEST_WHEN_ZERO() {
            let ct = CustomType { value : "0.0".into() };

            assert_eq!(0.0, ct.to_f64());

            let ct = &ct;

            assert_eq!(0.0, ct.to_f64());
        }

        #[test]
        fn TEST_WHEN_NONZERO() {
            let ct = CustomType { value : "1.0".into() };

            assert_ne!(0.0, ct.to_f64());

            let ct = &ct;

            assert_ne!(0.0, ct.to_f64());
        }

        #[test]
        fn TEST_WHEN_ZERO_IN_Box() {
            {
                let v = CustomType { value : "0.0".into() };
                let ct = Box::new(v);

                assert_eq!(0.0, ct.to_f64());

                let ct = &ct;

                assert_eq!(0.0, ct.to_f64());
            }

            {
                let v = CustomType { value : "0.0".into() };
                let ct = &Box::new(v);

                assert_eq!(0.0, ct.to_f64());

                let ct = &ct;

                assert_eq!(0.0, ct.to_f64());
            }

            {
                let v = CustomType { value : "0.0".into() };
                let ct = Box::new(&v);

                assert_eq!(0.0, ct.to_f64());

                let ct = &ct;

                assert_eq!(0.0, ct.to_f64());
            }

            {
                let v = CustomType { value : "0.0".into() };
                let ct = &Box::new(&v);

                assert_eq!(0.0, ct.to_f64());

                let ct = &ct;

                assert_eq!(0.0, ct.to_f64());
            }
        }

        #[test]
        fn TEST_WHEN_ZERO_IN_Rc() {
            {
                let ct = Rc::new(CustomType { value : "0.0".into() });

                assert_eq!(0.0, ct.to_f64());

                let ct = &ct;

                assert_eq!(0.0, ct.to_f64());
            }

            {
                let ct = &Rc::new(CustomType { value : "0.0".into() });

                assert_eq!(0.0, ct.to_f64());

                let ct = &ct;

                assert_eq!(0.0, ct.to_f64());
            }

            {
                let v = CustomType { value : "0.0".into() };
                let ct = Rc::new(&v);

                assert_eq!(0.0, ct.to_f64());

                let ct = &ct;

                assert_eq!(0.0, ct.to_f64());
            }

            {
                let v = CustomType { value : "0.0".into() };
                let ct = &Rc::new(&v);

                assert_eq!(0.0, ct.to_f64());

                let ct = &ct;

                assert_eq!(0.0, ct.to_f64());
            }
        }
    }


    #[cfg(feature = "implement-ToF64-for-built_ins")]
    mod TEST_BUILTIN_TYPES {
        #![allow(non_snake_case)]

        use super::*;


        mod TEST_i8 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_ZERO() {
                let v = 0i8;

                assert_eq!(0.0, v.to_f64());

                let ie = as_ToF64(&v);

                assert_eq!(0.0, ie.to_f64());
            }

            #[test]
            fn TEST_NONZERO() {
                let v = -123i8;

                assert_ne!(0.0, v.to_f64());

                let ie = as_ToF64(&v);

                assert_ne!(0.0, ie.to_f64());
            }
        }


        mod TEST_f32 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_ZERO() {
                let v = 0.0f32;

                assert_eq!(0.0, v.to_f64());

                let ie = as_ToF64(&v);

                assert_eq!(0.0, ie.to_f64());
            }

            #[test]
            fn TEST_NONZERO() {
                let v = -123.456f32;

                assert_ne!(0.0, v.to_f64());

                let ie = as_ToF64(&v);

                assert_ne!(0.0, ie.to_f64());
            }
        }


        mod TEST_f64 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_ZERO() {
                let v = 0.0f64;

                assert_eq!(0.0, v.to_f64());

                let ie = as_ToF64(&v);

                assert_eq!(0.0, ie.to_f64());
            }

            #[test]
            fn TEST_NONZERO() {
                let v = -123.456f64;

                assert_ne!(0.0, v.to_f64());

                let ie = as_ToF64(&v);

                assert_ne!(0.0, ie.to_f64());
            }

            #[test]
            fn TEST_ZERO_IN_Box() {
                let v = Box::new(0.0f64);

                assert_eq!(0.0, v.to_f64());

                let ie = as_ToF64(&v);

                assert_eq!(0.0, ie.to_f64());
            }

            #[test]
            fn TEST_ZERO_IN_Box_ref() {
                let v = &Box::new(0.0f64);

                assert_eq!(0.0, v.to_f64());

                let ie = as_ToF64(v);

                assert_eq!(0.0, ie.to_f64());
            }
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //

