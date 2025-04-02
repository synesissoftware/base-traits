// src/traits/len.rs : `Len`

/// Trait defining instance method `len() : usize` that indicates
/// whether the implementing type instance is logically empty.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-Len-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following types:
/// - [`str`];
/// - `[T; N]`;
/// - `[T]`;
///
/// ## Standard Collection Types
///
/// If the feature `"implement-Len-for-standard_collection_types"`
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
/// If the feature `"implement-Len-for-standard_ffi_types"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following types:
/// - [`std::ffi::CStr`];
/// - [`std::ffi::CString`];
///
/// ## Standard Path Types
///
/// If the feature `"implement-Len-for-standard_path_types"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following types:
/// - [`std::path::Path`];
/// - [`std::path::PathBuf`];
///
/// ## Standard Process Types
///
/// If the feature `"implement-Len-for-standard_process_types"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following types:
/// - [`std::process::CommandArgs`];
/// - [`std::process::CommandEnvs`];
#[allow(clippy::len_without_is_empty)]
pub trait Len {
    fn len(&self) -> usize;
}


#[cfg(all(not(test), not(feature = "nostd")))]
impl<T : Len + ?Sized> Len for Box<T> {
    fn len(&self) -> usize {
        (**self).len()
    }
}

#[cfg(all(not(test), not(feature = "nostd")))]
impl<T : Len + ?Sized> Len for std::rc::Rc<T> {
    fn len(&self) -> usize {
        (**self).len()
    }
}


#[cfg(feature = "implement-Len-for-built_ins")]
mod impl_for_built_ins {

    mod isolate_ {
        #![allow(non_snake_case)]


        #[inline]
        pub(super) fn get_len_str_(s : &str) -> usize {
            s.len()
        }

        #[inline]
        pub(super) fn get_len_Slice_<T>(s : &[T]) -> usize {
            s.len()
        }
    }


    // str

    impl super::Len for str {
        fn len(&self) -> usize {
            isolate_::get_len_str_(self)
        }
    }

    impl super::Len for &str {
        fn len(&self) -> usize {
            isolate_::get_len_str_(self)
        }
    }

    // Array

    impl<T, const N: usize> super::Len for [T; N] {
        fn len(&self) -> usize {
            N
        }
    }

    impl<T, const N: usize> super::Len for &[T; N] {
        fn len(&self) -> usize {
            N
        }
    }

    // Slice

    impl<T> super::Len for [T] {
        fn len(&self) -> usize {
            isolate_::get_len_Slice_(self)
        }
    }

    impl<T> super::Len for &[T] {
        fn len(&self) -> usize {
            isolate_::get_len_Slice_(self)
        }
    }
}


#[cfg(all(not(feature = "nostd"), feature = "implement-Len-for-standard_collection_types"))]
mod impl_for_std_coll_types {
    use std::collections as std_collections;


    mod isolate_ {
        #![allow(non_snake_case)]

        use std::collections as std_collections;


        #[inline]
        pub(super) fn get_len_BTreeMap_<K, V>(coll : &std_collections::BTreeMap<K, V>) -> usize {
            coll.len()
        }

        #[inline]
        pub(super) fn get_len_BTreeSet_<T>(coll : &std_collections::BTreeSet<T>) -> usize {
            coll.len()
        }

        #[inline]
        pub(super) fn get_len_BinaryHeap_<T>(coll : &std_collections::BinaryHeap<T>) -> usize {
            coll.len()
        }

        #[inline]
        pub(super) fn get_len_HashMap_<K, V>(coll : &std_collections::HashMap<K, V>) -> usize {
            coll.len()
        }

        #[inline]
        pub(super) fn get_len_HashSet_<T>(coll : &std_collections::HashSet<T>) -> usize {
            coll.len()
        }

        #[inline]
        pub(super) fn get_len_LinkedList_<T>(coll : &std_collections::LinkedList<T>) -> usize {
            coll.len()
        }

        // NOTE: parameter type is `&str`, not `&String`
        #[inline]
        pub(super) fn get_len_String_(s : &str) -> usize {
            s.len()
        }

        #[inline]
        pub(super) fn get_len_Vec_<T>(coll : &[T]) -> usize {
            coll.len()
        }

        #[inline]
        pub(super) fn get_len_VecDeque_<T>(coll : &std_collections::VecDeque<T>) -> usize {
            coll.len()
        }
    }


