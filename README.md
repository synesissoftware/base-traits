# base-traits <!-- omit in toc -->

base traits, for Rust


## Introduction

This crate defines general-purpose `trait`s that:
* seem obvious and yet are missing from the standard library (e.g. `IsEmpty`, `Len`); or
* provide a basis to introduce specific functionality to allow for other crates to interoperate;


## Table of Contents <!-- omit in toc -->

- [Introduction](#introduction)
- [Installation](#installation)
- [Components](#components)
	- [Enumerations](#enumerations)
	- [Features](#features)
	- [Functions](#functions)
	- [Macros](#macros)
	- [Structures](#structures)
	- [Traits](#traits)
- [Examples](#examples)
- [Project Information](#project-information)
	- [Where to get help](#where-to-get-help)
	- [Contribution guidelines](#contribution-guidelines)
	- [Dependencies](#dependencies)
	- [Related projects](#related-projects)
	- [License](#license)


## Installation

Reference in **Cargo.toml** in the usual way:

```toml
base-traits = { version = "0" }
```


## Components

### Enumerations

None defined at this time.


### Features

The following crate features are defined:

| Name                                                | Effect                                | Is `"default"`? | Dependent feature(s)                  |
| --------------------------------------------------- | ------------------------------------- | --------------- | ------------------------------------- |
| `"experimental-exact_size_is_empty"`                | Causes the experimental feature `"exact_size_is_empty"` to be enabled | **No** | |
| `"implement-AsF64-for-built_ins"`                   | Causes `AsF64` to be implemented for built-in type `f64` | Yes | |
| `"implement-AsI128-for-built_ins"`                  | Causes `AsI128` to be implemented for built-in type `i128` | Yes | |
| `"implement-AsI32-for-built_ins"`                   | Causes `AsI32` to be implemented for built-in type `i32` | Yes | |
| `"implement-AsI64-for-built_ins"`                   | Causes `AsI64` to be implemented for built-in type `i64` | Yes | |
| `"implement-AsISize-for-built_ins"`                 | Causes `AsISize` to be implemented for built-in type `isize` | Yes | |
| `"implement-AsStr-for-built_ins"`                   | Causes `AsStr` to be implemented for built-in type `str` | Yes | |
| `"implement-AsStr-for-standard_collection_types"`   | Causes `AsStr` to be implemented for standard collection type `String` | Yes | |
| `"implement-AsU128-for-built_ins"`                  | Causes `AsU128` to be implemented for built-in type `u128` | Yes | |
| `"implement-AsU32-for-built_ins"`                   | Causes `AsU32` to be implemented for built-in type `u32` | Yes | |
| `"implement-AsU64-for-built_ins"`                   | Causes `AsU64` to be implemented for built-in type `u64` | Yes | |
| `"implement-AsUSize-for-built_ins"`                 | Causes `AsUSize` to be implemented for built-in type `usize` | Yes | |
| `"implement-Infinity-for-built_ins"`                | Causes `Infinity` to be implemented for built-in types `f32`, `f64` | Yes | |
| `"implement-IsEmpty-for-built_ins"`                 | Causes `IsEmpty` to be implemented for built-in types `str`, `[T; N]`, `[T]` | Yes | |
| `"implement-IsEmpty-for-standard_collection_types"` | Causes `IsEmpty` to be implemented for standard collection types `BTreeMap`, `BTreeSet`, ..., `Vec`, `VecDeque`, and `String` | Yes | |
| `"implement-IsEmpty-for-standard_ffi_types"`        | Causes `IsEmpty` to be implemented for standard FFI types `CStr`, `CString` | Yes | |
| `"implement-IsEmpty-for-standard_path_types"`       | Causes `IsEmpty` to be implemented for standard path types `Path`, `PathBuf` | Yes | |
| `"implement-IsEmpty-for-standard_process_types"`    | Causes `IsEmpty` to be implemented for standard process types `CommandArgs`, `CommandEnvs` | **No** | `experimental-exact_size_is_empty` |
| `"implement-IsEmpty-for-standard_range_types"`      | Causes `IsEmpty` to be implemented for standard range types `Range`, `RangeFrom`, etc. | Yes | |
| `"implement-IsEmpty-for-standard_time_types"`       | Causes `IsEmpty` to be implemented for standard time type `Duration` | **No**  | |
| `"implement-IsInfinity-for-built_ins"`              | Causes `IsInfinity` to be implemented for built-in types `f32`, `f64` | Yes | |
| `"implement-IsNAN-for-built_ins"`                   | Causes `IsNAN` to be implemented for built-in types `f32`, `f64` | Yes | |
| `"implement-IsZero-for-built_ins"`                  | Causes `IsZero` to be implemented for built-in types `i8`, ..., `i128`, `u8`, ..., `u128`, `isize`, `usize`, `f32`, `f64`, `char` | Yes | |
| `"implement-IsZero-for-standard_process_types"`     | Causes `IsZero` to be implemented for standard process type `ExitStatus` | Yes | |
| `"implement-IsZero-for-standard_time_types"`        | Causes `IsZero` to be implemented for standard time type `Duration` | Yes | |
| `"implement-Len-for-built_ins"`                     | Causes `Len` to be implemented for built-in types `str`, `[T; N]`, `[T]` | Yes | |
| `"implement-Len-for-standard_collection_types"`     | Causes `Len` to be implemented for standard collection types `BTreeMap`, `BTreeSet`, ..., `Vec`, `VecDeque`, and `String` | Yes | |
| `"implement-Len-for-standard_ffi_types"`            | Causes `Len` to be implemented for standard FFI types `CStr`, `CString` | Yes | |
| `"implement-Len-for-standard_path_types"`           | Causes `Len` to be implemented for standard path types `Path`, `PathBuf` | Yes | |
| `"implement-Len-for-standard_process_types"`        | Causes `Len` to be implemented for standard process types `CommandArgs`, `CommandEnvs` | **No** | |
| `"implement-ToF64-for-built_ins"`                   | Causes `ToF64` to be implemented for built-in types `i8`, ..., `i128`, `u8`, ..., `u128`, `isize`, `usize`, `f32`, `f64` | Yes | |
| `"implement-ToISize-for-built_ins"`                 | Causes `ToISize` to be implemented for built-in types `isize`, `i8`, and all numeric types that, dependent on architecture, that can be represented in `isize` without loss | Yes | |
| `"implement-ToUSize-for-built_ins"`                 | Causes `ToUSize` to be implemented for built-in types `usize`, `u8`, and all unsigned numeric types that, dependent on architecture, that can be represented in `usize` without loss | Yes | |
| `"implement-Zero-for-built_ins"`                    | Causes `Zero` to be implemented for built-in types `i8`, ..., `i128`, `u8`, ..., `u128`, `isize`, `usize`, `f32`, `f64`, `char` | Yes | |


### Functions

None defined at this time.


### Macros

None defined at this time.


### Structures

None defined at this time.


### Traits

The following traits are defined:

* `AsF64` - provides (non-mutating) instance method `#as_f64() : f64`;
* `AsI128` - provides (non-mutating) instance method `#as_i128() : i128`;
* `AsI32` - provides (non-mutating) instance method `#as_i32() : i32`;
* `AsI64` - provides (non-mutating) instance method `#as_i64() : i64`;
* `AsISize` - provides (non-mutating) instance method `#as_isize() : isize`;
* `AsStr` - provides (non-mutating) instance method `#as_str() : &str`;
* `AsU128` - provides (non-mutating) instance method `#as_u128() : u128`;
* `AsU32` - provides (non-mutating) instance method `#as_u32() : u32`;
* `AsU64` - provides (non-mutating) instance method `#as_u64() : u64`;
* `AsUSize` - provides (non-mutating) instance method `#as_usize() : usize`;
* `Infinity` - provides method `::infinity() : Self`;
* `IsEmpty` - provides (non-mutating) instance method `#is_empty() : bool`;
* `IsInfinity` - provides (non-mutating) instance method `#is_infinity() : bool`;
* `IsNAN` - provides (non-mutating) instance method `#is_nan() : bool`;
* `IsZero` - provides (non-mutating) instance method `#is_zero() : bool`;
* `Len` - provides (non-mutating) instance method `#len() : usize`;
* `ToF64` - provides (non-mutating) instance method `#to_f64() : f64`;
* `AsU32` - provides (non-mutating) instance method `#as_u32() : u32`;
* `AsU64` - provides (non-mutating) instance method `#as_u64() : u64`;
* `AsU128` - provides (non-mutating) instance method `#as_u128() : u128`;
* `ToISize` - provides (non-mutating) instance method `#to_isize() : isize`;
* `ToUSize` - provides (non-mutating) instance method `#to_usize() : usize`;
* `Zero` - provides class method `::zero() : Self`;


## Examples

The purpose of all traits provided in this package is to enhance generic programming.

For example, you may be working on a subsystem that needs to abstract numbers into strong types, but you also need to build algorithms that can work generically on such types as well as on basic types. You might, therefore, implement `ToF64` on your `Price` type and then go ahead and build your generic algorithm(s) around `ToF64`, as in:

```Rust

use base_traits::ToF64;

/// Trivial implementation of a price type using exact types (integers)
#[derive(Debug)]
struct Price {
    dollars : u32,
    cents : u8,
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
    let values = i.map(|v| v.to_f64()).collect::<Vec::<_>>();

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
            // insert list
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
            // insert list
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


## Project Information

### Where to get help

[GitHub Page](https://github.com/synesissoftware/base-traits "GitHub Page")


### Contribution guidelines

Defect reports, feature requests, and pull requests are welcome on https://github.com/synesissoftware/base-traits.


### Dependencies

There are no dependencies on other crates.


### Related projects

* [**Diagnosticism.Rust**](https://github.com/synesissoftware/Diagnosticism.Rust);
* [**shwild.Rust**](https://github.com/synesissoftware/shwild.Rust);
* [**test_help-rs**](https://github.com/synesissoftware/test_help-rs);


### License

**base-traits** is released under the 3-clause BSD license. See [LICENSE](./LICENSE) for details.


<!-- ########################### end of file ########################### -->

