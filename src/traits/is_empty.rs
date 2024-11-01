// src/traits/is_empty.rs : `IsEmpty`

/// Trait defining instance method `is_empty() : bool` that indicates
/// whether the implementing type instance is logically empty.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-IsEmpty-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following types:
/// - [`str`];
/// - `[T; N]`;
/// - `[T]`;
///
/// ## Standard Collection Types
///
/// If the feature `"implement-IsEmpty-for-standard_collection_types"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following types:
/// - [`std::collections::BTreeMap`];
/// - [`std::collections::BTreeSet`];
/// - [`std::collections::BinaryHeap`];
/// - [`std::collections::HashMap`];
/// - [`std::collections::HashSet`];
/// - [`std::collections::LinkedList`];
/// - [`String`];
/// - [`Vec`];
/// - [`std::collections::VecDeque`];
///
/// ## Standard FFI Types
///
/// If the feature `"implement-IsEmpty-for-standard_ffi_types"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following types:
/// - [`std::ffi::CStr`];
/// - [`std::ffi::CString`];
///
/// ## Standard Path Types
///
/// If the feature `"implement-IsEmpty-for-standard_path_types"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following types:
/// - [`std::path::Path`];
/// - [`std::path::PathBuf`];
///
/// ## Standard Process Types
///
/// If the feature `"implement-IsEmpty-for-standard_process_types"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following types:
/// - [`std::process::CommandArgs`];
/// - [`std::process::CommandEnvs`];
///
/// ## Standard Range Types
///
/// If the feature `"implement-IsEmpty-for-standard_range_types"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following types:
/// - [`std::ops::Range`];
/// - [`std::ops::RangeFrom`];
/// - [`std::ops::RangeFull`];
/// - [`std::ops::RangeInclusive`];
/// - [`std::ops::RangeTo`];
///
/// ## Standard Time Types
///
/// If the feature `"implement-IsEmpty-for-standard_time_types"`
/// is defined (which is NOT by `"default"`), then this is also implemented
/// for the following types:
/// - [`std::time::Duration`];
pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}


impl<T : IsEmpty + ?Sized> IsEmpty for Box<T> {
    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }
}

impl<T : IsEmpty + ?Sized> IsEmpty for std::rc::Rc<T> {
    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }
}


#[cfg(feature = "implement-IsEmpty-for-built_ins")]
mod impl_for_built_ins {


    mod isolate_ {
        #![allow(non_snake_case)]


        #[inline]
        pub(super) fn get_is_empty_str_(s : &str) -> bool {
            s.is_empty()
        }

        #[inline]
        pub(super) fn get_is_empty_Slice_<T>(s : &[T]) -> bool {
            s.is_empty()
        }
    }


    // str

    impl super::IsEmpty for str {
        #[inline]
        fn is_empty(&self) -> bool {
            isolate_::get_is_empty_str_(self)
        }
    }

    impl super::IsEmpty for &str {
        #[inline]
        fn is_empty(&self) -> bool {
            isolate_::get_is_empty_str_(self)
        }
    }

    // Array

    impl<T, const N: usize> super::IsEmpty for [T; N] {
        #[inline]
        fn is_empty(&self) -> bool {
            0 == N
        }
    }

    impl<T, const N: usize> super::IsEmpty for &[T; N] {
        #[inline]
        fn is_empty(&self) -> bool {
            0 == N
        }
    }

    // Slice

    impl<T> super::IsEmpty for [T] {
        #[inline]
        fn is_empty(&self) -> bool {
            isolate_::get_is_empty_Slice_(self)
        }
    }

    impl<T> super::IsEmpty for &[T] {
        #[inline]
        fn is_empty(&self) -> bool {
            isolate_::get_is_empty_Slice_(self)
        }
    }
}


#[cfg(feature = "implement-IsEmpty-for-standard_collection_types")]
mod impl_for_std_coll_types {
    use std::collections as std_collections;


    mod isolate_ {
        #![allow(non_snake_case)]

        use std::collections as std_collections;


        #[inline]
        pub(super) fn get_is_empty_BTreeMap_<K, V>(coll : &std_collections::BTreeMap<K, V>) -> bool {
            coll.is_empty()
        }

        #[inline]
        pub(super) fn get_is_empty_BTreeSet_<T>(coll : &std_collections::BTreeSet<T>) -> bool {
            coll.is_empty()
        }

