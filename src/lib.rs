// lib.rs : base-traits

//! General-purpose traits for generic Rust programming.
//!
//! The crate supplies small traits for concepts that are
//! useful across unrelated types, including [`IsEmpty`],
//! [`Len`], [`ToF64`], and [`Zero`]. Implementations for
//! built-in and standard-library types are controlled by
//! feature flags so consumers can choose the API surface
//! they need.
//!
//! # Example
//!
//! ```
//! use base_traits::ToF64;
//!
//! struct Price(f64);
//!
//! impl ToF64 for Price {
//!     fn to_f64(&self) -> f64 {
//!         self.0
//!     }
//! }
//!
//! let price = Price(12.50);
//! assert_eq!(12.50, price.to_f64());
//! ```
//!
//! The default feature set enables the common built-in and
//! standard-library implementations. The `"full"` feature
//! enables the broader set, including the experimental
//! process-type implementations. Use `"nostd"` when the
//! standard library is unavailable.

// /////////////////////////////////////////////////////////
// crate-level feature definitions

#![cfg_attr(feature = "experimental-exact_size_is_empty", feature(exact_size_is_empty))]
#![cfg_attr(all(not(test), feature = "nostd"), no_std)]

// /////////////////////////////////////////////////////////
// crate-level feature discrimination

// /////////////////////////////////////////////////////////
// imports

pub(crate) mod macros;

macros::declare_and_publish!(pub traits,
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
);

mod private {
    #![allow(unused_imports)]

    pub(crate) use super::traits::Sealed;
}

// ///////////////////////////// end of file //////////////////////////// //
