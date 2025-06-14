use crate::simple::DecrunchError;
use crate::simple::common::{Input, Tables, decr};
use core::ptr::slice_from_raw_parts_mut;

/// Error returned by trying to convert an unsupported number into [`DynProto`].
#[derive(Clone, Copy)]
pub struct InvalidProtoError;

/// The Proto flags used for decompressing.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DynProto(u8);

impl DynProto {
    pub const P9: Self = Self(9);
    pub const P13: Self = Self(13);
    pub const P25: Self = Self(25);
    pub const P29: Self = Self(29);
    pub const P41: Self = Self(41);
    pub const P45: Self = Self(45);
    pub const P57: Self = Self(57);
    pub const P61: Self = Self(61);

    /// Convert from the number of a proto.
    ///
    /// # Errors
    ///
    /// Will return [`InvalidProtoError`] when an unsupported number is used.
    pub const fn from_num(value: u8) -> Result<Self, InvalidProtoError> {
        if (value | 4 | 16 | 32) == 61 {
            Ok(DynProto(value))
        } else {
            Err(InvalidProtoError)
        }
    }

    /// Convert to the number of a proto.
    #[inline]
    #[must_use]
    pub const fn to_num(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for DynProto {
    type Error = InvalidProtoError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_num(value)
    }
}

impl From<DynProto> for u8 {
    fn from(value: DynProto) -> Self {
        value.to_num()
    }
}

/// Decrunch data.
///
/// See [simple](crate::simple) for more information.
///
/// ```rust
/// # use exomizer::simple::dynamic::{decrunch, DynProto};
/// # let input = b"";
/// let mut output = [0; 800];
/// match decrunch(DynProto::P61, input.iter().copied(), &mut output) {
///   Ok(data) => println!("decrunched {} bytes", data.len()),
///   Err(err) => println!("error while decrunching: {err}"),
/// }
/// ```
///
/// # Errors
///
/// See [`DecrunchError`] for possible errors (though `DecrunchError::BufferTooBig` will never returned).
pub fn decrunch<I>(proto: DynProto, src: I, dst: &mut [u8]) -> Result<&mut [u8], DecrunchError>
where
    I: IntoIterator<Item = u8>,
{
    decr(proto, src, dst)
}

/// Decrunch data.
///
/// The output buffer must be the exact size of the decrunched data.
///
/// See [simple](crate::simple) for more information.
///
/// ```rust
/// # use exomizer::simple::dynamic::{decrunch_exact, DynProto};
/// # let input = b"";
/// let mut output = [0; 800];
/// match decrunch_exact(DynProto::P61, input.iter().copied(), &mut output) {
///   Ok(()) => println!("decrunched {} bytes", output.len()),
///   Err(err) => println!("error while decrunching: {err}"),
/// }
/// ```
///
/// # Errors
///
/// See [`DecrunchError`] for possible errors, will return `DecrunchError::BufferTooBig` when the
/// size of the decompressed data smaller than the size of the destination.
pub fn decrunch_exact<I>(proto: DynProto, src: I, dst: &mut [u8]) -> Result<(), DecrunchError>
where
    I: IntoIterator<Item = u8>,
{
    let dst_len = dst.len();
    decr(proto, src, dst).and_then(|r| {
        if r.len() == dst_len {
            Ok(())
        } else {
            Err(DecrunchError::BufferTooBig)
        }
    })
}

#[allow(clippy::too_many_lines)]
fn decr<I>(proto: DynProto, src: I, dst: &mut [u8]) -> Result<&mut [u8], DecrunchError>
where
    I: IntoIterator<Item = u8>,
{
    decr!(
        src,
        dst,
        proto.0 & 4 != 0,
        proto.0 & 16 != 0,
        proto.0 & 32 != 0
    );
}