        #[inline]
        pub(super) fn get_is_empty_BinaryHeap_<T>(coll : &std_collections::BinaryHeap<T>) -> bool {
            coll.is_empty()
        }

        #[inline]
        pub(super) fn get_is_empty_HashMap_<K, V>(coll : &std_collections::HashMap<K, V>) -> bool {
            coll.is_empty()
        }

        #[inline]
        pub(super) fn get_is_empty_HashSet_<T>(coll : &std_collections::HashSet<T>) -> bool {
            coll.is_empty()
        }

        #[inline]
        pub(super) fn get_is_empty_LinkedList_<T>(coll : &std_collections::LinkedList<T>) -> bool {
            coll.is_empty()
        }

        // NOTE: parameter type is `&str`, not `&String`
        #[inline]
        pub(super) fn get_is_empty_String_(s : &str) -> bool {
            s.is_empty()
        }

        // NOTE: parameter type is `&[T]`, not `&Vec<T>`
        #[inline]
        pub(super) fn get_is_empty_Vec_<T>(coll : &[T]) -> bool {
            coll.is_empty()
        }

        #[inline]
        pub(super) fn get_is_empty_VecDeque_<T>(coll : &std_collections::VecDeque<T>) -> bool {
            coll.is_empty()
        }
    }


    // BTreeMap<>

    impl<K, V> super::IsEmpty for std_collections::BTreeMap<K, V> {
        #[inline]
        fn is_empty(&self) -> bool {
            isolate_::get_is_empty_BTreeMap_(self)
        }
    }

    // BTreeSet<>

    impl<T> super::IsEmpty for std_collections::BTreeSet<T> {
        #[inline]
        fn is_empty(&self) -> bool {
            isolate_::get_is_empty_BTreeSet_(self)
        }
    }

    // BinaryHeap<>

    impl<T> super::IsEmpty for std_collections::BinaryHeap<T> {
        #[inline]
        fn is_empty(&self) -> bool {
            isolate_::get_is_empty_BinaryHeap_(self)
        }
    }

    // HashMap<>

    impl<K, V> super::IsEmpty for std_collections::HashMap<K, V> {
        #[inline]
        fn is_empty(&self) -> bool {
            isolate_::get_is_empty_HashMap_(self)
        }
    }

    // HashSet<>

    impl<T> super::IsEmpty for std_collections::HashSet<T> {
        #[inline]
        fn is_empty(&self) -> bool {
            isolate_::get_is_empty_HashSet_(self)
        }
    }

    // LinkedList<>

    impl<T> super::IsEmpty for std_collections::LinkedList<T> {
        #[inline]
        fn is_empty(&self) -> bool {
            isolate_::get_is_empty_LinkedList_(self)
        }
    }

    // String

    impl super::IsEmpty for String {
        #[inline]
        fn is_empty(&self) -> bool {
            isolate_::get_is_empty_String_(self)
        }
    }

    // Vec<>

    impl<T> super::IsEmpty for Vec<T> {
        #[inline]
        fn is_empty(&self) -> bool {
            isolate_::get_is_empty_Vec_(self)
        }
    }

    // VecDeque<>

    impl<T> super::IsEmpty for std_collections::VecDeque<T> {
        #[inline]
        fn is_empty(&self) -> bool {
            isolate_::get_is_empty_VecDeque_(self)
        }
    }
}


#[cfg(feature = "implement-IsEmpty-for-standard_ffi_types")]
mod impl_for_std_ffi_types {
    #![allow(non_snake_case)]

    use std::ffi as std_ffi;


    mod isolate_ {
        #![allow(non_snake_case)]

        use std::ffi as std_ffi;


        #[inline]
        pub(super) fn get_is_empty_CStr_(cstr : &std_ffi::CStr) -> bool {
            cstr.is_empty()
        }

        #[inline]
        pub(super) fn get_is_empty_CString_(cstring : &std_ffi::CString) -> bool {
            cstring.is_empty()
        }
    }


    // CStr

    impl super::IsEmpty for std_ffi::CStr {
        #[inline]
        fn is_empty(&self) -> bool {
            isolate_::get_is_empty_CStr_(self)
        }
    }

