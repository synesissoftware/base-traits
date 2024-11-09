// src/traits/scalar.rs : `Scalar`

pub trait Scalar {
}


mod impl_for_built_ins {
    #![allow(non_snake_case)]

    use super::Scalar;


    impl Scalar for bool {}

    impl Scalar for char {}

    impl Scalar for i8 {}
    impl Scalar for u8 {}
    impl Scalar for i16 {}
    impl Scalar for u16 {}
    impl Scalar for i32 {}
    impl Scalar for u32 {}
    impl Scalar for i64 {}
    impl Scalar for u64 {}
    impl Scalar for i128 {}
    impl Scalar for u128 {}

    impl Scalar for isize {}
    impl Scalar for usize {}

    impl Scalar for f32 {}
    impl Scalar for f64 {}
}


// ///////////////////////////// end of file //////////////////////////// //

