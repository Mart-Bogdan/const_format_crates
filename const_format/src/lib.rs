//! Compile-time string formatting.
//!
//! This crate provides types and macros for formatting strings at compile-time.
//!
//! # Rust versions
//!
//! There are some features that require Rust 1.46.0,
//! some that require Rust 1.51.0,
//! and others that require Rust nightly,
//! the sections below describe the features that are available for each version.
//!
//! ### Rust 1.46.0
//!
//! These macros are the only things available in Rust 1.46.0:
//!
//! - [`concatcp`]:
//! Concatenates `integers`, `bool`, `char`, and `&str` constants into a `&'static str` constant.
//!
//! - [`formatcp`]:
//! [`format`]-like formatting which takes `integers`, `bool`, `char`, and `&str` constants,
//! and emits a `&'static str` constant.
//!
//! - [`str_get`]:
//! Indexes a `&'static str` constant, returning `None` when the index is out of bounds.
//!
//! - [`str_index`]:
//! Indexes a `&'static str` constant.
//!
//! - [`str_repeat`]:
//! Creates a `&'static str` by repeating a `&'static str` constant `times` times.
//!
//! - [`str_splice`]:
//! Replaces a substring in a `&'static str` constant.
//!
//!
//! ### Rust 1.51.0
//!
//! By enabling the "const_generics" feature, you can use these macros:
//!
//! - [`map_ascii_case`]:
//! Converts a `&'static str` constant to a different casing style,
//! determined by a [`Case`] argument.
//!
//! - [`str_replace`]:
//! Replaces all the instances of a pattern in a `&'static str` constant with
//! another `&'static str` constant.
//!
//! ### Rust 1.57.0
//!
//! The "assertcp" feature enables the [`assertcp`], [`assertcp_eq`],
//! and [`assertcp_ne`] macros.
//! These macros are like the standard library assert macros,
//! but evaluated at compile-time,
//! with the limitation that they can only have primitive types as arguments
//! (just like [`concatcp`] and [`formatcp`]).
//!
//! ### Rust nightly
//!
//! By enabling the "fmt" feature, you can use a [`std::fmt`]-like API.
//!
//! This requires the nightly compiler because it uses mutable references in const fn,
//! which have not been stabilized as of writing these docs.
//!
//! All the other features of this crate are implemented on top of the [`const_format::fmt`] API:
//!
//! - [`concatc`]:
//! Concatenates many standard library and user defined types into a `&'static str` constant.
//!
//! - [`formatc`]:
//! [`format`]-like macro that can format many standard library and user defined types into
//! a `&'static str` constant.
//!
//! - [`writec`]:
//! [`write`]-like macro that can format many standard library and user defined types
//! into a type that implements [`WriteMarker`].
//!
//! The "derive" feature enables the [`ConstDebug`] macro,
//! and the "fmt" feature.<br>
//! [`ConstDebug`] derives the [`FormatMarker`] trait,
//! and implements an inherent `const_debug_fmt` method for compile-time debug formatting.
//!
//! The "assertc" feature enables the [`assertc`], [`assertc_eq`], [`assertc_ne`] macros,
//! and the "fmt" feature.<br>
//! These macros are like the standard library assert macros, but evaluated at compile-time.
//! # Examples
//!
//! ### Concatenation of primitive types
//!
//! This example works in Rust 1.46.0.
//!
//! ```rust
//! use const_format::concatcp;
//!
//! const NAME: &str = "Bob";
//! const FOO: &str = concatcp!(NAME, ", age ", 21u8,"!");
//!
//! assert_eq!(FOO, "Bob, age 21!");
//! ```
//!
//! ### Formatting primitive types
//!
//! This example works in Rust 1.46.0.
//!
//! ```rust
//! use const_format::formatcp;
//!
//! const NAME: &str = "John";
//!
//! const FOO: &str = formatcp!("{NAME}, age {}!", compute_age(NAME));
//!
//! assert_eq!(FOO, "John, age 24!");
//!
//! # const fn compute_age(s: &str) -> usize { s.len() * 6 }
//!
//! ```
//!
//! ### Formatting custom types
//!
//! This example demonstrates how you can use the [`ConstDebug`] derive macro,
//! and then format the type into a `&'static str` constant.
//!
//! This example requires Rust nightly, and the "derive" feature.
//!
#![cfg_attr(feature = "derive", doc = "```rust")]
#![cfg_attr(not(feature = "derive"), doc = "```ignore")]
//! #![feature(const_mut_refs)]
//!
//! use const_format::{ConstDebug, formatc};
//!
//! #[derive(ConstDebug)]
//! struct Message{
//!     ip: [Octet; 4],
//!     value: &'static str,
//! }
//!
//! #[derive(ConstDebug)]
//! struct Octet(u8);
//!
//! const MSG: Message = Message{
//!     ip: [Octet(127), Octet(0), Octet(0), Octet(1)],
//!     value: "Hello, World!",
//! };
//!
//! const FOO: &str = formatc!("{:?}", MSG);
//!
//! assert_eq!(
//!     FOO,
//!     "Message { ip: [Octet(127), Octet(0), Octet(0), Octet(1)], value: \"Hello, World!\" }"
//! );
//!
//! ```
//!
//! ### Formatted const assertions
//!
//! This example demonstrates how you can use the [`assertcp_ne`] macro to
//! do compile-time inequality assertions with formatted error messages.
//!
//! This requires the "assertcp" feature,
//! because using the `panic` macro at compile-time requires Rust 1.57.0.
//!
#![cfg_attr(feature = "assertcp", doc = "```compile_fail")]
#![cfg_attr(not(feature = "assertcp"), doc = "```ignore")]
//! #![feature(const_mut_refs)]
//!
//! use const_format::assertcp_ne;
//!
//! macro_rules! check_valid_pizza{
//!     ($user:expr, $topping:expr) => {
//!         assertcp_ne!(
//!             $topping,
//!             "pineapple",
//!             "You can't put pineapple on pizza, {}",
//!             $user,
//!         );
//!     }
//! }
//!
//! check_valid_pizza!("John", "salami");
//! check_valid_pizza!("Dave", "sausage");
//! check_valid_pizza!("Bob", "pineapple");
//!
//! # fn main(){}
//! ```
//!
//! This is the compiler output:
//!
//! ```text
//! error[E0080]: evaluation of constant value failed
//!   --> src/lib.rs:178:27
//!    |
//! 20 | check_valid_pizza!("Bob", "pineapple");
//!    |                           ^^^^^^^^^^^ the evaluated program panicked at '
//! assertion failed: `(left != right)`
//!  left: `"pineapple"`
//! right: `"pineapple"`
//! You can't put pineapple on pizza, Bob
//! ', src/lib.rs:20:27
//!
//!
//! ```
//!
//! <div id="macro-limitations"></div>
//!
//! # Limitations
//!
//! All of the macros from `const_format` have these limitations:
//!
//! - The formatting macros that expand to
//! `&'static str`s can only use constants from concrete types,
//! so while a `Type::<u8>::FOO` argument would be fine,
//! `Type::<T>::FOO` would not be (`T` being a type parameter).
//!
//! - Integer arguments must have a type inferrable from context,
//! [more details in the Integer arguments section](#integer-args).
//!
//! - They cannot be used places that take string literals.
//! So `#[doc = "foobar"]` cannot be replaced with `#[doc = concatcp!("foo", "bar") ]`.
//!
//! <span id="integer-args"></span>
//!
//! ### Integer arguments
//!
//! Integer arguments must have a type inferrable from context.
//! so if you only pass an integer literal it must have a suffix.
//!
//! Example of what does compile:
//!
//! ```rust
//! const N: u32 = 1;
//! assert_eq!(const_format::concatcp!(N + 1, 2 + N), "23");
//!
//! assert_eq!(const_format::concatcp!(2u32, 2 + 1u8, 3u8 + 1), "234");
//! ```
//!
//! Example of what does not compile:
//! ```compile_fail
//! assert_eq!(const_format::concatcp!(1 + 1, 2 + 1), "23");
//! ```
//!
//! # Renaming crate
//!
//! All function-like macros from `const_format` can be used when the crate is renamed.
//!
//! The [`ConstDebug`] derive macro has the `#[cdeb(crate = "foo::bar")]` attribute to
//! tell it where to find the `const_format` crate.
//!
//! Example of renaming the `const_format` crate in the Cargo.toml file:
//! ```toml
//! cfmt = {version = "0.*", package = "const_format"}
//! ```
//!
//! # Cargo features
//!
//! - "fmt": Enables the [`std::fmt`]-like API,
//! requires Rust nightly because it uses mutable references in const fn.<br>
//! This feature includes the [`formatc`]/[`writec`] formatting macros.
//!
//! - "derive": implies the "fmt" feature,
//! provides the [`ConstDebug`] derive macro to format user-defined types at compile-time.<br>
//! This implicitly uses the `syn` crate, so clean compiles take a bit longer than without the feature.
//!
//! - "assertc": implies the "fmt" feature,
//! enables the [`assertc`], [`assertc_eq`], and [`assertc_ne`] assertion macros.<br>
//! This feature was previously named "assert",
//! but it was renamed to avoid confusion with the "assertcp" feature.
//!
//! - "assertcp": Requires Rust 1.57.0, implies the "const_generics" feature.
//! Enables the [`assertcp`], [`assertcp_eq`], and [`assertcp_ne`] assertion macros.
//!
//! - "constant_time_as_str": implies the "fmt" feature.
//! An optimization that requires a few additional nightly features,
//! allowing the `as_bytes_alt` methods and `slice_up_to_len_alt` methods to run
//! in constant time, rather than linear time proportional to the truncated part of the slice.
//!
//! - "const_generics": Requires Rust 1.51.0.
//! Enables the macros listed in the [Rust 1.51.0](#rust-1510) section.
//! Also changes the the implementation of the [`concatcp`] and [`formatcp`]
//! macros to use const generics.
//!
//!
//!
//! # No-std support
//!
//! `const_format` is unconditionally `#![no_std]`, it can be used anywhere Rust can be used.
//!
//! # Minimum Supported Rust Version
//!
//! `const_format` requires Rust 1.46.0, because it uses looping an branching in const contexts.
//!
//! Features that require newer versions of Rust, or the nightly compiler,
//! need to be explicitly enabled with cargo features.
//!
//!
//! [`assertc`]: ./macro.assertc.html
//!
//! [`assertc_eq`]: ./macro.assertc_eq.html
//!
//! [`assertc_ne`]: ./macro.assertc_ne.html
//!
//! [`assertcp`]: ./macro.assertcp.html
//!
//! [`assertcp_eq`]: ./macro.assertcp_eq.html
//!
//! [`assertcp_ne`]: ./macro.assertcp_ne.html
//!
//! [`concatcp`]: ./macro.concatcp.html
//!
//! [`formatcp`]: ./macro.formatcp.html
//!
//! [`format`]: https://doc.rust-lang.org/std/macro.format.html
//!
//! [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
//!
//! [`const_format::fmt`]: ./fmt/index.html
//!
//! [`concatc`]: ./macro.concatc.html
//!
//! [`formatc`]: ./macro.formatc.html
//!
//! [`writec`]: ./macro.writec.html
//!
//! [`write`]: https://doc.rust-lang.org/std/macro.write.html
//!
//! [`Formatter`]: ./fmt/struct.Formatter.html
//!
//! [`StrWriter`]: ./fmt/struct.StrWriter.html
//!
//! [`ConstDebug`]: ./derive.ConstDebug.html
//!
//! [`FormatMarker`]: ./marker_traits/trait.FormatMarker.html
//!
//! [`WriteMarker`]: ./marker_traits/trait.WriteMarker.html
//!
//! [`map_ascii_case`]: ./macro.map_ascii_case.html
//!
//! [`Case`]: ./enum.Case.html
//!
//!
//! [`str_get`]: ./macro.str_get.html
//!
//! [`str_index`]: ./macro.str_index.html
//!
//! [`str_repeat`]: ./macro.str_repeat.html
//!
//! [`str_splice`]: ./macro.str_splice.html
//!
//! [`str_replace`]: ./macro.str_replace.html
//!
//! [`str::replace`]: https://doc.rust-lang.org/std/primitive.str.html#method.replace
//!
#![no_std]
#![cfg_attr(feature = "fmt", feature(const_mut_refs))]
#![cfg_attr(feature = "assertc", feature(const_panic))]
#![cfg_attr(
    feature = "constant_time_as_str",
    feature(const_slice_from_raw_parts, const_fn_union,)
)]
#![cfg_attr(feature = "docsrs", feature(doc_cfg))]
#![deny(rust_2018_idioms)]
// This lint is silly
#![allow(clippy::blacklisted_name)]
// This lint is silly
#![allow(clippy::needless_doctest_main)]
#![deny(clippy::missing_safety_doc)]
#![deny(clippy::shadow_unrelated)]
#![deny(clippy::wildcard_imports)]
// All The methods that take self by value are for small Copy types
#![allow(clippy::wrong_self_convention)]
#![deny(missing_docs)]

include! {"const_debug_derive.rs"}

#[macro_use]
mod macros;

mod formatting;

#[cfg(feature = "assertc")]
mod equality;

#[doc(hidden)]
#[cfg(feature = "assertcp")]
#[macro_use]
pub mod for_assert_macros;

mod char_encoding;

mod pargument;

#[cfg(feature = "const_generics")]
mod const_generic_concatcp;

#[cfg_attr(feature = "docsrs", doc(cfg(feature = "fmt")))]
#[cfg(feature = "fmt")]
pub mod utils;

#[doc(hidden)]
#[cfg(any(feature = "fmt", feature = "assertcp"))]
mod slice_cmp;

#[doc(hidden)]
pub mod __hidden_utils;

#[cfg_attr(feature = "docsrs", doc(cfg(feature = "fmt")))]
#[cfg(feature = "fmt")]
pub mod for_examples;

#[cfg_attr(feature = "docsrs", doc(cfg(feature = "fmt")))]
#[cfg(feature = "fmt")]
pub mod marker_traits;

#[cfg(feature = "testing")]
pub mod test_utils;

#[cfg(feature = "testing")]
#[allow(missing_docs)]
pub mod doctests;

#[cfg_attr(feature = "docsrs", doc(cfg(feature = "fmt")))]
#[cfg(feature = "fmt")]
pub mod fmt;

#[cfg(feature = "fmt")]
#[doc(hidden)]
pub mod msg;

#[cfg_attr(feature = "docsrs", doc(cfg(feature = "fmt")))]
#[cfg_attr(not(feature = "fmt"), doc(hidden))]
pub mod wrapper_types;

#[doc(hidden)]
#[cfg(feature = "const_generics")]
pub mod __ascii_case_conv;

#[doc(hidden)]
pub mod __str_methods;

pub use __str_methods::SplicedStr;

#[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_generics")))]
#[cfg(feature = "const_generics")]
pub use __ascii_case_conv::Case;

#[cfg(feature = "fmt")]
#[doc(no_inline)]
pub use crate::fmt::{Error, Formatter, FormattingFlags, Result, StrWriter, StrWriterMut};

#[cfg(feature = "fmt")]
pub use crate::wrapper_types::ascii_str::AsciiStr;

#[cfg(feature = "fmt")]
pub use crate::wrapper_types::sliced::Sliced;

#[cfg_attr(not(feature = "fmt"), doc(hidden))]
pub use crate::wrapper_types::pwrapper::PWrapper;

#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __cf_osRcTFl4A {
    pub use crate::*;
}

#[doc(hidden)]
pub mod pmr {
    pub use {bool, str, u8, usize};