    // BTreeMap<>

    impl<K, V> super::Len for std_collections::BTreeMap<K, V> {
        fn len(&self) -> usize {
            isolate_::get_len_BTreeMap_(self)
        }
    }

    // BTreeSet<>

    impl<T> super::Len for std_collections::BTreeSet<T> {
        fn len(&self) -> usize {
            isolate_::get_len_BTreeSet_(self)
        }
    }

    // BinaryHeap<>

    impl<T> super::Len for std_collections::BinaryHeap<T> {
        fn len(&self) -> usize {
            isolate_::get_len_BinaryHeap_(self)
        }
    }

    // HashMap<>

    impl<K, V> super::Len for std_collections::HashMap<K, V> {
        fn len(&self) -> usize {
            isolate_::get_len_HashMap_(self)
        }
    }

    // HashSet<>

    impl<T> super::Len for std_collections::HashSet<T> {
        fn len(&self) -> usize {
            isolate_::get_len_HashSet_(self)
        }
    }

    // LinkedList<>

    impl<T> super::Len for std_collections::LinkedList<T> {
        fn len(&self) -> usize {
            isolate_::get_len_LinkedList_(self)
        }
    }

    // String

    impl super::Len for String {
        fn len(&self) -> usize {
            isolate_::get_len_String_(self)
        }
    }

    // Vec<>

    impl<T> super::Len for Vec<T> {
        fn len(&self) -> usize {
            isolate_::get_len_Vec_(self)
        }
    }

    // VecDeque<>

    impl<T> super::Len for std_collections::VecDeque<T> {
        fn len(&self) -> usize {
            isolate_::get_len_VecDeque_(self)
        }
    }
}


#[cfg(all(not(feature = "nostd"), feature = "implement-Len-for-standard_ffi_types"))]
mod impl_for_std_ffi_types {
    #![allow(non_snake_case)]

    use std::ffi as std_ffi;


    mod isolate_ {
        #![allow(non_snake_case)]

        use std::ffi as std_ffi;


        #[inline]
        pub(super) fn get_len_CStr_(cstr : &std_ffi::CStr) -> usize {
            cstr.count_bytes()
        }

        #[inline]
        pub(super) fn get_len_CString_(cstring : &std_ffi::CString) -> usize {
            cstring.count_bytes()
        }
    }


    // CStr

    impl super::Len for std_ffi::CStr {
        fn len(&self) -> usize {
            isolate_::get_len_CStr_(self)
        }
    }

    impl super::Len for &std_ffi::CStr {
        fn len(&self) -> usize {
            isolate_::get_len_CStr_(self)
        }
    }

    // CString

    impl super::Len for std_ffi::CString {
        fn len(&self) -> usize {
            isolate_::get_len_CString_(self)
        }
    }
}


#[cfg(all(not(feature = "nostd"), feature = "implement-Len-for-standard_path_types"))]
mod impl_for_std_path_types {
    use std::path as std_path;


    // Path

    impl super::Len for &std_path::Path {
        fn len(&self) -> usize {
            self.as_os_str().len()
        }
    }

    // PathBuf

    impl super::Len for std_path::PathBuf {
        fn len(&self) -> usize {
            self.as_os_str().len()
        }
    }

    impl super::Len for &std_path::PathBuf {
        fn len(&self) -> usize {
            self.as_os_str().len()
        }
    }
}


#[cfg(all(not(feature = "nostd"), feature = "implement-Len-for-standard_process_types"))]
mod impl_for_std_process_types {
    #![allow(non_snake_case)]

    use std::process as std_process;


    mod isolate_ {
        #![allow(non_snake_case)]

        use std::process as std_process;


        #[inline]
        pub(super) fn get_len_CommandArgs_<'a>(ca : &std_process::CommandArgs<'a>) -> usize {
            ca.len()
        }

