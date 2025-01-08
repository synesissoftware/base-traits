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
    Integer,
    IsDefault,
    IsEmpty,
    IsInfinity,
    IsNAN,
    IsZero,
    Len,
    Numeric,
    Real,
    Scalar,
    Signed,
    ToF64,
    ToI128,
    ToI16,
    ToI32,
    ToI64,
    ToISize,
    ToU128,
    ToU16,
    ToU32,
    ToU64,
    ToUSize,
    Unsigned,
    Zero,
};

mod private {
    #[allow(unused_imports)]
    pub(crate) use super::traits::Sealed;
}


// ///////////////////////////// end of file //////////////////////////// //

