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
    AsI128,
    AsI32,
    AsI64,
    AsISize,
    AsStr,
    AsU128,
    AsU32,
    AsU64,
    AsUSize,
    Infinity,
    IsDefault,
    IsEmpty,
    IsInfinity,
    IsNAN,
    IsZero,
    Len,
    ToF64,
    ToISize,
    ToUSize,
    Zero,
};

mod private {
    #[allow(unused_imports)]
    pub(crate) use super::traits::Sealed;
}


// ///////////////////////////// end of file //////////////////////////// //

