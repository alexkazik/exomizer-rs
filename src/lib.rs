#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![deny(unreachable_pub)]
#![deny(clippy::redundant_pub_crate)]
#![deny(clippy::let_underscore_untyped)]
#![allow(rustdoc::redundant_explicit_links)]
// deny most but but not all pedantic lints
#![deny(clippy::pedantic)]
#![allow(clippy::verbose_bit_mask)]

//! Exomizer is a program that compresses files in a way that tries to be as efficient as possible
//! but still allows them to be decompressed in environments where CPU speed and RAM are limited.
//!
//! While the [original project](https://bitbucket.org/magli143/exomizer/wiki/Home) has a binary
//! which not only allows de-/crunching it is also capable to create self-extracting programs for
//! some popular 8-bit computers.
//!
//! This library only supports de-/crunching in "raw" mode. There are two modules:
//! - [original](crate::original): Routines for de-/crunching, converted from the original C code.
//!   Requires `alloc`.
//! - [simple](crate::simple): Routines for decrunching only, also only a subset of all possible
//!   parameters are supported (is `no_std`).
//!
//! # Features
//!
//! For [original](crate::original):
//! - `alloc` (default): enables the original routines.
//! - `std`: provides logging to io (incl. stdio) (you can write your own without it).
//!
//! For [simple](crate::simple):
//! - `clz` (default): enable the use of clz ([`usize::leading_zeros`](core::primitive::usize::leading_zeros)),
//!   please diable if using simple and your cpu does not support it
//!   (the function is always available but emulated on cpus which doesn't have it).
//!
//! Unless `std` is activates the library is `no_std`.
//!
//! ## Usage
//!
//! With defaults (`alloc` and `clz`):
//! ```toml
//! [dependencies]
//! lzss = "0.5"
//! ```
//!
//! Without `alloc`:
//! ```toml
//! [dependencies]
//! lzss = { version = "0.5", default-features = false, features = ["clz"] }
//! ```
//!

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
pub mod original;

pub mod simple;