    impl super::IsEmpty for &std_ffi::CStr {
        #[inline]
        fn is_empty(&self) -> bool {
            isolate_::get_is_empty_CStr_(self)
        }
    }

    // CString

    impl super::IsEmpty for std_ffi::CString {
        #[inline]
        fn is_empty(&self) -> bool {
            isolate_::get_is_empty_CString_(self)
        }
    }
}


#[cfg(feature = "implement-IsEmpty-for-standard_path_types")]
mod impl_for_std_path_types {
    use std::path as std_path;


    // Path

    impl super::IsEmpty for &std_path::Path {
        fn is_empty(&self) -> bool {
            self.as_os_str().is_empty()
        }
    }

    // PathBuf

    impl super::IsEmpty for std_path::PathBuf {
        fn is_empty(&self) -> bool {
            self.as_os_str().is_empty()
        }
    }

    impl super::IsEmpty for &std_path::PathBuf {
        fn is_empty(&self) -> bool {
            self.as_os_str().is_empty()
        }
    }
}


#[cfg(feature = "implement-IsEmpty-for-standard_process_types")]
mod impl_for_std_process_types {
    #![allow(non_snake_case)]

    #[cfg(feature = "experimental-exact_size_is_empty")]
    use std::process as std_process;


    mod isolate_ {
        #![allow(non_snake_case)]

        #[cfg(feature = "experimental-exact_size_is_empty")]
        use std::process as std_process;


        #[cfg(feature = "experimental-exact_size_is_empty")]
        #[inline]
        pub(super) fn get_is_empty_CommandArgs_<'a>(ca : &std_process::CommandArgs<'a>) -> bool {
            ca.is_empty()
        }

        #[cfg(feature = "experimental-exact_size_is_empty")]
        #[inline]
        pub(super) fn get_is_empty_CommandEnvs_<'a>(ce : &std_process::CommandEnvs<'a>) -> bool {
            ce.is_empty()
        }
    }


    // CommandArgs<'>

    #[cfg(feature = "experimental-exact_size_is_empty")]
    impl<'a> super::IsEmpty for &std_process::CommandArgs<'a> {
        fn is_empty(&self) -> bool {
            isolate_::get_is_empty_CommandArgs_(self)
        }
    }

    // CommandEnvs<'>

    #[cfg(feature = "experimental-exact_size_is_empty")]
    impl<'a> super::IsEmpty for &std_process::CommandEnvs<'a> {
        fn is_empty(&self) -> bool {
            isolate_::get_is_empty_CommandEnvs_(self)
        }
    }
}


#[cfg(feature = "implement-IsEmpty-for-standard_range_types")]
mod impl_for_std_range_types {
    #![allow(non_snake_case)]

    use std::{
        cmp as std_cmp,
        ops as std_ops,
    };


    mod isolate_ {
        use std::{
            cmp as std_cmp,
            ops as std_ops,
        };


        #[inline]
        pub(super) fn get_is_empty_Range_<Idx : std_cmp::PartialOrd>(r : &std_ops::Range<Idx>) -> bool {
            r.is_empty()
        }

        #[inline]
        pub(super) fn get_is_empty_RangeInclusive_<Idx : std_cmp::PartialOrd>(r : &std_ops::RangeInclusive<Idx>) -> bool {
            r.is_empty()
        }
    }


    // Range<>

    impl<Idx : std_cmp::PartialOrd> super::IsEmpty for std_ops::Range<Idx> {
        #[inline]
        fn is_empty(&self) -> bool {
            isolate_::get_is_empty_Range_(self)
        }
    }

    // RangeFrom<>

    impl<Idx> super::IsEmpty for std_ops::RangeFrom<Idx> {
        #[inline]
        fn is_empty(&self) -> bool {
            false
        }
    }

    // RangeFull<>

    impl super::IsEmpty for std_ops::RangeFull {
        #[inline]
        fn is_empty(&self) -> bool {
            false
        }
    }

    // RangeInclusive<>

    impl<Idx : std_cmp::PartialOrd> super::IsEmpty for std_ops::RangeInclusive<Idx> {
        #[inline]
        fn is_empty(&self) -> bool {
            isolate_::get_is_empty_RangeInclusive_(self)
        }
    }

    // RangeTo<>

