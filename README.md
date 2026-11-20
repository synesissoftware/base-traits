# base-traits <!-- omit in toc -->

General-purpose traits for Rust that the standard library lacks, such as `IsEmpty`, `Len`, `ToU64`

![Language](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![Crates.io](https://img.shields.io/crates/v/base-traits.svg)](https://crates.io/crates/base-traits)
[![GitHub release](https://img.shields.io/github/v/release/synesissoftware/base-traits.svg)](https://github.com/synesissoftware/base-traits/releases/latest)
![MSRV](https://img.shields.io/badge/MSRV-1.74-lightgrey)
[![CI](https://github.com/synesissoftware/base-traits/actions/workflows/ci.yml/badge.svg)](https://github.com/synesissoftware/base-traits/actions/workflows/ci.yml)
[![docs.rs](https://docs.rs/base-traits/badge.svg)](https://docs.rs/base-traits)


## Table of Contents <!-- omit in toc -->

- [Introduction](#introduction)
- [Developer Experience](#developer-experience)
- [Installation](#installation)
- [Features](#features)
  - [Trait Features](#trait-features)
  - [Special Features](#special-features)
- [Components](#components)
  - [Traits](#traits)
- [Examples](#examples)
  - [In use in other projects](#in-use-in-other-projects)
- [Project Information](#project-information)
  - [Where to get help](#where-to-get-help)
  - [Contribution guidelines](#contribution-guidelines)
  - [Minimum Supported Rust Version (MSRV)](#minimum-supported-rust-version-msrv)
  - [Dependencies](#dependencies)
    - [Efferent (fan-out)](#efferent-fan-out)
      - [Runtime Dependencies](#runtime-dependencies)
      - [Build Dependencies](#build-dependencies)
      - [Development Dependencies](#development-dependencies)
    - [Afferent (fan-in)](#afferent-fan-in)
  - [Related projects](#related-projects)
  - [License](#license)


## Introduction

This crate defines general-purpose `trait`s that:
* seem obvious and yet are missing from the standard library (e.g.
  `IsEmpty`, `Len`); or
* provide a basis to introduce specific functionality to allow for other
  crates to interoperate;


## Developer Experience

**Requires:** a Rust toolchain at or above the MSRV (**1.74**); no other crates are needed at runtime.

```bash
cargo add base-traits
cargo test
cargo run --example price-to_f64
```

The simplest use case is to implement a trait on your own type, and then write generic code against that trait (this is the essence of the **price-to_f64** example):

```Rust
use base_traits::ToF64;

struct Price {
    dollars : u32,
    cents :   u8,
}

impl ToF64 for Price {
    fn to_f64(&self) -> f64 {
        self.dollars as f64 + (self.cents as f64 / 100.0)
    }
}

fn total<T : ToF64>(values : &[T]) -> f64 {
    values.iter().map(|v| v.to_f64()).sum()
}

fn main() {
    let prices = [
        Price { dollars : 10, cents : 50 },
        Price { dollars : 4, cents : 25 },
    ];

    println!("total = {}", total(&prices));
}
```

For more, see [Examples](#examples), **EXAMPLES.md**, [Features](#features), and the [API documentation](https://docs.rs/base-traits).


## Installation

Reference in **Cargo.toml** in the usual way:

```toml
base-traits = { version = "0.3" }
```


## Features

The following crate features are defined:


### Trait Features

| Name                                              | Effect | Is `"default"`? | Is `"full"`? | Dependent feature(s) |
| ------------------------------------------------- | ------ | --------------- | ------------ | -------------------- |
| `implement-AsF64-for-built_ins`                   | Causes `AsF64` to be implemented for built-in type `f64` | Yes | Yes | |
| `implement-AsI128-for-built_ins`                  | Causes `AsI128` to be implemented for built-in type `i128` | Yes | Yes | |
| `implement-AsI32-for-built_ins`                   | Causes `AsI32` to be implemented for built-in type `i32` | Yes | Yes | |
| `implement-AsI64-for-built_ins`                   | Causes `AsI64` to be implemented for built-in type `i64` | Yes | Yes | |
| `implement-AsISize-for-built_ins`                 | Causes `AsISize` to be implemented for built-in type `isize` | Yes | Yes | |
| `implement-AsStr-for-built_ins`                   | Causes `AsStr` to be implemented for built-in type `str` | Yes | Yes | |
| `implement-AsStr-for-String`                      | Causes `AsStr` to be implemented for standard collection type `String` | Yes | Yes (via `implement-AsStr-for-standard_collection_types`) | |
| `implement-AsStr-for-standard_collection_types`   | Causes `AsStr` to be implemented for standard collection type `String` | **No** | Yes | `implement-AsStr-for-String` |
| `implement-AsU128-for-built_ins`                  | Causes `AsU128` to be implemented for built-in type `u128` | Yes | Yes | |
| `implement-AsU32-for-built_ins`                   | Causes `AsU32` to be implemented for built-in type `u32` | Yes | Yes | |
| `implement-AsU64-for-built_ins`                   | Causes `AsU64` to be implemented for built-in type `u64` | Yes | Yes | |
| `implement-AsUSize-for-built_ins`                 | Causes `AsUSize` to be implemented for built-in type `usize` | Yes | Yes | |
| `implement-Infinity-for-built_ins`                | Causes `Infinity` to be implemented for built-in types `f32`, `f64` | Yes | Yes | |
| `implement-IsEmpty-for-built_ins`                 | Causes `IsEmpty` to be implemented for built-in types `str`, `[T; N]`, `[T]` | Yes | Yes | |
| `implement-IsEmpty-for-String`                    | Causes `IsEmpty` to be implemented for standard collection type `String` | Yes (via `implement-IsEmpty-for-standard_collection_types`) | Yes (via `implement-IsEmpty-for-standard_collection_types`) | |
| `implement-IsEmpty-for-standard_collection_types` | Causes `IsEmpty` to be implemented for standard collection types `BTreeMap`, `BTreeSet`, ..., `Vec`, `VecDeque`, and `String` | Yes | Yes | `implement-IsEmpty-for-String` |
| `implement-IsEmpty-for-standard_ffi_types`        | Causes `IsEmpty` to be implemented for standard FFI types `CStr`, `CString` | Yes | Yes | |
| `implement-IsEmpty-for-standard_path_types`       | Causes `IsEmpty` to be implemented for standard path types `Path`, `PathBuf` | Yes | Yes | |
| `implement-IsEmpty-for-standard_process_types`    | Causes `IsEmpty` to be implemented for standard process types `CommandArgs`, `CommandEnvs` | **No** | Yes | `experimental-exact_size_is_empty` |
| `implement-IsEmpty-for-standard_range_types`      | Causes `IsEmpty` to be implemented for standard range types `Range`, `RangeFrom`, etc. | Yes | Yes | |
| `implement-IsEmpty-for-standard_time_types`       | Causes `IsEmpty` to be implemented for standard time type `Duration` | **No** | Yes | |
| `implement-IsInfinity-for-built_ins`              | Causes `IsInfinity` to be implemented for built-in types `f32`, `f64` | Yes | Yes | |
| `implement-IsNAN-for-built_ins`                   | Causes `IsNAN` to be implemented for built-in types `f32`, `f64` | Yes | Yes | |
| `implement-IsNegative-for-built_ins`              | Causes `IsNegative` to be implemented for built-in types `f32`, `f64`, `i8`, ..., `i128`, `u8`, ..., `u128`, `isize`, `usize` | Yes | Yes | |
| `implement-IsPositive-for-built_ins`              | Causes `IsPositive` to be implemented for built-in types `f32`, `f64`, `i8`, ..., `i128`, `u8`, ..., `u128`, `isize`, `usize` | Yes | Yes | |
| `implement-IsZero-for-built_ins`                  | Causes `IsZero` to be implemented for built-in types `i8`, ..., `i128`, `u8`, ..., `u128`, `isize`, `usize`, `f32`, `f64`, `char` | Yes | Yes | |
| `implement-IsZero-for-standard_num_types`         | Reserved for `IsZero` implementations on standard numeric wrapper types | Yes | Yes | |
| `implement-IsZero-for-standard_process_types`     | Causes `IsZero` to be implemented for standard process type `ExitStatus` | Yes | Yes | |
| `implement-IsZero-for-standard_time_types`        | Causes `IsZero` to be implemented for standard time type `Duration` | Yes | Yes | |
| `implement-Len-for-built_ins`                     | Causes `Len` to be implemented for built-in types `str`, `[T; N]`, `[T]` | Yes | Yes | |
| `implement-Len-for-String`                        | Causes `Len` to be implemented for standard collection type `String` | Yes (via `implement-Len-for-standard_collection_types`) | Yes (via `implement-Len-for-standard_collection_types`) | |
| `implement-Len-for-standard_collection_types`     | Causes `Len` to be implemented for standard collection types `BTreeMap`, `BTreeSet`, ..., `Vec`, `VecDeque`, and `String` | Yes | Yes | `implement-Len-for-String` |
| `implement-Len-for-standard_ffi_types`            | Causes `Len` to be implemented for standard FFI types `CStr`, `CString` | Yes | Yes | |
| `implement-Len-for-standard_path_types`           | Causes `Len` to be implemented for standard path types `Path`, `PathBuf` | Yes | Yes | |
| `implement-Len-for-standard_process_types`        | Causes `Len` to be implemented for standard process types `CommandArgs`, `CommandEnvs` | **No** | Yes | |
| `implement-ToF64-for-built_ins`                   | Causes `ToF64` to be implemented for built-in types `i8`, ..., `i128`, `u8`, ..., `u128`, `isize`, `usize`, `f32`, `f64` | Yes | Yes | |
| `implement-ToI128-for-built_ins`                  | Causes `ToI128` to be implemented for built-in types `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64` | Yes | Yes | |
| `implement-ToI16-for-built_ins`                   | Causes `ToI16` to be implemented for built-in types `i8`, `i16`, `u8` | Yes | Yes | |
| `implement-ToI32-for-built_ins`                   | Causes `ToI32` to be implemented for built-in types `i8`, `i16`, `i32`, `u8`, `u16` | Yes | Yes | |
| `implement-ToI64-for-built_ins`                   | Causes `ToI64` to be implemented for built-in types `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32` | Yes | Yes | |
| `implement-ToISize-for-built_ins`                 | Causes `ToISize` to be implemented for built-in types `isize`, `i8`, and all other numeric types that can, depending on the target architecture, be represented in `isize` without loss | Yes | Yes | |
| `implement-ToU128-for-built_ins`                  | Causes `ToU128` to be implemented for built-in types `u8`, `u16`, `u32`, `u64`, `u128` | Yes | Yes | |
| `implement-ToU16-for-built_ins`                   | Causes `ToU16` to be implemented for built-in types `u8`, `u16` | Yes | Yes | |
| `implement-ToU32-for-built_ins`                   | Causes `ToU32` to be implemented for built-in types `u8`, `u16`, `u32` | Yes | Yes | |
| `implement-ToU64-for-ThreadId`                    | Causes `ToU64` to be implemented for standard thread type `ThreadId`; incompatible with `nostd` | **No** | Yes | |
| `implement-ToU64-for-built_ins`                   | Causes `ToU64` to be implemented for built-in types `u8`, `u16`, `u32`, `u64` | Yes | Yes | |
| `implement-ToUSize-for-built_ins`                 | Causes `ToUSize` to be implemented for built-in types `usize`, `u8`, and all other unsigned numeric types that can, depending on the target architecture, be represented in `usize` without loss | Yes | Yes | |
| `implement-Zero-for-built_ins`                    | Causes `Zero` to be implemented for built-in types `i8`, ..., `i128`, `u8`, ..., `u128`, `isize`, `usize`, `f32`, `f64`, `char` | Yes | Yes | |


### Special Features

| Name                                              | Effect | Is `"default"`? | Is `"full"`? | Dependent feature(s) |
| ------------------------------------------------- | ------ | --------------- | ------------ | -------------------- |
| `experimental-exact_size_is_empty`                | Causes the experimental feature `exact_size_is_empty` to be enabled | **No** | Yes (via `implement-IsEmpty-for-standard_process_types`) | |
| `full`                                            | Enables the broader set of foreign-type implementations (including process/time IsEmpty, process Len, and `ToU64` for `ThreadId`) | **No** | Yes | |
| `nostd`                                           | Builds the crate without the standard library           | **No** | **No** | |
| `null-feature`                                    | Has no effect; provided for driver-script compatibility | **No** | **No** | |

The `"full"` feature enables the broader standard-library implementation set
and requires nightly Rust because it enables the experimental
`exact_size_is_empty` feature. That set includes `ToU64` for
`std::thread::ThreadId`, which is selected by
`"implement-ToU64-for-ThreadId"`.

The `"implement-ToU64-for-ThreadId"` feature may also be enabled on its own,
and does not itself require nightly Rust: on stable it derives the value from
the `Debug` form of the `ThreadId`; on a nightly toolchain (Rust 1.76+) it
uses `ThreadId::as_u64()`. It cannot be combined with `"nostd"`; doing so is
a compile-time error.

The `"nostd"` feature is intended for builds that do not use the standard
library.


## Components

This crate defines traits only; no enumerations, functions, macros, or structures are defined at this time.


### Traits

The following traits are defined (marker traits provide no methods):

| Trait                                                                                | Method                          |
| ------------------------------------------------------------------------------------ | ------------------------------- |
| [`AsF64`](https://docs.rs/base-traits/latest/base_traits/trait.AsF64.html)           | `fn as_f64(&self) -> f64`       |
| [`AsI128`](https://docs.rs/base-traits/latest/base_traits/trait.AsI128.html)         | `fn as_i128(&self) -> i128`     |
| [`AsI32`](https://docs.rs/base-traits/latest/base_traits/trait.AsI32.html)           | `fn as_i32(&self) -> i32`       |
| [`AsI64`](https://docs.rs/base-traits/latest/base_traits/trait.AsI64.html)           | `fn as_i64(&self) -> i64`       |
| [`AsISize`](https://docs.rs/base-traits/latest/base_traits/trait.AsISize.html)       | `fn as_isize(&self) -> isize`   |
| [`AsStr`](https://docs.rs/base-traits/latest/base_traits/trait.AsStr.html)           | `fn as_str(&self) -> &str`      |
| [`AsU128`](https://docs.rs/base-traits/latest/base_traits/trait.AsU128.html)         | `fn as_u128(&self) -> u128`     |
| [`AsU32`](https://docs.rs/base-traits/latest/base_traits/trait.AsU32.html)           | `fn as_u32(&self) -> u32`       |
| [`AsU64`](https://docs.rs/base-traits/latest/base_traits/trait.AsU64.html)           | `fn as_u64(&self) -> u64`       |
| [`AsUSize`](https://docs.rs/base-traits/latest/base_traits/trait.AsUSize.html)       | `fn as_usize(&self) -> usize`   |
| [`Infinity`](https://docs.rs/base-traits/latest/base_traits/trait.Infinity.html)     | `fn infinity() -> Self`         |
| [`Integer`](https://docs.rs/base-traits/latest/base_traits/trait.Integer.html)       | *(marker; no methods)*          |
| [`IsDefault`](https://docs.rs/base-traits/latest/base_traits/trait.IsDefault.html)   | `fn is_default(&self) -> bool`  |
| [`IsEmpty`](https://docs.rs/base-traits/latest/base_traits/trait.IsEmpty.html)       | `fn is_empty(&self) -> bool`    |
| [`IsInfinity`](https://docs.rs/base-traits/latest/base_traits/trait.IsInfinity.html) | `fn is_infinity(&self) -> bool` |
| [`IsNAN`](https://docs.rs/base-traits/latest/base_traits/trait.IsNAN.html)           | `fn is_nan(&self) -> bool`      |
| [`IsNegative`](https://docs.rs/base-traits/latest/base_traits/trait.IsNegative.html) | `fn is_negative(&self) -> bool` |
| [`IsPositive`](https://docs.rs/base-traits/latest/base_traits/trait.IsPositive.html) | `fn is_positive(&self) -> bool` |
| [`IsZero`](https://docs.rs/base-traits/latest/base_traits/trait.IsZero.html)         | `fn is_zero(&self) -> bool`     |
| [`Len`](https://docs.rs/base-traits/latest/base_traits/trait.Len.html)               | `fn len(&self) -> usize`        |
| [`Numeric`](https://docs.rs/base-traits/latest/base_traits/trait.Numeric.html)       | *(marker; no methods)*          |
| [`Real`](https://docs.rs/base-traits/latest/base_traits/trait.Real.html)             | *(marker; no methods)*          |
| [`Scalar`](https://docs.rs/base-traits/latest/base_traits/trait.Scalar.html)         | *(marker; no methods)*          |
| [`Signed`](https://docs.rs/base-traits/latest/base_traits/trait.Signed.html)         | *(marker; no methods)*          |
| [`ToF64`](https://docs.rs/base-traits/latest/base_traits/trait.ToF64.html)           | `fn to_f64(&self) -> f64`       |
| [`ToI128`](https://docs.rs/base-traits/latest/base_traits/trait.ToI128.html)         | `fn to_i128(&self) -> i128`     |
| [`ToI16`](https://docs.rs/base-traits/latest/base_traits/trait.ToI16.html)           | `fn to_i16(&self) -> i16`       |
| [`ToI32`](https://docs.rs/base-traits/latest/base_traits/trait.ToI32.html)           | `fn to_i32(&self) -> i32`       |
| [`ToI64`](https://docs.rs/base-traits/latest/base_traits/trait.ToI64.html)           | `fn to_i64(&self) -> i64`       |
| [`ToISize`](https://docs.rs/base-traits/latest/base_traits/trait.ToISize.html)       | `fn to_isize(&self) -> isize`   |
| [`ToU128`](https://docs.rs/base-traits/latest/base_traits/trait.ToU128.html)         | `fn to_u128(&self) -> u128`     |
| [`ToU16`](https://docs.rs/base-traits/latest/base_traits/trait.ToU16.html)           | `fn to_u16(&self) -> u16`       |
| [`ToU32`](https://docs.rs/base-traits/latest/base_traits/trait.ToU32.html)           | `fn to_u32(&self) -> u32`       |
| [`ToU64`](https://docs.rs/base-traits/latest/base_traits/trait.ToU64.html)           | `fn to_u64(&self) -> u64`       |
| [`ToUSize`](https://docs.rs/base-traits/latest/base_traits/trait.ToUSize.html)       | `fn to_usize(&self) -> usize`   |
| [`Unsigned`](https://docs.rs/base-traits/latest/base_traits/trait.Unsigned.html)     | *(marker; no methods)*          |
| [`Zero`](https://docs.rs/base-traits/latest/base_traits/trait.Zero.html)             | `fn zero() -> Self`             |


## Examples

The purpose of all traits provided in this package is to enhance generic programming.

For example, you may be working on a subsystem that needs to abstract numbers into strong types, but you also need to build algorithms that can work generically on such types as well as on basic types. You might, therefore, implement `ToF64` on your `Price` type and then go ahead and build your generic algorithm(s) around `ToF64`, as in the following (the complete program is the [**price-to_f64**](./examples/price-to_f64.rs) example, listed in **EXAMPLES.md**; run it with `cargo run --example price-to_f64`):

```Rust
use base_traits::ToF64;

/// Trivial implementation of a price type using exact types (integers)
#[derive(Debug)]
struct Price {
    dollars : u32,
    cents :   u8,
}

impl ToF64 for Price {
    fn to_f64(&self) -> f64 {
        self.dollars as f64 + (self.cents as f64 / 100.0)
    }
}


/// Trivial implementation for calculation of mean and std-deviation
fn calc_mean_and_stddev<'a, F, I>(
    i : I
) -> Option<(
    f64, // mean
    f64, // stddev
)>
where
    F : ToF64 + 'a,
    I : Iterator<Item = &'a F>,
{
    let values = i.map(|v| v.to_f64()).collect::<Vec<_>>();

    if values.is_empty() {
        None
    } else {
        let n = values.len() as f64;
        let sum : f64 = values.iter().sum();
        let mean = sum / n;
        let ss : f64 = values.iter().map(|v| (v - mean)).map(|v| v*v).sum();
        let var = ss / n;
        let stddev = var.sqrt();

        Some((mean, stddev))
    }
}


fn main() {
    {
        let raw_values = vec![
            10.0,
            10.1,
            10.2,
            10.3,
        ];

        let (raw_mean, raw_stddev) = calc_mean_and_stddev(raw_values.iter()).unwrap();

        println!("for {raw_values:?}, mean={raw_mean}, std-dev={raw_stddev}");
    }

    {
        let prices = vec![
            Price { dollars: 10, cents : 0 },
            Price { dollars: 10, cents : 1 },
            Price { dollars: 10, cents : 2 },
            Price { dollars: 10, cents : 3 },
        ];

        let (price_mean, price_stddev) = calc_mean_and_stddev(prices.iter()).unwrap();

        println!("for {prices:?}, mean={price_mean}, std-dev={price_stddev}");
    }
}

```


### In use in other projects

Other Synesis crates build on these traits:

* [**shwild.Rust**](https://github.com/synesissoftware/shwild.Rust) — with its `"flexible-flags-type"` feature, the `shwild_matches!`, `assert_shwild_matches!`, and `assert_shwild_not_matches!` macros accept a flags argument of any type that implements `AsI64`, so callers can pass their own flag enumerations or newtypes rather than a bare `i64`;
* [**test_help-rs**](https://github.com/synesissoftware/test_help-rs) — `TestableAsF64` is implemented for every `ToF64 + Debug` type, so approximate-comparison assertions work with your own strong types, and `assert_as_str_eq!` / `assert_as_str_ne!` compare anything that implements `AsStr`;
* [**to-be.Rust**](https://github.com/synesissoftware/to-be.Rust) — a blanket implementation, `impl<T : AsStr> Truthy for T`, so that any type that can present itself as a `&str` can be tested for "truthiness" (`"yes"`, `"no"`, etc.) without further code;


## Project Information

### Where to get help

* [GitHub Page](https://github.com/synesissoftware/base-traits "GitHub Page");
* [GitHub Issues](https://github.com/synesissoftware/base-traits/issues "GitHub Issues") for defect reports and questions;
* [API documentation](https://docs.rs/base-traits "API documentation on docs.rs");


### Contribution guidelines

Defect reports, feature requests, and pull requests are welcome on https://github.com/synesissoftware/base-traits.


### Minimum Supported Rust Version (MSRV)

The declared Minimum Supported Rust Version (MSRV) for **base-traits** is **1.74**.

This MSRV guarantee applies to the library crate itself, its runtime dependencies (`[dependencies]`), and its build dependencies (`[build-dependencies]`). Downstream consumers compiling this crate as a dependency are guaranteed that it builds cleanly on the declared MSRV toolchain.

There are currently no development dependencies (`[dev-dependencies]`). Should any be added, they are never fetched or compiled by downstream consumers and do not affect the library's MSRV guarantee, even if they require a newer Rust toolchain for local development.


### Dependencies


#### Efferent (fan-out)

Libraries upon which **base-traits** depends:


##### Runtime Dependencies

There are no runtime dependencies (aka "normal dependencies") on other crates.


##### Build Dependencies

Libraries used only when building **base-traits** (not required by downstream consumers):

* [**bt-rs**](https://github.com/synesissoftware/bt-rs) — [`rustc::compiler_version()`](https://docs.rs/bt-rs/latest/bt_rs/rustc/fn.compiler_version.html) in **build.rs** to detect the compiler version and set the `sis_rustc_1_76_or_newer`, `sis_rustc_1_79_or_newer` (for use of `CStr::count_bytes()`), and `sis_rustc_nightly` cfgs;


##### Development Dependencies

There are no development dependencies on other crates.


#### Afferent (fan-in)

Projects that depend on **base-traits**:

* [**collect-rs**](https://github.com/synesissoftware/collect-rs);
* [**Diagnosticism.Rust**](https://github.com/synesissoftware/Diagnosticism.Rust);
* [**shwild.Rust**](https://github.com/synesissoftware/shwild.Rust);
* [**test_help-rs**](https://github.com/synesissoftware/test_help-rs);
* [**to-be.Rust**](https://github.com/synesissoftware/to-be.Rust);


### Related projects

* [**bt-rs**](https://github.com/synesissoftware/bt-rs) — build-time toolchain detection, used by **build.rs** (see [Build Dependencies](#build-dependencies));
* [**test_help-rs**](https://github.com/synesissoftware/test_help-rs) — test helpers built on **base-traits** traits (see [In use in other projects](#in-use-in-other-projects));


### License

**base-traits** is released under the 3-clause BSD license. See [LICENSE](./LICENSE) for details.


<!-- ########################### end of file ########################### -->