        #[inline]
        pub(super) fn get_len_CommandEnvs_<'a>(ce : &std_process::CommandEnvs<'a>) -> usize {
            ce.len()
        }
    }


    // CommandArgs<'>

    impl<'a> super::Len for &std_process::CommandArgs<'a> {
        fn len(&self) -> usize {
            isolate_::get_len_CommandArgs_(self)
        }
    }

    // CommandEnvs<'>

    impl<'a> super::Len for &std_process::CommandEnvs<'a> {
        fn len(&self) -> usize {
            isolate_::get_len_CommandEnvs_(self)
        }
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::Len;

    #[cfg(feature = "implement-Len-for-standard_collection_types")]
    use std::rc::Rc;


    #[allow(unused)]
    fn as_Len<T : Len>(t : &T) -> &impl Len {
        t
    }


    mod TEST_CUSTOM_TYPE {
        #![allow(non_snake_case)]

        use super::*;


        #[derive(Debug)]
        struct CustomType {
            num_elements : usize,
        }

        impl Len for CustomType {
            fn len(&self) -> usize {
                self.num_elements
            }
        }


        #[test]
        fn TEST_WHEN_ZERO_ELEMENTS() {
            let ct = CustomType { num_elements : 0 };

            assert_eq!(0, ct.len());

            let ct = &ct;

            assert_eq!(0, ct.len());
        }

        #[test]
        fn TEST_WHEN_HAVE_ELEMENTS() {
            let ct = CustomType { num_elements : 1 };

            assert_ne!(0, ct.len());

            let ct = &ct;

            assert_ne!(0, ct.len());
        }
    }


    #[cfg(feature = "implement-Len-for-built_ins")]
    mod TEST_BUILTIN_TYPES {
        #![allow(non_snake_case)]

        use super::*;


        mod TEST_str {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let s = "";

                assert_eq!(0, s.len());

                let ie = as_Len(&s);

                assert_eq!(0, ie.len());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let s = "abc";

                assert_ne!(0, s.len());

                let ie = as_Len(&s);

                assert_ne!(0, ie.len());
            }
        }


        mod TEST_Array {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let ar : [i64; 0] = [];

                assert_eq!(0, ar.len());

                let ie = as_Len(&ar);

                assert_eq!(0, ie.len());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let ar : [i64; 1] = [ 0 ];

                assert_ne!(0, ar.len());

                let ie = as_Len(&ar);

                assert_ne!(0, ie.len());
            }
        }


        mod TEST_Slice {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let ar : &[i64; 0] = &[];

                assert_eq!(0, ar.len());

                let ie = as_Len(&ar);

                assert_eq!(0, ie.len());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let ar = &[0];

                assert_ne!(0, ar.len());

                let ie = as_Len(&ar);

                assert_ne!(0, ie.len());
            }
        }
    }


    #[cfg(feature = "implement-Len-for-standard_collection_types")]
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

                assert_eq!(0, v.len());

                let ie = as_Len(&v);

                assert_eq!(0, ie.len());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let v = BTreeMap::from_iter(vec![ (0, 0) ]);

                assert_ne!(0, v.len());

                let ie = as_Len(&v);

                assert_ne!(0, ie.len());
            }
        }


        mod TEST_BTreeSetT {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let v : BTreeSet<i32> = Default::default();

                assert_eq!(0, v.len());

                let ie = as_Len(&v);

                assert_eq!(0, ie.len());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let v = BTreeSet::from_iter(vec![ 0 ]);

                assert_ne!(0, v.len());

                let ie = as_Len(&v);

                assert_ne!(0, ie.len());
            }
        }


        mod TEST_BinaryHeapT {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let v : BinaryHeap<i32> = Default::default();

                assert_eq!(0, v.len());

                let ie = as_Len(&v);

                assert_eq!(0, ie.len());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let v = BinaryHeap::from_iter(vec![ 0 ]);

                assert_ne!(0, v.len());

                let ie = as_Len(&v);

                assert_ne!(0, ie.len());
            }
        }


        mod TEST_HashMapTU {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let v : HashMap<i32, i32> = Default::default();

                assert_eq!(0, v.len());

                let ie = as_Len(&v);

                assert_eq!(0, ie.len());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let v = HashMap::from_iter(vec![ (0, 0) ]);

                assert_ne!(0, v.len());

                let ie = as_Len(&v);

                assert_ne!(0, ie.len());
            }
        }


        mod TEST_HashSetT {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let v : HashSet<i32> = Default::default();

                assert_eq!(0, v.len());

                let ie = as_Len(&v);

                assert_eq!(0, ie.len());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let v = HashSet::from_iter(vec![ 0 ]);

                assert_ne!(0, v.len());

                let ie = as_Len(&v);

                assert_ne!(0, ie.len());
            }
        }


        mod TEST_LinkedListT {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let v : LinkedList<i32> = Default::default();

                assert_eq!(0, v.len());

                let ie = as_Len(&v);

                assert_eq!(0, ie.len());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let v = LinkedList::from_iter(vec![ 0 ]);

                assert_ne!(0, v.len());

                let ie = as_Len(&v);

                assert_ne!(0, ie.len());
            }
        }


        mod TEST_String {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let s : String = "".into();

                assert_eq!(0, s.len());

                let ie = as_Len(&s);

                assert_eq!(0, ie.len());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let s : String = "abc".into();

                assert_ne!(0, s.len());

                let ie = as_Len(&s);

                assert_ne!(0, ie.len());
            }
        }


        mod TEST_String_IN_Box {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let s : Box<String> = Box::new("".into());

                assert_eq!(0, s.len());

                let ie = as_Len(&s);

                assert_eq!(0, ie.len());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let s : Box<String> = Box::new("abc".into());

                assert_ne!(0, s.len());

                let ie = as_Len(&s);

                assert_ne!(0, ie.len());
            }
        }


        mod TEST_String_IN_Rc {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let s : Rc<String> = Rc::new("".into());

                assert_eq!(0, s.len());

                let ie = as_Len(&s);

                assert_eq!(0, ie.len());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let s : Rc<String> = Rc::new("abc".into());

                assert_ne!(0, s.len());

                let ie = as_Len(&s);

                assert_ne!(0, ie.len());
            }
        }


        mod TEST_VecT {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let v : Vec<i32> = Default::default();

                assert_eq!(0, v.len());

                let ie = as_Len(&v);

                assert_eq!(0, ie.len());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let v : Vec<i32> = vec![ 0 ];

                assert_ne!(0, v.len());

                let ie = as_Len(&v);

                assert_ne!(0, ie.len());
            }
        }


        mod TEST_VecDequeT {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let v : VecDeque<i32> = Default::default();

                assert_eq!(0, v.len());

                let ie = as_Len(&v);

                assert_eq!(0, ie.len());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let v = VecDeque::from_iter(vec![ 0 ]);

                assert_ne!(0, v.len());

                let ie = as_Len(&v);

                assert_ne!(0, ie.len());
            }
        }
    }


    #[cfg(feature = "implement-Len-for-standard_ffi_types")]
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

                assert_eq!(0, s.len());

                let ie = as_Len(&s);

                assert_eq!(0, ie.len());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let s : &CStr = &CString::new("abc").unwrap();

                assert_ne!(0, s.len());

                let ie = as_Len(&s);

                assert_ne!(0, ie.len());
            }
        }


        mod TEST_CString {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let s : CString = CString::new("").unwrap();

                assert_eq!(0, s.len());

                let ie = as_Len(&s);

                assert_eq!(0, ie.len());
            }

            #[test]
            fn TEST_NONEMPTY() {
                let s : CString = CString::new("abc").unwrap();

                assert_ne!(0, s.len());

                let ie = as_Len(&s);

                assert_ne!(0, ie.len());
            }
        }
    }


    #[cfg(feature = "implement-Len-for-standard_path_types")]
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

                assert_eq!(0, p.len());

                let ie = as_Len(&p);

                assert_eq!(0, ie.len());
            }

            #[test]
            fn TEST_NOTEMPTY() {
                let p = Path::new("./foo/bar.txt");

                assert_ne!(0, p.len());

                let ie = as_Len(&p);

                assert_ne!(0, ie.len());
            }
        }


        mod TEST_PathBuf {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_EMPTY() {
                let p = PathBuf::new();

                assert_eq!(0, p.len());

                let ie = as_Len(&p);

                assert_eq!(0, ie.len());
            }

            #[test]
            fn TEST_NOTEMPTY() {
                let mut p = PathBuf::new();

                p.push("./foo/bar.txt");

                assert_ne!(0, p.len());

                let ie = as_Len(&p);

                assert_ne!(0, ie.len());
            }
        }
    }


    #[cfg(feature = "implement-Len-for-standard_process_types")]
    mod TEST_PROCESS_TYPES {
        #![allow(non_snake_case)]

    }
}


// ///////////////////////////// end of file //////////////////////////// //

