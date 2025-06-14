//! The (converted) original exomizer core functions.
//!
//! The inner workings of this module contains converted (c2rust) and minimal adapted code from the
//! [exomizer project](https://bitbucket.org/magli143/exomizer/wiki/Home).
//! See [converted] for for information about that.
//!
//! Thanks to Magnus Lind for it.
//!
//! It is `no_std` but requires `alloc`.

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools
)]

pub mod converted;
mod crunch;
mod crunch_options;
mod decrunch;
mod decrunch_options;
mod flags;
mod output;
mod replacement;

pub use crate::original::crunch::{
    CrunchInfo, Crunched, CrunchedMulti, crunch_multi_raw, crunch_raw,
};
pub use crate::original::crunch_options::{
    CrunchOptions, CrunchOptionsBuilder, CrunchOptionsError,
};
pub use crate::original::decrunch::decrunch_raw;
pub use crate::original::decrunch_options::{
    DecrunchOptions, DecrunchOptionsBuilder, DecrunchOptionsError,
};
pub use crate::original::flags::{ProtoFlags, TraitFlags};
pub use crate::original::output::Output;
#[cfg(feature = "std")]
pub use crate::original::output::feature_std::{Level, Logger, LoggerDump};
