# base-traits Changes <!-- omit in toc -->


## 0.1.1 - 15th July 2026

* fixed wrong line in NEWS.md;
* fixing (internal) macro name;


## 0.1.0 - 10th July 2026

* first minor release;
* fixed `Box`/`Rc` trait implementations being excluded under `cargo test` (incorrect `cfg` gating introduced with `"nostd"`);
* enhanced **README.md** (badges; dependency / afferent project sections);
* added **CHANGES.md**;
* added **EXAMPLES.md**;
* added **NEWS.md**;
* added **TODO.md**;
* updated **LICENSE** copyright year;
* renamed **.rustfmt.toml** => **rustfmt.toml**;
* canonicalised **.gitattributes** and **.gitignore**;
* added CI (GitHub Actions);
* various other boilerplate fixes;
* added **`[build-dependencies]`** **bt-rs** and **build.rs** (`rustc_1_79_or_newer` cfg);
* improved `declare_and_publish!()`;


## 0.0.13 - 2nd April 2025

* added feature `"nostd"`, which enables crate feature `no_std` if specified;


## 0.0.12 - 2nd April 2025

* added general feature `"null-feature"`;


## 0.0.11 - 8th January 2025

* added traits `ToI16`, `ToI32`, `ToI64`, `ToI128`, `ToU16`, `ToU32`, `ToU64`, `ToU128`;
* minor documentation changes to extant `To***` traits;


## 0.0.10 - 9th November 2024

* added traits `Integer`, `Numeric`, `Real`, `Scalar`, `Signed`, `Unsigned`;


## 0.0.9 - 4th November 2024

* added traits `AsI128`, `AsI32`, `AsI64`, `AsU128`, `AsU32`, `AsU64`;


## 0.0.8 - 1st November 2024

* completed boilerplate files;
* added example **price-to_f64**;
* tidied use of `"experimental-exact_size_is_empty"`, `"implement-IsEmpty-for-standard_process_types"`;


## 0.0.7 - 1st November 2024

* added documentation for all traits regarding their implementation on other (foreign) types;
* fixed built-in implementations for `ToISize`;
* fixed missing documentation markup for `AsUSize` and `ToUSize`;


## 0.0.6 - 31st October 2024

* further application of `#[inline]`;


## 0.0.5 - 31st October 2024

* added traits `AsISize`, `ToISize`;


## 0.0.4 - 20th September 2024

* added traits `AsUSize`, `ToUSize`;
* file format canonicalisation;


## 0.0.3 - 17th September 2024

* added traits `Infinity`, `IsInfinity`, `IsNAN`;


## 0.0.2 - 17th September 2024

* added (missing) implementations of traits for `Box<>` and `std::rc::Rc<>`;


## 0.0.1 - 13th September 2024

* added traits `AsF64`, `AsStr`, `IsDefault`, `IsEmpty`, `IsZero`, `Len`, `ToF64`, `Zero`;


## 0.0.0 - 10th September 2024

* first release;


<!-- ########################### end of file ########################### -->
