//! Simple (and specialized) decrunch routines.
//!
//! In comparison to [`crate::original::decrunch_raw`] it doesn't require `alloc`, is a bit faster but has
//! limited options, see below.
//!
//! Only a subset of all protos are supported: 9, 13, 25, 29, 41, 45, 57 and 61.
//!
//! Or by flag:
//! - `Impl1Literal`, `FourOffsetTables` and `ReuseOffset` can be chosen at will
//! - `OrderBe` and `AlignStart` must be enabled
//! - `CopyGt7` bust be disabled
//!
//! The data must be crunched backwards and fed reversed into the routine.
//!
//! The latter can either though writing the compresses data reversed with `.write_reverse(true)` or `-r`,
//! or not writing it reversed but reversing it when feeding it into the decompressor: `data.iter().copied().rev()`.
//!
//! It comes in two favors: [`dynamic`] and [`generic`].
//! The generic version is a different, specialized, function for each proto while
//! the dynamic version does support all.
//!
//! In most cases they perform very similar, but if only a single proto is needed and
//! size matters the generic one will be better.
//!
//! # Create a compressed version
//!
//! ## Using this crate
//!
//! ```rust
//! # #[cfg(feature = "alloc")] {
//! # use exomizer::original::{crunch_raw, CrunchOptions};
//! # use exomizer::simple::generic::GenProto;
//! # let input = b"";
//! let crunched = unsafe {
//!     crunch_raw(
//!         &mut (),
//!         input,
//!         None,
//!         &CrunchOptions::builder()
//!             .flags_proto(61) // Or another supported proto, see above.
//!             .direction_forward(false)
//!             .write_reverse(true)
//!             .build()
//!             .unwrap(),
//!     )
//! };
//! let output = crunched.output;
//! # }
//! ```
//!
//! ## Using the exomizer binary
//!
//! ```shell
//! exomizer raw -b -r -P PROTO -o outfile infile
//! ```
//! Where `PROTO` is any of the allowed protos, see above.
//!
//! (Optionally `-C` for "`favor_speed`" can be used.)
//!
//! ##

mod common;
pub mod dynamic;
mod error;
pub mod generic;

pub use crate::simple::error::DecrunchError;
