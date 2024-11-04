// src/traits/as_u32.rs : `AsU32`

/// Trait defining instance method `as_u32() : u32` that provides a
/// cost-free conversion into `u32`.
///
/// It is expected that the implementing type "is-a" `u32` in a direct
/// manner as well as in a logical manner.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-AsU32-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following type(s):
/// - [`u32`];
pub trait AsU32 {
    fn as_u32(&self) -> u32;
}


#[cfg(feature = "implement-AsU32-for-built_ins")]
mod impl_for_built_ins {
    #![allow(non_snake_case)]


    impl super::AsU32 for u32 {
        #[inline]
        fn as_u32(&self) -> u32 {
            *self
        }
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::AsU32;


    #[cfg(feature = "implement-AsU32-for-built_ins")]
    #[test]
    fn TEST_u32_AsU32() {

        {
            let v : u32 = 12345678;
            let actual = v.as_u32();

            assert_eq!(12345678, actual);
        }

        {
            let v : &u32 = &12345678;
            let actual = v.as_u32();

            assert_eq!(12345678, actual);
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //
