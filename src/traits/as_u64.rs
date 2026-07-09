// src/traits/as_u64.rs : `AsU64`

/// Trait defining instance method `as_u64() : u64` that provides a
/// cost-free conversion into `u64`.
///
/// It is expected that the implementing type "is-a" `u64` in a direct
/// manner as well as in a logical manner.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-AsU64-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following type(s):
/// - [`u64`];
pub trait AsU64 {
    fn as_u64(&self) -> u64;
}


#[cfg(feature = "implement-AsU64-for-built_ins")]
mod impl_for_built_ins {
    #![allow(non_snake_case)]


    impl super::AsU64 for u64 {
        #[inline]
        fn as_u64(&self) -> u64 {
            *self
        }
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::AsU64;


    #[cfg(feature = "implement-AsU64-for-built_ins")]
    #[test]
    fn TEST_u64_AsU64() {

        {
            let v : u64 = 12345678;
            let actual = v.as_u64();

            assert_eq!(12345678, actual);
        }

        {
            let v : &u64 = &12345678;
            let actual = v.as_u64();

            assert_eq!(12345678, actual);
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //

