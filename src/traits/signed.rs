// src/traits/signed.rs : `Signed`

pub trait Signed {
}


mod impl_for_built_ins {
    #![allow(non_snake_case)]

    use super::Signed;


    impl Signed for i8 {}
    impl Signed for i16 {}
    impl Signed for i32 {}
    impl Signed for i64 {}
    impl Signed for i128 {}

    impl Signed for isize {}

    impl Signed for f32 {}
    impl Signed for f64 {}
}


// ///////////////////////////// end of file //////////////////////////// //

