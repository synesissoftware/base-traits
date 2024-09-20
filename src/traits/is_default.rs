// src/traits/is_default.rs : `IsDefault`

/// Trait defining instance method `is_default() : bool` that allows a type
/// instance to indicate whether it holds the "default" value.
pub trait IsDefault {
	fn is_default(&self) -> bool;
}


impl<T : IsDefault + ?Sized> IsDefault for Box<T> {
    fn is_default(&self) -> bool {
        (**self).is_default()
    }
}

impl<T : IsDefault + ?Sized> IsDefault for std::rc::Rc<T> {
    fn is_default(&self) -> bool {
        (**self).is_default()
    }
}


// ///////////////////////////// end of file //////////////////////////// //

