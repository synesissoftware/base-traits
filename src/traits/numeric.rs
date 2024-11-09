// src/traits/numeric.rs : `Numeric`

pub trait Numeric {
}


mod impl_for_built_ins {
    #![allow(non_snake_case)]

    use super::Numeric;


    impl Numeric for i8 {}
    impl Numeric for u8 {}
    impl Numeric for i16 {}
    impl Numeric for u16 {}
    impl Numeric for i32 {}
    impl Numeric for u32 {}
    impl Numeric for i64 {}
    impl Numeric for u64 {}
    impl Numeric for i128 {}
    impl Numeric for u128 {}

    impl Numeric for isize {}
    impl Numeric for usize {}

    impl Numeric for f32 {}
    impl Numeric for f64 {}
}


// ///////////////////////////// end of file //////////////////////////// //

