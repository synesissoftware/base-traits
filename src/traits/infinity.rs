// src/traits/infinity.rs : `Infinity`

/// Trait defining class method `infinity() : T` that creates an instance of
/// the implementing type that is conceptually (or actually) infinity.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-Infinity-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following types:
/// - [`f32`];
/// - [`f64`];
pub trait Infinity {
    fn infinity() -> Self;
}


#[cfg(feature = "implement-Infinity-for-built_ins")]
mod impl_for_built_ins {
    #![allow(non_snake_case)]

    macro_rules! implement_Infinity_ {
        ($type:tt) => {
            impl super::Infinity for $type {
                #[inline]
                fn infinity() -> Self {
                    $type::INFINITY
                }
            }
        };
    }

    implement_Infinity_!(f32);
    implement_Infinity_!(f64);
}


// ///////////////////////////// end of file //////////////////////////// //

