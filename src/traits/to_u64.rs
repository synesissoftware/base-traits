// src/traits/to_u64.rs : `ToU64`

/// Trait defining instance method `to_u64() : u64` that provides a
/// no-cost or low-cost conversion into `u64`.
///
/// It is expected that the implementing type "is-a" `u64` in a logical
/// manner.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-ToU64-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following type(s):
/// - [`u8`];
/// - [`u16`];
/// - [`u32`];
/// - [`u64`];
///
/// ## Standard Thread Types
///
/// If the feature `"implement-ToU64-for-ThreadId"`
/// is defined (as it is by `"full"`), then this is also implemented
/// for the following type(s):
/// - [`std::thread::ThreadId`];
pub trait ToU64 {
    fn to_u64(&self) -> u64;
}


#[cfg(any(test, not(feature = "nostd")))]
impl<T : ToU64 + ?Sized> ToU64 for Box<T> {
    fn to_u64(&self) -> u64 {
        (**self).to_u64()
    }
}

#[cfg(any(test, not(feature = "nostd")))]
impl<T : ToU64 + ?Sized> ToU64 for std::rc::Rc<T> {
    fn to_u64(&self) -> u64 {
        (**self).to_u64()
    }
}


#[cfg(feature = "implement-ToU64-for-built_ins")]
#[rustfmt::skip]
mod impl_for_built_ins {
    #![allow(non_snake_case)]


    impl super::ToU64 for u64 {
        #[inline]
        fn to_u64(&self) -> u64 {
            *self
        }
    }

    macro_rules! implement_ToU64_ {
        ($type:tt) => {
            impl super::ToU64 for $type {
                #[inline]
                fn to_u64(&self) -> u64 {
                    *self as u64
                }
            }
        };
    }

    implement_ToU64_!(u8);
    implement_ToU64_!(u16);
    implement_ToU64_!(u32);
}


#[cfg(all(feature = "implement-ToU64-for-ThreadId", not(feature = "nostd")))]
#[rustfmt::skip]
mod impl_for_ThreadId {
    #![allow(non_snake_case)]

    use std::thread as std_thread;


    mod isolate_ {
        #![allow(non_snake_case)]

        use std::thread as std_thread;


        pub(crate) fn threadid_u64_or_0_(tid : &std_thread::ThreadId) -> u64 {

            let mut r = 0;

            #[cfg(all(sis_rustc_1_76_or_newer, sis_rustc_nightly))]
            {
                if r == 0 {

                    r = tid.as_u64().get();
                }
            }

            if r == 0 {
                let s = format!("{:?}", tid);
                let s = s.trim();
                let s = s.strip_prefix("ThreadId(").and_then(|rem| rem.strip_suffix(")"));
                let s = s.unwrap_or("0");

                r = s.parse().unwrap_or(0);
            }

            r
        }
    }


    impl super::ToU64 for std_thread::ThreadId {
        fn to_u64(&self) -> u64 {

            isolate_::threadid_u64_or_0_(self)
        }
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::ToU64;

    #[allow(unused_imports)]
    use std::rc as std_rc;


    mod TEST_CUSTOM_TYPE {
        #![allow(clippy::redundant_field_names)]
        #![allow(non_snake_case)]

        use super::ToU64;


        struct CustomType {
            value : u64,
        }

        impl ToU64 for CustomType {
            fn to_u64(&self) -> u64 {
                self.value
            }
        }

        #[test]
        fn TEST_RANGE_OF_VALUES() {
            const VALUES : &[u64] = &[
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
                u16::MAX as u64,
                u32::MAX as u64,
                u64::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = CustomType {
                    value,
                };
                let actual = instance.to_u64();

                assert_eq!(expected, actual);
            }
        }
    }


    #[cfg(feature = "implement-ToU64-for-built_ins")]
    mod TEST_BUILTIN_TYPES {
        #![allow(non_snake_case)]

        use super::*;


        #[test]
        fn TEST_RANGE_OF_u64_VALUES() {
            const VALUES : &[u64] = &[
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
                u64::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let actual = value.to_u64();

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
                let expected = value as u64;
                let value = &value;
                let actual = value.to_u64();

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
                let expected = value as u64;
                let value = &value;
                let actual = value.to_u64();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u64_VALUES_REF() {
            const VALUES : &[u64] = &[
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
                u64::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let value = &value;
                let actual = value.to_u64();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u64_VALUES_IN_Box() {
            const VALUES : &[u64] = &[
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
                u64::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = Box::new(value);
                let actual = instance.to_u64();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u64_VALUES_IN_REF_Box() {
            const VALUES : &[u64] = &[
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
                u64::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = &Box::new(value);
                let actual = instance.to_u64();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u64_VALUES_IN_Rc() {
            const VALUES : &[u64] = &[
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
                u64::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = std_rc::Rc::new(value);
                let actual = instance.to_u64();

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn TEST_RANGE_OF_u64_VALUES_IN_REF_Rc() {
            const VALUES : &[u64] = &[
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
                u64::MAX,
            ];

            for &value in VALUES {
                let expected = value;
                let instance = &std_rc::Rc::new(value);
                let actual = instance.to_u64();

                assert_eq!(expected, actual);
            }
        }
    }


    #[cfg(all(feature = "implement-ToU64-for-ThreadId", not(feature = "nostd")))]
    mod TEST_ThreadId {
        #![allow(non_snake_case)]

        use super::*;

        use std::thread;


        fn thread_id_u64_(id : &thread::ThreadId) -> u64 {
            id.to_u64()
        }

        #[test]
        fn TEST_CURRENT_THREAD() {
            let id = thread::current().id();
            let actual = thread_id_u64_(&id);

            assert_ne!(0, actual);
            assert_eq!(actual, id.to_u64());
        }

        #[test]
        fn TEST_CURRENT_THREAD_REF() {
            let id = thread::current().id();
            let expected = thread_id_u64_(&id);
            let id = &id;
            let actual = id.to_u64();

            assert_eq!(expected, actual);
        }

        #[test]
        fn TEST_CURRENT_THREAD_IN_Box() {
            let id = thread::current().id();
            let expected = thread_id_u64_(&id);
            let instance = Box::new(id);
            let actual = instance.to_u64();

            assert_eq!(expected, actual);
        }

        #[test]
        fn TEST_CURRENT_THREAD_IN_REF_Box() {
            let id = thread::current().id();
            let expected = thread_id_u64_(&id);
            let instance = &Box::new(id);
            let actual = instance.to_u64();

            assert_eq!(expected, actual);
        }

        #[test]
        fn TEST_CURRENT_THREAD_IN_Rc() {
            let id = thread::current().id();
            let expected = thread_id_u64_(&id);
            let instance = std_rc::Rc::new(id);
            let actual = instance.to_u64();

            assert_eq!(expected, actual);
        }

        #[test]
        fn TEST_CURRENT_THREAD_IN_REF_Rc() {
            let id = thread::current().id();
            let expected = thread_id_u64_(&id);
            let instance = &std_rc::Rc::new(id);
            let actual = instance.to_u64();

            assert_eq!(expected, actual);
        }

        #[test]
        fn TEST_DISTINCT_THREADS() {
            let main_id = thread::current().id();
            let child_id = thread::spawn(|| thread::current().id()).join().unwrap();

            assert_ne!(main_id.to_u64(), child_id.to_u64());
            assert_ne!(0, main_id.to_u64());
            assert_ne!(0, child_id.to_u64());
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //
