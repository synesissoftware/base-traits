// src/traits/unsigned.rs : `Unsigned`

pub trait Unsigned {
}


mod impl_for_built_ins {
    #![allow(non_snake_case)]

    use super::Unsigned;


    impl Unsigned for u8 {}
    impl Unsigned for u16 {}
    impl Unsigned for u32 {}
    impl Unsigned for u64 {}
    impl Unsigned for u128 {}

    impl Unsigned for usize {}
}


// ///////////////////////////// end of file //////////////////////////// //

