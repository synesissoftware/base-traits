// src/traits/as_i64.rs : `AsI64`

/// Trait defining instance method `as_i64() : i64` that provides a
/// cost-free conversion into `i64`.
///
/// It is expected that the implementing type "is-a" `i64` in a direct
/// manner as well as in a logical manner.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-AsI64-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following type(s):
/// - [`i64`];
pub trait AsI64 {
    fn as_i64(&self) -> i64;
}


#[cfg(feature = "implement-AsI64-for-built_ins")]
mod impl_for_built_ins {
    #![allow(non_snake_case)]


    impl super::AsI64 for i64 {
        #[inline]
        fn as_i64(&self) -> i64 {
            *self
        }
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::AsI64;


    #[cfg(feature = "implement-AsI64-for-built_ins")]
    #[test]
    fn TEST_i64_AsI64() {

        {
            let v : i64 = 12345678;
            let actual = v.as_i64();

            assert_eq!(12345678, actual);
        }

        {
            let v : &i64 = &12345678;
            let actual = v.as_i64();

            assert_eq!(12345678, actual);
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //
