use crate::original::converted::exo_helper::decrunch_options;
use crate::original::flags::ProtoFlags;
use alloc::ffi::CString;
use alloc::string::String;
use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use core::ops::Deref;
use core::ptr::null;

/// Options for decrunching.
#[derive(Clone)]
pub struct DecrunchOptions {
    pub(crate) options: DecrunchOptionsBuilder,
    pub(crate) enc: Option<CString>,
}

impl Debug for DecrunchOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.options.fmt(f)
    }
}

impl Default for DecrunchOptions {
    fn default() -> Self {
        DecrunchOptionsBuilder::default().build().unwrap()
    }
}

impl Deref for DecrunchOptions {
    type Target = DecrunchOptionsBuilder;

    fn deref(&self) -> &Self::Target {
        &self.options
    }
}

impl DecrunchOptions {
    #[must_use]
    pub fn builder() -> DecrunchOptionsBuilder {
        DecrunchOptionsBuilder::default()
    }

    pub(crate) fn to_decrunch_options(&self) -> decrunch_options<'_> {
        decrunch_options {
            imported_encoding: self.enc.as_ref().map_or(null(), |s| s.as_ptr()),
            flags_proto: self.flags_proto.as_num().into(),
            direction_forward: self.direction_forward.into(),
            write_reverse: self.write_reverse.into(),
            imported_encoding_source: PhantomData,
        }
    }
}

/// Errors while building `DecrunchOptions`.
#[derive(Copy, Clone, Debug, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
pub enum DecrunchOptionsError {
    #[error("Invalid imported encoding")]
    InvalidImportedEncoding,
    #[error("Invalid proto flags")]
    InvalidFlagsProto,
}

/// Options for decrunching builder.
#[derive(Clone, Debug)]
pub struct DecrunchOptionsBuilder {
    /// Uses the given encoding for crunching, default is None.
    pub imported_encoding: Option<String>,
    /// Bitfield that controls bit stream format. \[0-63], default 39.
    pub flags_proto: ProtoFlags,
    /// Set to false for crunch backwards, default false.
    pub direction_forward: bool,
    /// Write outfile in reverse order, default false.
    pub write_reverse: bool,
}

impl DecrunchOptionsBuilder {
    #[inline]
    #[must_use]
    pub fn imported_encoding(mut self, value: Option<String>) -> Self {
        self.imported_encoding = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn flags_proto<T: Into<ProtoFlags>>(mut self, value: T) -> Self {
        self.flags_proto = value.into();
        self
    }

    #[inline]
    #[must_use]
    pub fn direction_forward(mut self, value: bool) -> Self {
        self.direction_forward = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn write_reverse(mut self, value: bool) -> Self {
        self.write_reverse = value;
        self
    }

    pub(crate) fn default() -> Self {
        Self {
            imported_encoding: None,
            flags_proto: ProtoFlags::OrderBe
                | ProtoFlags::CopyGt7
                | ProtoFlags::Impl1Literal
                | ProtoFlags::ReuseOffset,
            direction_forward: false,
            write_reverse: false,
        }
    }

    /// Build the `DecrunchOptions`,
    ///
    /// # Errors
    ///
    /// Will return an error in case invalid options are used.
    pub fn build(self) -> Result<DecrunchOptions, DecrunchOptionsError> {
        let enc = self.convert_encoding()?;
        if self.flags_proto.as_num() > 63 {
            return Err(DecrunchOptionsError::InvalidFlagsProto);
        }
        Ok(DecrunchOptions { options: self, enc })
    }

    fn convert_encoding(&self) -> Result<Option<CString>, DecrunchOptionsError> {
        if let Some(imported_encoding) = self.imported_encoding.as_ref() {
            Ok(Some(CString::new(imported_encoding.as_bytes()).map_err(
                |_| DecrunchOptionsError::InvalidImportedEncoding,
            )?))
        } else {
            Ok(None)
        }
    }
}
