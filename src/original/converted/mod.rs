//! This module contains automatically converted C source files.
//!
//! No functions are directly publicly available, this is for documentation purpose only.
//!
//! Source: [exomizer project](https://bitbucket.org/magli143/exomizer/wiki/Home), Commit ba91318.
//!
//! Conversion with c2rust (either 0.18 or 0.20, I've forgotten).
//!
//! I've tried to change little, but the following is different
//! - Logging
//! - Removed file read (the encoding could be read from file)
//! - Unify all converted sources
//! - Replaced ffi/libc types/function with ones in core (or created function in `../replacement`)
//! - Add a lifetime to the options (in order to guarantee that they live at least as long as the ones pointing to)

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    path_statements,
    static_mut_refs,
    unused_assignments,
    unused_must_use,
    unused_variables,
    clippy::all,
    clippy::pedantic,
    clippy::redundant_pub_crate
)]

pub(crate) mod buf;
mod chunkpool;
pub(crate) mod exo_helper;
mod exodec;
mod r#match;
mod optimal;
mod output;
mod progress;
mod radix;
mod search;
pub(crate) mod vec;
