use crate::original::converted::buf::{buf, buf_append, buf_data, buf_free, buf_init, buf_size};
use crate::original::converted::exo_helper;
use crate::original::converted::exo_helper::{crunch_info, io_bufs, reverse_buffer};
use crate::original::converted::vec::{vec, vec_free, vec_get, vec_push};
use crate::original::crunch_options::CrunchOptions;
use crate::original::flags::TraitFlags;
use crate::original::output::Output;
use crate::original::replacement::c_void;
use alloc::string::String;
use alloc::vec::Vec;
use core::ptr::null;
use core::slice::from_raw_parts;

/// Crunch data.
///
/// This simply calls [`crunch_multi_raw`] and manages the to/from many.
///
/// For more information see [`crate::original`].
///
/// # Safety
///
/// It is not allowed to run another [`crunch_raw`] or [`crunch_multi_raw`] in parallel since static mut data is used.
#[must_use]
#[inline]
#[allow(clippy::missing_panics_doc)]
pub unsafe fn crunch_raw<O: Output, T: AsRef<[u8]>>(
    output: &mut O,
    input: T,
    no_read: Option<&[u8]>,
    opts: &CrunchOptions,
) -> Crunched {
    unsafe {
        let CrunchedMulti {
            outputs,
            enc_bytes,
            enc_string,
            crunch_info,
        } = crunch_multi_raw::<O, _>(output, &[input], no_read, opts);
        Crunched {
            output: outputs.into_iter().next().unwrap_or_else(|| unreachable!()),
            enc_bytes,
            enc_string,
            crunch_info,
        }
    }
}

/// Crunch data.
///
/// For more information see [`crate::original`].
///
/// # Safety
///
/// It is not allowed to run another [`crunch_raw`] or [`crunch_multi_raw`] in parallel since static mut data is used.
#[must_use]
pub unsafe fn crunch_multi_raw<O: Output, T: AsRef<[u8]>>(
    output: &mut O,
    inputs: &[T],
    no_read: Option<&[u8]>,
    opts: &CrunchOptions,
) -> CrunchedMulti {
    unsafe {
        let opts = opts.to_crunch_options();

        let mut io_bufs: vec = {
            vec {
                elsize: size_of::<io_bufs>(),
                buf: {
                    buf {
                        data: core::ptr::null_mut::<c_void>(),
                        size: 0_i32,
                        capacity: 0_i32,
                    }
                },
                flags: 1_i32,
            }
        };

        for input in inputs {
            let input = input.as_ref();
            let io: *mut io_bufs = vec_push(&raw mut io_bufs, null::<c_void>()).cast::<io_bufs>();
            buf_init(&raw mut (*io).in_0);
            buf_init(&raw mut (*io).out);

            buf_append(&raw mut (*io).in_0, input.as_ptr().cast(), input.len() as _);
        }

        let mut no_read_buf_ptr: *mut buf = core::ptr::null_mut::<buf>();
        let mut no_read_buf = buf {
            data: core::ptr::null_mut::<c_void>(),
            size: 0_i32,
            capacity: 0_i32,
        };
        if let Some(no_read) = no_read {
            buf_init(&raw mut no_read_buf);
            buf_append(
                &raw mut no_read_buf,
                no_read.as_ptr().cast(),
                no_read.len() as _,
            );
            no_read_buf_ptr = &raw mut no_read_buf;
        }
        let mut enc_buf = buf {
            data: core::ptr::null_mut::<c_void>(),
            size: 0_i32,
            capacity: 0_i32,
        };
        buf_init(&raw mut enc_buf);

        let mut crunch_info: crunch_info = {
            crunch_info {
                traits_used: 0_i32,
                max_len: 0_i32,
                max_offset: 0_i32,
                needed_safety_offset: 0,
            }
        };
        let enc_string = exo_helper::crunch_multi(
            output,
            &raw mut io_bufs,
            no_read_buf_ptr,
            &raw mut enc_buf,
            &raw const opts,
            &raw mut crunch_info,
        );

        let mut outputs = Vec::with_capacity(inputs.len());
        for i in 0..inputs.len() {
            let io: *mut io_bufs = vec_get(&raw const io_bufs, i as i32).cast::<io_bufs>();
            if opts.write_reverse != 0 {
                reverse_buffer(
                    buf_data(&raw const (*io).out).cast(),
                    buf_size(&raw const (*io).out),
                );
            }
            let mut output = Vec::new();
            output.extend_from_slice(from_raw_parts(
                buf_data(&raw const (*io).out) as *const u8,
                buf_size(&raw const (*io).out) as _,
            ));
            outputs.push(output);
            buf_free(&raw mut (*io).in_0);
            buf_free(&raw mut (*io).out);
        }
        vec_free(&raw mut io_bufs);

        if no_read.is_some() {
            buf_free(&raw mut no_read_buf);
        }
        let enc_bytes = from_raw_parts(
            buf_data(&raw const enc_buf) as *const u8,
            buf_size(&raw const enc_buf) as _,
        )
        .to_vec();
        buf_free(&raw mut enc_buf);

        CrunchedMulti {
            outputs,
            enc_bytes,
            enc_string,
            crunch_info: CrunchInfo {
                traits_used: (crunch_info.traits_used as u8).into(),
                max_len: crunch_info.max_len as usize,
                max_offset: crunch_info.max_offset as usize,
                needed_safety_offset: crunch_info.needed_safety_offset as usize,
            },
        }
    }
}

/// Result of a single-file crunch.
pub struct Crunched {
    pub output: Vec<u8>,
    pub enc_bytes: Vec<u8>,
    pub enc_string: String,
    pub crunch_info: CrunchInfo,
}

/// Result of a multi-file crunch.
pub struct CrunchedMulti {
    pub outputs: Vec<Vec<u8>>,
    pub enc_bytes: Vec<u8>,
    pub enc_string: String,
    pub crunch_info: CrunchInfo,
}

/// Information about the crunch process.
#[derive(Clone, Debug)]
pub struct CrunchInfo {
    pub traits_used: TraitFlags,
    pub max_len: usize,
    pub max_offset: usize,
    pub needed_safety_offset: usize,
}
