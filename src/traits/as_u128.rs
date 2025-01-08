// src/traits/as_u128.rs : `AsU128`

/// Trait defining instance method `as_u128() : u128` that provides a
/// cost-free conversion into `u128`.
///
/// It is expected that the implementing type "is-a" `u128` in a direct
/// manner as well as in a logical manner.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-AsU128-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following type(s):
/// - [`u128`];
pub trait AsU128 {
    fn as_u128(&self) -> u128;
}


#[cfg(feature = "implement-AsU128-for-built_ins")]
mod impl_for_built_ins {
    #![allow(non_snake_case)]


    impl super::AsU128 for u128 {
        #[inline]
        fn as_u128(&self) -> u128 {
            *self
        }
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::AsU128;


    #[cfg(feature = "implement-AsU128-for-built_ins")]
    #[test]
    fn TEST_u128_AsU128() {

        {
            let v : u128 = 12345678;
            let actual = v.as_u128();

            assert_eq!(12345678, actual);
        }

        {
            let v : &u128 = &12345678;
            let actual = v.as_u128();

            assert_eq!(12345678, actual);
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //

