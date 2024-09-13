// src/traits/is_default.rs : `IsDefault`

/// Trait defining instance method `is_default() : bool` that allows a type
/// instance to indicate whether it holds the "default" value.
pub trait IsDefault {
	fn is_default(&self) -> bool;
}


// ///////////////////////////// end of file //////////////////////////// //
