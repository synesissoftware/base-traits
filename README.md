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

T.B.C.


## Components

### Enumerations

None defined at this time.


### Functions

None defined at this time.


### Macros

None defined at this time.


### Structures

None defined at this time.


### Traits

The following traits are defined:

* `AsF64` - provides (non-mutating) instance method `#as_f64() : f64`;
* `AsStr` - provides (non-mutating) instance method `#as_str() : &str`;
* `AsUSize` - provides (non-mutating) instance method `#as_usize() : usize`;
* `Infinity` - provides method `::infinity() : Self`;
* `IsEmpty` - provides (non-mutating) instance method `#is_empty() : bool`;
* `IsInfinity` - provides (non-mutating) instance method `#is_infinity() : bool`;
* `IsNAN` - provides (non-mutating) instance method `#is_nan() : bool`;
* `IsZero` - provides (non-mutating) instance method `#is_zero() : bool`;
* `Len` - provides (non-mutating) instance method `#len() : usize`;
* `ToF64` - provides (non-mutating) instance method `#to_f64() : f64`;
* `ToUSize` - provides (non-mutating) instance method `#to_usize() : usize`;
* `Zero` - provides class method `::zero() : Self`;


## Examples

T.B.C.


## Project Information

T.B.C.


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

