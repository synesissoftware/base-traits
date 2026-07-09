// src/traits/real.rs : `Real`

pub trait Real {
}


mod impl_for_built_ins {
    #![allow(non_snake_case)]

    use super::Real;


    impl Real for f32 {}
    impl Real for f64 {}
}


// ///////////////////////////// end of file //////////////////////////// //