    pub use const_format_proc_macros::{__concatcp_impl, __formatcp_impl, respan_to};

    #[cfg(feature = "fmt")]
    pub use const_format_proc_macros::{__formatc_if_impl, __formatc_impl, __writec_impl};

    #[cfg(feature = "assertcp")]
    pub use const_format_proc_macros::__formatcp_if_impl;

    pub use core::{
        cmp::Reverse,
        convert::identity,
        mem::transmute,
        num::Wrapping,
        ops::Range,
        option::Option::{self, None, Some},
        result::Result::{self, Err, Ok},
    };

    #[cfg(feature = "const_generics")]
    pub use crate::const_generic_concatcp::__priv_concatenate;

    #[cfg(feature = "assertcp")]
    pub use crate::for_assert_macros::{assert_, ConcatArgsIf};

    #[cfg(feature = "fmt")]
    pub use crate::{
        fmt::{ComputeStrLength, Error, Formatter, StrWriter, StrWriterMut, ToResult},
        marker_traits::{
            FormatMarker, IsAFormatMarker, IsAWriteMarker, IsNotStdKind, IsStdKind, WriteMarker,
        },
    };

    pub use crate::{
        formatting::{
            hex_as_ascii, ForEscaping, Formatting, FormattingFlags, LenAndArray, NumberFormatting,
            StartAndArray, FOR_ESCAPING,
        },
        pargument::{PArgument, PConvWrapper, PVariant},
        wrapper_types::PWrapper,
    };
}

#[cfg(all(test, not(feature = "testing")))]
compile_error! { "tests must be run with the \"testing\" feature" }
