// src/traits/integer.rs : `Integer`

pub trait Integer {
}


mod impl_for_built_ins {
    #![allow(non_snake_case)]

    use super::Integer;


    impl Integer for i8 {}
    impl Integer for u8 {}
    impl Integer for i16 {}
    impl Integer for u16 {}
    impl Integer for i32 {}
    impl Integer for u32 {}
    impl Integer for i64 {}
    impl Integer for u64 {}
    impl Integer for i128 {}
    impl Integer for u128 {}

    impl Integer for isize {}
    impl Integer for usize {}
}


// ///////////////////////////// end of file //////////////////////////// //

