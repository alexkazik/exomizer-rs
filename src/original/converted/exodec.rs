/*
 * Copyright (c) 2002 - 2005 Magnus Lind.
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
use alloc::format;

pub(crate) struct dec_table {
    pub(crate) table_bit: [u8; 8],
    pub(crate) table_off: [u8; 8],
    pub(crate) table_bi: [u8; 100],
    pub(crate) table_lo: [u8; 100],
    pub(crate) table_hi: [u8; 100],
}
pub(crate) struct dec_ctx {
    pub(crate) inpos: i32,
    pub(crate) inend: i32,
    pub(crate) inbuf: *mut u8,
    pub(crate) outbuf: *mut buf,
    pub(crate) bitbuf: u8,
    pub(crate) t: dec_table,
    pub(crate) bits_read: i32,
    pub(crate) flags_proto: i32,
}
unsafe fn bitbuf_rotate(ctx: *mut dec_ctx, carry: i32) -> i32 {
    unsafe {
        let mut carry_out: i32 = 0;
        if (*ctx).flags_proto & 1_i32 << 0_i32 != 0 {
            carry_out = ((*ctx).bitbuf as i32 & 0x80_i32 != 0_i32) as i32;
            (*ctx).bitbuf = (((*ctx).bitbuf as i32) << 1_i32) as u8;
            if carry != 0 {
                (*ctx).bitbuf = ((*ctx).bitbuf as i32 | 0x1_i32) as u8;
            }
        } else {
            carry_out = (*ctx).bitbuf as i32 & 0x1_i32;
            (*ctx).bitbuf = ((*ctx).bitbuf as i32 >> 1_i32) as u8;
            if carry != 0 {
                (*ctx).bitbuf = ((*ctx).bitbuf as i32 | 0x80_i32) as u8;
            }
        }
        carry_out
    }
}
unsafe fn get(buf: *mut buf) -> *mut i8 {
    unsafe { buf_data(buf) as *mut i8 }
}
unsafe fn get_byte(ctx: *mut dec_ctx) -> i32 {
    unsafe {
        let mut c: i32 = 0;
        if (*ctx).inpos == (*ctx).inend {
            panic!("unexpected end of input data");
        }
        let fresh0 = (*ctx).inpos;
        (*ctx).inpos += 1;
        c = *((*ctx).inbuf).offset(fresh0 as isize) as i32;
        (*ctx).bits_read += 8_i32;
        c
    }
}
unsafe fn get_bits(ctx: *mut dec_ctx, mut count: i32) -> i32 {
    unsafe {
        let mut byte_copy: i32 = 0_i32;
        let mut val: i32 = 0;
        val = 0_i32;
        if (*ctx).flags_proto & 1_i32 << 1_i32 != 0 {
            while count > 7_i32 {
                byte_copy = count >> 3_i32;
                count &= 7_i32;
            }
        }
        loop {
            let fresh1 = count;
            count -= 1;
            if fresh1 <= 0_i32 {
                break;
            }
            let mut carry: i32 = bitbuf_rotate(ctx, 0_i32);
            if (*ctx).bitbuf as i32 == 0_i32 {
                (*ctx).bitbuf = get_byte(ctx) as u8;
                (*ctx).bits_read -= 8_i32;
                carry = bitbuf_rotate(ctx, 1_i32);
            }
            val <<= 1_i32;
            val |= carry;
            (*ctx).bits_read += 1;
            (*ctx).bits_read;
        }
        loop {
            let fresh2 = byte_copy;
            byte_copy -= 1;
            if fresh2 <= 0_i32 {
                break;
            }
            val <<= 8_i32;
            val |= get_byte(ctx);
        }
        val
    }
}
unsafe fn get_gamma_code(ctx: *mut dec_ctx) -> i32 {
    unsafe {
        let mut gamma_code: i32 = 0;
        gamma_code = 0_i32;
        while get_bits(ctx, 1_i32) == 0_i32 {
            gamma_code += 1;
            gamma_code;
        }
        gamma_code
    }
}
unsafe fn get_cooked_code_phase2(ctx: *mut dec_ctx, index: i32) -> i32 {
    unsafe {
        let mut base: i32 = 0;
        let mut tp: *mut dec_table = core::ptr::null_mut::<dec_table>();
        tp = &mut (*ctx).t;
        base = (*tp).table_lo[index as usize] as i32
            | ((*tp).table_hi[index as usize] as i32) << 8_i32;
        base + get_bits(ctx, (*tp).table_bi[index as usize] as i32)
    }
}
unsafe fn table_init(ctx: *mut dec_ctx, tp: *mut dec_table) {
    unsafe {
        let mut i: i32 = 0;
        let mut end: i32 = 0;
        let mut a: u32 = 0_i32 as u32;
        let mut b: u32 = 0_i32 as u32;
        (*tp).table_bit[0_i32 as usize] = 2_i32 as u8;
        (*tp).table_bit[1_i32 as usize] = 4_i32 as u8;
        (*tp).table_bit[2_i32 as usize] = 4_i32 as u8;
        if (*ctx).flags_proto & 1_i32 << 4_i32 != 0 {
            end = 68_i32;
            (*tp).table_bit[3_i32 as usize] = 4_i32 as u8;
            (*tp).table_off[0_i32 as usize] = 64_i32 as u8;
            (*tp).table_off[1_i32 as usize] = 48_i32 as u8;
            (*tp).table_off[2_i32 as usize] = 32_i32 as u8;
            (*tp).table_off[3_i32 as usize] = 16_i32 as u8;
        } else {
            end = 52_i32;
            (*tp).table_off[0_i32 as usize] = 48_i32 as u8;
            (*tp).table_off[1_i32 as usize] = 32_i32 as u8;
            (*tp).table_off[2_i32 as usize] = 16_i32 as u8;
        }
        i = 0_i32;
        while i < end {
            if i & 0xf_i32 != 0 {
                a = a.wrapping_add((1_i32 << b) as u32);
            } else {
                a = 1_i32 as u32;
            }
            (*tp).table_lo[i as usize] = (a & 0xff_i32 as u32) as u8;
            (*tp).table_hi[i as usize] = (a >> 8_i32) as u8;
            if (*ctx).flags_proto & 1_i32 << 1_i32 != 0 {
                b = get_bits(ctx, 3_i32) as u32;
                b |= (get_bits(ctx, 1_i32) << 3_i32) as u32;
            } else {
                b = get_bits(ctx, 4_i32) as u32;
            }
            (*tp).table_bi[i as usize] = b as u8;
            i += 1;
            i;
        }
    }
}
pub(crate) unsafe fn dec_ctx_init(
    ctx: *mut dec_ctx,
    enc_in: *mut buf,
    inbuf: *mut buf,
    outbuf: *mut buf,
    flags_proto: i32,
) {
    unsafe {
        let mut enc: *mut buf = enc_in;
        if enc_in.is_null() {
            enc = inbuf;
        }
        (*ctx).bits_read = 0_i32;
        (*ctx).inbuf = buf_data(enc) as *mut u8;
        (*ctx).inend = buf_size(enc);
        (*ctx).inpos = 0_i32;
        (*ctx).flags_proto = flags_proto;
        (*ctx).outbuf = outbuf;
        if flags_proto & 1_i32 << 3_i32 != 0 {
            (*ctx).bitbuf = 0_i32 as u8;
        } else {
            (*ctx).bitbuf = get_byte(ctx) as u8;
        }
        table_init(ctx, &mut (*ctx).t);
        if !enc_in.is_null() {
            (*ctx).inbuf = buf_data(inbuf) as *mut u8;
            (*ctx).inend = buf_size(inbuf);
            (*ctx).inpos = 0_i32;
            if flags_proto & 1_i32 << 3_i32 != 0 {
                (*ctx).bitbuf = 0_i32 as u8;
            } else {
                (*ctx).bitbuf = get_byte(ctx) as u8;
            }
        }
    }
}
pub(crate) unsafe fn dec_ctx_table_dump(ctx: *mut dec_ctx, enc_out: *mut buf) {
    unsafe {
        if !enc_out.is_null() {
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            let mut offset_tables: i32 = 3_i32;
            if (*ctx).flags_proto & 1_i32 << 4_i32 != 0 {
                offset_tables = 4_i32;
            }
            buf_clear(enc_out);
            i = 0_i32;
            while i < 16_i32 {
                let s = format!("{:X}", (*ctx).t.table_bi[i as usize] as i32);
                buf_append(enc_out, s.as_ptr() as *const _, s.len() as i32);
                i += 1;
                i;
            }
            j = 0_i32;
            while j < offset_tables {
                let mut start: i32 = 0;
                let mut end: i32 = 0;
                buf_append_char(enc_out, ',' as i32 as i8);
                start = (*ctx).t.table_off[j as usize] as i32;
                end = start + (1_i32 << (*ctx).t.table_bit[j as usize] as i32);
                i = start;
                while i < end {
                    let s = format!("{:X}", (*ctx).t.table_bi[i as usize] as i32);
                    buf_append(enc_out, s.as_ptr() as *const _, s.len() as i32);
                    i += 1;
                    i;
                }
                j += 1;
                j;
            }
        }
    }
}
pub(crate) unsafe fn dec_ctx_free(ctx: *mut dec_ctx) {}
pub(crate) unsafe fn dec_ctx_decrunch<O: Output>(output: &mut O, ctx: *mut dec_ctx) {
    unsafe {
        let mut reuse_offset: i32 = 0;
        let mut current_block: usize;
        let mut bits: i32 = (*ctx).bits_read;
        let mut val: i32 = 0;
        let mut i: i32 = 0;
        let mut len: i32 = 0;
        let mut literal: i32 = 1_i32;
        let mut offset: i32 = 0_i32;
        let mut src: i32 = 0_i32;
        let treshold: i32 = if (*ctx).flags_proto & 1_i32 << 4_i32 != 0 {
            4_i32
        } else {
            3_i32
        };
        let mut reuse_offset_state: i32 = 1_i32;
        if (*ctx).flags_proto & 1_i32 << 2_i32 != 0 {
            current_block = 4701521019073072020;
        } else {
            current_block = 11875828834189669668;
        }
        loop {
            match current_block {
                4701521019073072020 => {
                    len = 1_i32;
                    output.log_debug_ln(format_args!(
                        "[{}] literal ${:02x}",
                        buf_size((*ctx).outbuf),
                        *((*ctx).inbuf).offset((*ctx).inpos as isize) as i32,
                    ));
                    literal = 1_i32;
                }
                _ => {
                    reuse_offset = 0;
                    reuse_offset_state <<= 1_i32;
                    reuse_offset_state |= literal;
                    literal = 0_i32;
                    bits = (*ctx).bits_read;
                    output.log_debug(format_args!("[{:02x}]", (*ctx).bitbuf as i32));
                    if get_bits(ctx, 1_i32) != 0 {
                        current_block = 4701521019073072020;
                        continue;
                    }
                    val = get_gamma_code(ctx);
                    if val == 16_i32 {
                        break;
                    }
                    if val == 17_i32 {
                        len = get_bits(ctx, 16_i32);
                        literal = 1_i32;
                        output.log_debug_ln(format_args!(
                            "[{}] literal copy len {}",
                            buf_size((*ctx).outbuf),
                            len,
                        ));
                    } else {
                        len = get_cooked_code_phase2(ctx, val);
                        reuse_offset = 0_i32;
                        if (*ctx).flags_proto & 1_i32 << 5_i32 != 0_i32
                            && reuse_offset_state & 3_i32 == 1_i32
                        {
                            reuse_offset = get_bits(ctx, 1_i32);
                            output.log_debug_ln(format_args!(
                                "[{}] offset reuse bit = {}, latest = {}",
                                buf_size((*ctx).outbuf),
                                reuse_offset,
                                offset,
                            ));
                        }
                        if reuse_offset == 0 {
                            i = (if len > treshold { treshold } else { len }) - 1_i32;
                            val = (*ctx).t.table_off[i as usize] as i32
                                + get_bits(ctx, (*ctx).t.table_bit[i as usize] as i32);
                            offset = get_cooked_code_phase2(ctx, val);
                        }
                        output.log_debug_ln(format_args!(
                            "[{}] sequence offset = {}, len = {}",
                            buf_size((*ctx).outbuf),
                            offset,
                            len,
                        ));
                        src = buf_size((*ctx).outbuf) - offset;
                    }
                }
            }
            loop {
                if literal != 0 {
                    val = get_byte(ctx);
                } else {
                    let fresh3 = src;
                    src += 1;
                    val = *(get((*ctx).outbuf)).offset(fresh3 as isize) as i32;
                }
                buf_append_char((*ctx).outbuf, val as i8);
                len -= 1;
                if len <= 0_i32 {
                    break;
                }
            }
            if (*ctx).flags_proto & 1_i32 << 4_i32 != 0 {
                output.log_debug_ln(format_args!(
                    "bits read for this iteration {}, total {}.",
                    (*ctx).bits_read - bits,
                    (*ctx).bits_read - 280_i32,
                ));
            } else {
                output.log_debug_ln(format_args!(
                    "bits read for this iteration {}, total {}.",
                    (*ctx).bits_read - bits,
                    (*ctx).bits_read - 216_i32,
                ));
            }
            current_block = 11875828834189669668;
        }
    }
}
