// lib.rs : base-traits


// /////////////////////////////////////////////////////////
// crate-level feature definitions

#![cfg_attr(feature = "experimental-exact_size_is_empty", feature(exact_size_is_empty))]


// /////////////////////////////////////////////////////////
// crate-level feature discrimination


// /////////////////////////////////////////////////////////
// imports

mod traits;

pub use traits::{
    AsF64,
    AsStr,
    Infinity,
    IsDefault,
    IsEmpty,
    IsInfinity,
    IsNAN,
    IsZero,
    Len,
    ToF64,
    Zero,
};

mod private {
    #[allow(unused_imports)]
    pub(crate) use super::traits::Sealed;
}


// ///////////////////////////// end of file //////////////////////////// //
