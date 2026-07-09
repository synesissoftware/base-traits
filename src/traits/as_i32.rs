// src/traits/as_i32.rs : `AsI32`

/// Trait defining instance method `as_i32() : i32` that provides a
/// cost-free conversion into `i32`.
///
/// It is expected that the implementing type "is-a" `i32` in a direct
/// manner as well as in a logical manner.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-AsI32-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following type(s):
/// - [`i32`];
pub trait AsI32 {
    fn as_i32(&self) -> i32;
}


#[cfg(feature = "implement-AsI32-for-built_ins")]
mod impl_for_built_ins {
    #![allow(non_snake_case)]


    impl super::AsI32 for i32 {
        #[inline]
        fn as_i32(&self) -> i32 {
            *self
        }
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::AsI32;


    #[cfg(feature = "implement-AsI32-for-built_ins")]
    #[test]
    fn TEST_i32_AsI32() {

        {
            let v : i32 = 12345678;
            let actual = v.as_i32();

            assert_eq!(12345678, actual);
        }

        {
            let v : &i32 = &12345678;
            let actual = v.as_i32();

            assert_eq!(12345678, actual);
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //

