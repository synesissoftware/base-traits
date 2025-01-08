// traits/mod.rs

macro_rules! declare_and_publish {
    ($mod_name:ident, $($type_name:ident),*) => {
        mod $mod_name;

        pub use $mod_name::{
            $($type_name),*
        };
    };
}

declare_and_publish!(as_f64, AsF64);
declare_and_publish!(as_i128, AsI128);
declare_and_publish!(as_i32, AsI32);
declare_and_publish!(as_i64, AsI64);
declare_and_publish!(as_isize, AsISize);
declare_and_publish!(as_str, AsStr);
declare_and_publish!(as_u128, AsU128);
declare_and_publish!(as_u32, AsU32);
declare_and_publish!(as_u64, AsU64);
declare_and_publish!(as_usize, AsUSize);
declare_and_publish!(infinity, Infinity);
declare_and_publish!(integer, Integer);
declare_and_publish!(is_default, IsDefault);
declare_and_publish!(is_empty, IsEmpty);
declare_and_publish!(is_infinity, IsInfinity);
declare_and_publish!(is_nan, IsNAN);
declare_and_publish!(is_zero, IsZero);
declare_and_publish!(len, Len);
declare_and_publish!(numeric, Numeric);
declare_and_publish!(real, Real);
declare_and_publish!(scalar, Scalar);
declare_and_publish!(signed, Signed);
declare_and_publish!(to_f64, ToF64);
declare_and_publish!(to_i64, ToI64);
declare_and_publish!(to_i128, ToI128);
declare_and_publish!(to_isize, ToISize);
declare_and_publish!(to_u64, ToU64);
declare_and_publish!(to_u128, ToU128);
declare_and_publish!(to_usize, ToUSize);
declare_and_publish!(unsigned, Unsigned);
declare_and_publish!(zero, Zero);

mod sealed;
pub(crate) use sealed::Sealed;


// ///////////////////////////// end of file //////////////////////////// //
