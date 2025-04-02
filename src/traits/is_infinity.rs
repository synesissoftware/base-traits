// src/traits/is_infinity.rs : `IsInfinity`

/// Trait defining instance method `is_infinity() : bool` that indicates
/// whether the implementing type instance is conceptually (or actually)
/// infinite.
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-IsInfinity-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following types:
/// - [`f32`];
/// - [`f64`];
pub trait IsInfinity {
    fn is_infinity(&self) -> bool;
}


#[cfg(all(not(test), not(feature = "nostd")))]
impl<T : IsInfinity + ?Sized> IsInfinity for Box<T> {
    fn is_infinity(&self) -> bool {
        (**self).is_infinity()
    }
}

#[cfg(all(not(test), not(feature = "nostd")))]
impl<T : IsInfinity + ?Sized> IsInfinity for std::rc::Rc<T> {
    fn is_infinity(&self) -> bool {
        (**self).is_infinity()
    }
}


#[cfg(feature = "implement-IsInfinity-for-built_ins")]
mod impl_for_built_ins {
    #![allow(non_snake_case)]

    macro_rules! implement_IsInfinity_ {
        ($type:tt) => {
            impl super::IsInfinity for $type {
                #[inline]
                fn is_infinity(&self) -> bool {
                    (*self).is_infinite()
                }
            }
        };
    }

    implement_IsInfinity_!(f32);
    implement_IsInfinity_!(f64);
}


// ///////////////////////////// end of file //////////////////////////// //

