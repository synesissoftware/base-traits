// src/traits/as_i128.rs : `AsI128`

/// Trait defining instance method `as_i128() : i128` that provides a
/// cost-free conversion into `i128`.
///
/// It is expected that the implementing type "is-a" `i128` in a direct
/// manner as well as in a logical manner.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-AsI128-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following type(s):
/// - [`i128`];
pub trait AsI128 {
    fn as_i128(&self) -> i128;
}


#[cfg(feature = "implement-AsI128-for-built_ins")]
mod impl_for_built_ins {
    #![allow(non_snake_case)]


    impl super::AsI128 for i128 {
        #[inline]
        fn as_i128(&self) -> i128 {
            *self
        }
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::AsI128;


    #[cfg(feature = "implement-AsI128-for-built_ins")]
    #[test]
    fn TEST_i128_AsI128() {

        {
            let v : i128 = 12345678;
            let actual = v.as_i128();

            assert_eq!(12345678, actual);
        }

        {
            let v : &i128 = &12345678;
            let actual = v.as_i128();

            assert_eq!(12345678, actual);
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //

