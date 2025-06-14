/*
 * Copyright (c) 2002 - 2007 Magnus Lind.
 *
 * This software is provided 'as-is', without any express or implied warranty.
 * In no event will the authors be held liable for any damages arising from
 * the use of this software.
 *
 * Permission is granted to anyone to use this software for any purpose,
 * including commercial applications, and to alter it and redistribute it
 * freely, subject to the following restrictions:
 *
 *   1. The origin of this software must not be misrepresented; you must not
 *   claim that you wrote the original software. If you use this software in a
 *   product, an acknowledgment in the product documentation would be
 *   appreciated but is not required.
 *
 *   2. Altered source versions must be plainly marked as such, and must not
 *   be misrepresented as being the original software.
 *
 *   3. This notice may not be removed or altered from any source distribution.
 *
 */

/*
 * Rust adaptions by Alex Kazik.
 */

use crate::original::converted::buf::*;
use crate::original::output::Output;

pub(crate) struct output_ctx {
    pub(crate) bitbuf: u8,
    pub(crate) bitcount: u8,
    pub(crate) pos: i32,
    pub(crate) buf: *mut buf,
    pub(crate) flags_proto: i32,
}
unsafe fn bitbuf_bit<O: Output>(output: &mut O, ctx: *mut output_ctx, bit: i32) {
    unsafe {
        if (*ctx).flags_proto & 1_i32 << 0_i32 != 0 {
            (*ctx).bitbuf = ((*ctx).bitbuf as i32 >> 1_i32) as u8;
            if bit != 0 {
                (*ctx).bitbuf = ((*ctx).bitbuf as i32 | 0x80_i32) as u8;
            }
            (*ctx).bitcount = ((*ctx).bitcount).wrapping_add(1);
            if (*ctx).bitcount as i32 == 8_i32 {
                output_bits_flush(output, ctx, 0_i32);
            }
        } else {
            (*ctx).bitbuf = (((*ctx).bitbuf as i32) << 1_i32) as u8;
            if bit != 0 {
                (*ctx).bitbuf = ((*ctx).bitbuf as i32 | 0x1_i32) as u8;
            }
            (*ctx).bitcount = ((*ctx).bitcount).wrapping_add(1);
            if (*ctx).bitcount as i32 == 8_i32 {
                output_bits_flush(output, ctx, 0_i32);
            }
        };
    }
}
pub(crate) unsafe fn output_ctx_init(ctx: *mut output_ctx, flags_proto: i32, out: *mut buf) {
    unsafe {
        (*ctx).bitbuf = 0_i32 as u8;
        (*ctx).bitcount = 0_i32 as u8;
        (*ctx).pos = buf_size(out);
        (*ctx).buf = out;
        (*ctx).flags_proto = flags_proto;
    }
}
pub(crate) unsafe fn output_get_pos(ctx: *mut output_ctx) -> u32 {
    unsafe { (*ctx).pos as u32 }
}
pub(crate) unsafe fn output_byte(ctx: *mut output_ctx, byte: u8) {
    unsafe {
        if (*ctx).pos < buf_size((*ctx).buf) {
            let mut p: *mut i8 = core::ptr::null_mut::<i8>();
            p = buf_data((*ctx).buf) as *mut i8;
            *p.offset((*ctx).pos as isize) = byte as i8;
        } else {
            while (*ctx).pos > buf_size((*ctx).buf) {
                buf_append_char((*ctx).buf, '\0' as i32 as i8);
            }
            buf_append_char((*ctx).buf, byte as i8);
        }
        (*ctx).pos += 1;
        (*ctx).pos;
    }
}
pub(crate) unsafe fn output_bits_flush<O: Output>(
    output: &mut O,
    ctx: *mut output_ctx,
    add_marker_bit: i32,
) {
    unsafe {
        if add_marker_bit != 0 {
            if (*ctx).flags_proto & 1_i32 << 0_i32 != 0 {
                (*ctx).bitbuf = ((*ctx).bitbuf as i32 | 0x80_i32 >> (*ctx).bitcount as i32) as u8;
            } else {
                (*ctx).bitbuf = ((*ctx).bitbuf as i32 | 0x1_i32 << (*ctx).bitcount as i32) as u8;
            }
            (*ctx).bitcount = ((*ctx).bitcount).wrapping_add(1);
            (*ctx).bitcount;
        }
        if (*ctx).bitcount as i32 > 0_i32 {
            output_byte(ctx, (*ctx).bitbuf);
            output.log_dump_ln(format_args!(
                "bitstream flushed 0x{:02x}",
                (*ctx).bitbuf as i32,
            ));
            (*ctx).bitbuf = 0_i32 as u8;
            (*ctx).bitcount = 0_i32 as u8;
        }
    }
}
pub(crate) unsafe fn output_bits_alignment<O: Output>(output: &mut O, ctx: *mut output_ctx) -> i32 {
    unsafe {
        let alignment: i32 = (8_i32 - (*ctx).bitcount as i32) & 7_i32;
        output.log_dump_ln(format_args!(
            "bitbuf 0x{:02x} aligned {}",
            (*ctx).bitbuf as i32,
            alignment,
        ));
        alignment
    }
}
unsafe fn output_bits_int<O: Output>(
    output: &mut O,
    ctx: *mut output_ctx,
    mut count: i32,
    mut val: i32,
) {
    unsafe {
        if (*ctx).flags_proto & 1_i32 << 1_i32 != 0 {
            while count > 7_i32 {
                output_byte(ctx, (val & 0xff_i32) as u8);
                count -= 8_i32;
                val >>= 8_i32;
            }
        }
        loop {
            let fresh0 = count;
            count -= 1;
            if fresh0 <= 0_i32 {
                break;
            }
            bitbuf_bit(output, ctx, val & 1_i32);
            val >>= 1_i32;
        }
    }
}
pub(crate) unsafe fn output_bits<O: Output>(
    output: &mut O,
    ctx: *mut output_ctx,
    count: i32,
    val: i32,
) {
    unsafe {
        output.log_dump_ln(format_args!(
            "output bits: count = {}, val = {}",
            count, val,
        ));
        output_bits_int(output, ctx, count, val);
    }
}
pub(crate) unsafe fn output_gamma_code<O: Output>(
    output: &mut O,
    ctx: *mut output_ctx,
    mut code: i32,
) {
    unsafe {
        output.log_dump_ln(format_args!("output gamma: code = {}", code,));
        output_bits_int(output, ctx, 1_i32, 1_i32);
        loop {
            let fresh1 = code;
            code -= 1;
            if fresh1 <= 0_i32 {
                break;
            }
            output_bits_int(output, ctx, 1_i32, 0_i32);
        }
    }
}
