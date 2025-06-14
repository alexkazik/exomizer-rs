use crate::original::converted::exo_helper::crunch_options;
use crate::original::flags::{ProtoFlags, TraitFlags};
use alloc::ffi::CString;
use alloc::string::String;
use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use core::ops::Deref;
use core::ptr::null;

/// Options for crunching.
#[derive(Clone)]
pub struct CrunchOptions {
    pub(crate) options: CrunchOptionsBuilder,
    pub(crate) enc: Option<CString>,
}

impl Debug for CrunchOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.options.fmt(f)
    }
}

impl Default for CrunchOptions {
    fn default() -> Self {
        CrunchOptionsBuilder::default().build().unwrap()
    }
}

impl Deref for CrunchOptions {
    type Target = CrunchOptionsBuilder;

    fn deref(&self) -> &Self::Target {
        &self.options
    }
}

impl CrunchOptions {
    #[must_use]
    pub fn builder() -> CrunchOptionsBuilder {
        CrunchOptionsBuilder::default()
    }

    pub(crate) fn to_crunch_options(&self) -> crunch_options<'_> {
        crunch_options {
            imported_encoding: self.enc.as_ref().map_or(null(), |s| s.as_ptr()),
            max_passes: self.max_passes.into(),
            max_len: self.max_len.into(),
            max_offset: self.max_offset.into(),
            favor_speed: self.favor_speed.into(),
            output_header: self.output_header.into(),
            flags_proto: self.flags_proto.as_num().into(),
            flags_notrait: self.flags_notrait.as_num().into(),
            direction_forward: self.direction_forward.into(),
            write_reverse: self.write_reverse.into(),
            imported_encoding_source: PhantomData,
        }
    }
}

/// Errors while building `CrunchOptions`.
#[derive(Copy, Clone, Debug, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
pub enum CrunchOptionsError {
    #[error("Invalid imported encoding")]
    InvalidImportedEncoding,
    #[error("Invalid max passes")]
    InvalidMaxPasses,
    #[error("Invalid proto flags")]
    InvalidFlagsProto,
    #[error("Invalid trait flags")]
    InvalidFlagsTrait,
}

/// Options for crunching builder.
#[derive(Clone, Debug)]
pub struct CrunchOptionsBuilder {
    /// Uses the given encoding for crunching, default is None.
    pub imported_encoding: Option<String>,
    /// Limits the number of optimization passes, default is 100. Valid: 0..=100
    pub max_passes: u8,
    /// Sets the maximum sequence length, default is 65535.
    pub max_len: u16,
    /// Sets the maximum sequence offset, default is 65535.
    pub max_offset: u16,
    /// Favor compression speed over ratio.
    pub favor_speed: bool,
    /// Don't write the encoding to the outfile.
    pub output_header: bool,
    /// Bitfield that controls bit stream format. \[0-63], default 39.
    pub flags_proto: ProtoFlags,
    /// Bitfield that controls bit stream traits. \[0-7], default 0.
    pub flags_notrait: TraitFlags,
    /// Set to false for crunch backwards, default false.
    pub direction_forward: bool,
    /// Write outfile in reverse order, default false.
    pub write_reverse: bool,
}

impl CrunchOptionsBuilder {
    #[inline]
    #[must_use]
    pub fn imported_encoding(mut self, value: Option<String>) -> Self {
        self.imported_encoding = value;
        self
    }

    #[inline]
    #[must_use]
    pub const fn max_passes(mut self, value: u8) -> Self {
        self.max_passes = value;
        self
    }

    #[inline]
    #[must_use]
    pub const fn max_len(mut self, value: u16) -> Self {
        self.max_len = value;
        self
    }

    #[inline]
    #[must_use]
    pub const fn max_offset(mut self, value: u16) -> Self {
        self.max_offset = value;
        self
    }

    #[inline]
    #[must_use]
    pub const fn favor_speed(mut self, value: bool) -> Self {
        self.favor_speed = value;
        self
    }

    #[inline]
    #[must_use]
    pub const fn output_header(mut self, value: bool) -> Self {
        self.output_header = value;
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
    pub const fn flags_notrait(mut self, value: TraitFlags) -> Self {
        self.flags_notrait = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn flags_notrait_num(mut self, value: u8) -> Self {
        self.flags_notrait = value.into();
        self
    }

    #[inline]
    #[must_use]
    pub const fn direction_forward(mut self, value: bool) -> Self {
        self.direction_forward = value;
        self
    }

    #[inline]
    #[must_use]
    pub const fn write_reverse(mut self, value: bool) -> Self {
        self.write_reverse = value;
        self
    }

    pub(crate) fn default() -> Self {
        Self {
            imported_encoding: None,
            max_passes: 100,
            max_len: 65535,
            max_offset: 65535,
            favor_speed: false,
            output_header: true,
            flags_proto: ProtoFlags::OrderBe
                | ProtoFlags::CopyGt7
                | ProtoFlags::Impl1Literal
                | ProtoFlags::ReuseOffset,
            flags_notrait: TraitFlags::None,
            direction_forward: false,
            write_reverse: false,
        }
    }

    /// Build the `CrunchOptions`,
    ///
    /// # Errors
    ///
    /// Will return an error in case invalid options are used.
    pub fn build(self) -> Result<CrunchOptions, CrunchOptionsError> {
        let enc = self.convert_encoding()?;
        if self.max_passes > 100 {
            return Err(CrunchOptionsError::InvalidMaxPasses);
        }
        if self.flags_proto.as_num() > 63 {
            return Err(CrunchOptionsError::InvalidFlagsProto);
        }
        if self.flags_notrait.as_num() > 7 {
            return Err(CrunchOptionsError::InvalidFlagsProto);
        }
        Ok(CrunchOptions { options: self, enc })
    }

    fn convert_encoding(&self) -> Result<Option<CString>, CrunchOptionsError> {
        // Examples:
        //         000000000011111111112222222222333333333344444444445555555555666666666677
        //         012345678901234567890123456789012345678901234567890123456789012345678901
        // 3table: 0010000000000000,2133,1213402124053561,3221313414452562
        // 4table: 1102345678A27A53,2110,1010314326067543,123142585043500B,020515804463DBEF
        if let Some(imported_encoding) = self.imported_encoding.as_ref() {
            // check length
            let len = if self.flags_proto & ProtoFlags::FourOffsetTables != 0 {
                72
            } else {
                55
            };
            if imported_encoding.len() != len {
                return Err(CrunchOptionsError::InvalidImportedEncoding);
            }

            // check data
            if !imported_encoding
                .as_bytes()
                .iter()
                .copied()
                .enumerate()
                .all(|(i, c)| {
                    if [16, 21, 38, 55].contains(&i) {
                        c == b','
                    } else {
                        c.is_ascii_hexdigit()
                    }
                })
            {
                return Err(CrunchOptionsError::InvalidImportedEncoding);
            }

            // convert to CString (this should never fail due to the check above)
            Ok(Some(CString::new(imported_encoding.as_bytes()).map_err(
                |_| CrunchOptionsError::InvalidImportedEncoding,
            )?))
        } else {
            Ok(None)
        }
    }
}
