// src/traits/is_infinity.rs : `IsInfinity`

/// Trait defining instance method `is_infinity() : bool` that indicates
/// whether the implementing type instance is conceptually (or actually)
/// infinite.
pub trait IsInfinity {
    fn is_infinity(&self) -> bool;
}


impl<T : IsInfinity + ?Sized> IsInfinity for Box<T> {
    fn is_infinity(&self) -> bool {
        (**self).is_infinity()
    }
}

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