    impl<Idx> super::IsEmpty for std_ops::RangeTo<Idx> {
        #[inline]
        fn is_empty(&self) -> bool {
            false
        }
    }
}


#[cfg(feature = "implement-IsEmpty-for-standard_time_types")]
mod impl_for_std_time_types {
    #![allow(non_snake_case)]

    use std::time as std_time;


    // Duration

    impl super::IsEmpty for std_time::Duration {
        #[inline]
        fn is_empty(&self) -> bool {
            self.is_zero()
        }
    }

    impl super::IsEmpty for &std_time::Duration {
        #[inline]
        fn is_empty(&self) -> bool {
            self.is_zero()
        }
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::IsEmpty;

    use std::rc::Rc;


    #[allow(unused)]
    fn as_IsEmpty<T : IsEmpty>(t : &T) -> &impl IsEmpty {
        t
    }


    mod TEST_CUSTOM_TYPE {
        #![allow(non_snake_case)]

        use super::*;


        #[derive(Debug)]
        struct CustomType {
            num_elements : usize,
        }

        impl IsEmpty for CustomType {
            fn is_empty(&self) -> bool {
                0 == self.num_elements
            }
        }


        #[test]
        fn TEST_WHEN_ZERO_ELEMENTS() {
            let ct = CustomType { num_elements : 0 };

            assert!(ct.is_empty());

            let ct = &ct;

            assert!(ct.is_empty());
        }

        #[test]
        fn TEST_WHEN_HAVE_ELEMENTS() {
            let ct = CustomType { num_elements : 1 };

            assert!(!ct.is_empty());

            let ct = &ct;

            assert!(!ct.is_empty());
        }

        #[test]
        fn TEST_WHEN_ZERO_ELEMENTS_IN_Box() {
            {
                let ct = Box::new(CustomType { num_elements : 0 });

                assert!(ct.is_empty());

                let ct = &ct;

                assert!(ct.is_empty());
            }

            {
                let ct = &Box::new(CustomType { num_elements : 0 });

                assert!(ct.is_empty());

                let ct = &ct;

                assert!(ct.is_empty());
            }

            {
                let ct = Box::new(&CustomType { num_elements : 0 });

                assert!(ct.is_empty());

                let ct = &ct;

                assert!(ct.is_empty());
            }

            {
                let ct = &Box::new(&CustomType { num_elements : 0 });

                assert!(ct.is_empty());

                let ct = &ct;

                assert!(ct.is_empty());
            }
        }

        #[test]
        fn TEST_WHEN_ZERO_ELEMENTS_IN_Rc() {
            {
                let ct = Rc::new(CustomType { num_elements : 0 });

                assert!(ct.is_empty());

                let ct = &ct;

                assert!(ct.is_empty());
            }

            {
                let ct = &Rc::new(CustomType { num_elements : 0 });

                assert!(ct.is_empty());

                let ct = &ct;

                assert!(ct.is_empty());
            }

            {
                let ct = Rc::new(&CustomType { num_elements : 0 });

                assert!(ct.is_empty());

                let ct = &ct;

                assert!(ct.is_empty());
            }

            {
                let ct = &Rc::new(&CustomType { num_elements : 0 });

                assert!(ct.is_empty());

                let ct = &ct;

                assert!(ct.is_empty());
            }
        }
    }


    #[cfg(feature = "implement-IsEmpty-for-built_ins")]
    mod TEST_BUILTIN_TYPES {
        #![allow(non_snake_case)]

        use super::*;


        mod TEST_str {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let s = "";

                assert!(s.is_empty());

                let ie = as_IsEmpty(&s);

                assert!(ie.is_empty());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let s = "abc";

                assert!(!s.is_empty());

                let ie = as_IsEmpty(&s);

                assert!(!ie.is_empty());
            }
        }


        mod TEST_Array {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let ar : [i64; 0] = [];

                assert!(ar.is_empty());

                let ie = as_IsEmpty(&ar);

                assert!(ie.is_empty());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let ar : [i64; 1] = [ 0 ];

                assert!(!ar.is_empty());

                let ie = as_IsEmpty(&ar);

                assert!(!ie.is_empty());
            }
        }


        mod TEST_Slice {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let ar : &[i64; 0] = &[];

                assert!(ar.is_empty());

                let ie = as_IsEmpty(&ar);

                assert!(ie.is_empty());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let ar = &[0];

