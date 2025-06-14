use crate::original::converted::buf::{buf, buf_append, buf_data, buf_free, buf_init, buf_size};
use crate::original::converted::exo_helper;
use crate::original::converted::exo_helper::reverse_buffer;
use crate::original::decrunch_options::DecrunchOptions;
use crate::original::output::Output;
use crate::original::replacement::c_void;
use alloc::vec::Vec;
use core::slice::from_raw_parts;

/// Decrunch packed data.
///
/// For more information see [`crate::original`].
///
/// # Safety
///
/// It is known to segfault when decrunching bad data or with wrong parameters.
///
/// Whether [`decrunch_raw`] and/or [`crunch_raw`](crate::original::crunch::crunch_raw)/[`crunch_multi_raw`](crate::original::crunch::crunch_multi_raw) can be run in parallel depends on
/// the logging implementation.
#[must_use]
pub unsafe fn decrunch_raw<O: Output>(
    output: &mut O,
    input: &[u8],
    dopts: &DecrunchOptions,
) -> Vec<u8> {
    unsafe {
        let mut dopts = dopts.to_decrunch_options();

        let mut inbuf: buf = buf {
            data: core::ptr::null_mut::<c_void>(),
            size: 0,
            capacity: 0,
        };
        let mut outbuf: buf = buf {
            data: core::ptr::null_mut::<c_void>(),
            size: 0,
            capacity: 0,
        };

        buf_init(&raw mut inbuf);
        buf_init(&raw mut outbuf);

        buf_append(&raw mut inbuf, input.as_ptr().cast(), input.len() as _);

        exo_helper::decrunch(
            output,
            &raw mut inbuf,
            0_i32,
            &raw mut outbuf,
            &raw mut dopts,
        );

        if dopts.write_reverse != 0 {
            reverse_buffer(
                buf_data(&raw const outbuf).cast(),
                buf_size(&raw const outbuf),
            );
        }

        let mut output = Vec::new();
        output.extend_from_slice(from_raw_parts(
            buf_data(&raw const outbuf) as *const u8,
            buf_size(&raw const outbuf) as _,
        ));

        buf_free(&raw mut outbuf);
        buf_free(&raw mut inbuf);

        output
    }
}
