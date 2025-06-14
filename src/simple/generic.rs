use crate::simple::DecrunchError;
use crate::simple::common::{Input, Tables, decr};
use core::ptr::slice_from_raw_parts_mut;

#[derive(Copy, Clone)]
/// The Proto flags used for decompressing.
pub struct GenProto<const P: u8>(());

impl GenProto<0> {
    pub const P9: GenProto<9> = GenProto(());
    pub const P13: GenProto<13> = GenProto(());
    pub const P25: GenProto<25> = GenProto(());
    pub const P29: GenProto<29> = GenProto(());
    pub const P41: GenProto<41> = GenProto(());
    pub const P45: GenProto<45> = GenProto(());
    pub const P57: GenProto<57> = GenProto(());
    pub const P61: GenProto<61> = GenProto(());
}

impl<const P: u8> GenProto<P> {
    /// Convert to the number of a proto.
    #[inline]
    #[must_use]
    pub const fn to_num(self) -> u8 {
        P
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
pub fn decrunch<I, const P: u8>(
    #[allow(unused_variables)] proto: GenProto<P>,
    src: I,
    dst: &mut [u8],
) -> Result<&mut [u8], DecrunchError>
where
    I: IntoIterator<Item = u8>,
{
    decr::<_, P>(src, dst)
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
pub fn decrunch_exact<I, const P: u8>(
    #[allow(unused_variables)] proto: GenProto<P>,
    src: I,
    dst: &mut [u8],
) -> Result<(), DecrunchError>
where
    I: IntoIterator<Item = u8>,
{
    let dst_len = dst.len();
    decr::<_, P>(src, dst).and_then(|r| {
        if r.len() == dst_len {
            Ok(())
        } else {
            Err(DecrunchError::BufferTooBig)
        }
    })
}

#[allow(clippy::too_many_lines)]
fn decr<I, const P: u8>(src: I, dst: &mut [u8]) -> Result<&mut [u8], DecrunchError>
where
    I: IntoIterator<Item = u8>,
{
    decr!(src, dst, P & 4 != 0, P & 16 != 0, P & 32 != 0);
}