                assert!(!ar.is_empty());

                let ie = as_IsEmpty(&ar);

                assert!(!ie.is_empty());
            }
        }
    }


    #[cfg(feature = "implement-IsEmpty-for-standard_collection_types")]
    mod TEST_STANDARD_TYPES {
        #![allow(non_snake_case)]

        use super::*;

        use std::collections::{
            BTreeMap,
            BTreeSet,
            BinaryHeap,
            HashMap,
            HashSet,
            LinkedList,
            VecDeque,
        };


        mod TEST_BTreeMapTU {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let v : BTreeMap<i32, i32> = Default::default();

                assert!(v.is_empty());

                let ie = as_IsEmpty(&v);

                assert!(ie.is_empty());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let v = BTreeMap::from_iter(vec![ (0, 0) ]);

                assert!(!v.is_empty());

                let ie = as_IsEmpty(&v);

                assert!(!ie.is_empty());
            }
        }


        mod TEST_BTreeSetT {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let v : BTreeSet<i32> = Default::default();

                assert!(v.is_empty());

                let ie = as_IsEmpty(&v);

                assert!(ie.is_empty());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let v = BTreeSet::from_iter(vec![ 0 ]);

                assert!(!v.is_empty());

                let ie = as_IsEmpty(&v);

                assert!(!ie.is_empty());
            }
        }


        mod TEST_BinaryHeapT {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let v : BinaryHeap<i32> = Default::default();

                assert!(v.is_empty());

                let ie = as_IsEmpty(&v);

                assert!(ie.is_empty());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let v = BinaryHeap::from_iter(vec![ 0 ]);

                assert!(!v.is_empty());

                let ie = as_IsEmpty(&v);

                assert!(!ie.is_empty());
            }
        }


        mod TEST_HashMapTU {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let v : HashMap<i32, i32> = Default::default();

                assert!(v.is_empty());

                let ie = as_IsEmpty(&v);

                assert!(ie.is_empty());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let v = HashMap::from_iter(vec![ (0, 0) ]);

                assert!(!v.is_empty());

                let ie = as_IsEmpty(&v);

                assert!(!ie.is_empty());
            }
        }


        mod TEST_HashSetT {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let v : HashSet<i32> = Default::default();

                assert!(v.is_empty());

                let ie = as_IsEmpty(&v);

                assert!(ie.is_empty());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let v = HashSet::from_iter(vec![ 0 ]);

                assert!(!v.is_empty());

                let ie = as_IsEmpty(&v);

                assert!(!ie.is_empty());
            }
        }


        mod TEST_LinkedListT {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let v : LinkedList<i32> = Default::default();

                assert!(v.is_empty());

                let ie = as_IsEmpty(&v);

                assert!(ie.is_empty());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let v = LinkedList::from_iter(vec![ 0 ]);

                assert!(!v.is_empty());

                let ie = as_IsEmpty(&v);

                assert!(!ie.is_empty());
            }
        }


        mod TEST_String {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let s : String = "".into();

                assert!(s.is_empty());

                let ie = as_IsEmpty(&s);

                assert!(ie.is_empty());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let s : String = "abc".into();

                assert!(!s.is_empty());

                let ie = as_IsEmpty(&s);

                assert!(!ie.is_empty());
            }
        }


        mod TEST_String_IN_Box {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let s : Box<String> = Box::new("".into());

                assert!(s.is_empty());

                let ie = as_IsEmpty(&s);

                assert!(ie.is_empty());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let s : Box<String> = Box::new("abc".into());

                assert!(!s.is_empty());

                let ie = as_IsEmpty(&s);

                assert!(!ie.is_empty());
            }
        }


        mod TEST_VecT {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let v : Vec<i32> = Default::default();

                assert!(v.is_empty());

                let ie = as_IsEmpty(&v);

                assert!(ie.is_empty());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let v : Vec<i32> = vec![ 0 ];

                assert!(!v.is_empty());

                let ie = as_IsEmpty(&v);

                assert!(!ie.is_empty());
            }
        }


        mod TEST_VecDequeT {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let v : VecDeque<i32> = Default::default();

                assert!(v.is_empty());

                let ie = as_IsEmpty(&v);

                assert!(ie.is_empty());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let v = VecDeque::from_iter(vec![ 0 ]);

                assert!(!v.is_empty());

                let ie = as_IsEmpty(&v);

                assert!(!ie.is_empty());
            }
        }
    }


    #[cfg(feature = "implement-IsEmpty-for-standard_ffi_types")]
    mod TEST_FFI_TYPES {
        #![allow(non_snake_case)]

        use super::*;

        use std::ffi::{
            CStr,
            CString,
        };


        mod TEST_CStr {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let s : &CStr = &CString::new("").unwrap();

                assert!(s.is_empty());

                let ie = as_IsEmpty(&s);

                assert!(ie.is_empty());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let s : &CStr = &CString::new("abc").unwrap();

                assert!(!s.is_empty());

                let ie = as_IsEmpty(&s);

                assert!(!ie.is_empty());
            }
        }


        mod TEST_CString {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let s : CString = CString::new("").unwrap();

                assert!(s.is_empty());

                let ie = as_IsEmpty(&s);

                assert!(ie.is_empty());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let s : CString = CString::new("abc").unwrap();

                assert!(!s.is_empty());

                let ie = as_IsEmpty(&s);

                assert!(!ie.is_empty());
            }
        }
    }


    #[cfg(feature = "implement-IsEmpty-for-standard_path_types")]
    mod TEST_PATH_TYPES {
        #![allow(non_snake_case)]

        use super::*;

        use std::path::{
            Path,
            PathBuf,
        };


        mod TEST_Path {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let p = Path::new("");

                assert!(p.is_empty());

                let ie = as_IsEmpty(&p);

                assert!(ie.is_empty());
            }

            #[test]
            fn TEST_NOTEMPTY() {
                let p = Path::new("./foo/bar.txt");

                assert!(!p.is_empty());

                let ie = as_IsEmpty(&p);

                assert!(!ie.is_empty());
            }
        }


        mod TEST_PathBuf {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let p = PathBuf::new();

                assert!(p.is_empty());

                let ie = as_IsEmpty(&p);

                assert!(ie.is_empty());
            }

            #[test]
            fn TEST_NOTEMPTY() {
                let mut p = PathBuf::new();

                p.push("./foo/bar.txt");

                assert!(!p.is_empty());

                let ie = as_IsEmpty(&p);

                assert!(!ie.is_empty());
            }
        }
    }


    #[cfg(feature = "implement-IsEmpty-for-standard_process_types")]
    mod TEST_PROCESS_TYPES {
        #![allow(non_snake_case)]

    }


    #[cfg(feature = "implement-IsEmpty-for-standard_range_types")]
    mod TEST_RANGE_TYPES {
        #![allow(non_snake_case)]

        use super::*;


        mod TEST_Range {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let r = 0..0;

                assert!(r.is_empty());

                let ie = as_IsEmpty(&r);

                assert!(ie.is_empty());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let r = 0..1;

                assert!(!r.is_empty());

                let ie = as_IsEmpty(&r);

                assert!(!ie.is_empty());
            }
        }


        mod TEST_RangeFrom {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_NONEMPTY() {
                let r = 0..;

                assert!(!r.is_empty());

                let ie = as_IsEmpty(&r);

                assert!(!ie.is_empty());
            }
        }


        mod TEST_RangeFull {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_NONEMPTY() {
                let r = ..;

                assert!(!r.is_empty());

                let ie = as_IsEmpty(&r);

                assert!(!ie.is_empty());
            }
        }


        mod TEST_RangeInclusive {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_NONEMPTY() {
                let r = 0..=1;

                assert!(!r.is_empty());

                let ie = as_IsEmpty(&r);

                assert!(!ie.is_empty());
            }
        }


        mod TEST_RangeTo {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_NONEMPTY() {
                let r = ..1;

                assert!(!r.is_empty());

                let ie = as_IsEmpty(&r);

                assert!(!ie.is_empty());
            }
        }
    }


    #[cfg(feature = "implement-IsEmpty-for-standard_time_types")]
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
            fn TEST_EMPTY() {
                let d = Duration::from_micros(0);

                assert!(d.is_empty());

                let ie = as_IsEmpty(&d);

                assert!(ie.is_empty());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let d = Duration::from_micros(1);

                assert!(!d.is_empty());

                let ie = as_IsEmpty(&d);

                assert!(!ie.is_empty());
            }
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //

