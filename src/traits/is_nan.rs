// src/traits/is_nan.rs : `IsNAN`

/// Trait defining instance method `is_nan() : bool` that indicates
/// whether the implementing type instance has a value that is deemed to be
/// "not a number" (as in so for [`f32::NAN`] [`f64::NAN`]).
///
/// # Additional Implementations on Foreign Types
///
/// ## Built-in Types
///
/// If the feature `"implement-IsNAN-for-built_ins"`
/// is defined (as it is by `"default"`), then this is also implemented
/// for the following types:
/// - [`f32`];
/// - [`f64`];
pub trait IsNAN {
    fn is_nan(&self) -> bool;
}


#[cfg(all(not(test), not(feature = "nostd")))]
impl<T : IsNAN + ?Sized> IsNAN for Box<T> {
    fn is_nan(&self) -> bool {
        (**self).is_nan()
    }
}

#[cfg(all(not(test), not(feature = "nostd")))]
impl<T : IsNAN + ?Sized> IsNAN for std::rc::Rc<T> {
    fn is_nan(&self) -> bool {
        (**self).is_nan()
    }
}


#[cfg(feature = "implement-IsNAN-for-built_ins")]
mod impl_for_built_ins {
    #![allow(non_snake_case)]

    macro_rules! implement_IsNAN_ {
        ($type:tt) => {
            impl super::IsNAN for $type {
                #[inline]
                fn is_nan(&self) -> bool {
                    $type::is_nan(*self)
                }
            }
        };
    }

    implement_IsNAN_!(f32);
    implement_IsNAN_!(f64);
}


// ///////////////////////////// end of file //////////////////////////// //

