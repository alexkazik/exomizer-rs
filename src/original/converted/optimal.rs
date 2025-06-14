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
use crate::original::converted::chunkpool::*;
use crate::original::converted::r#match::*;
use crate::original::converted::output::*;
use crate::original::converted::radix::*;
use crate::original::converted::vec::*;
use crate::original::output::Output;
use crate::original::replacement::*;
use alloc::string::String;
use core::ffi::CStr;
use core::fmt::Write;

pub(crate) struct encode_int_bucket {
    pub(crate) start: u32,
    pub(crate) end: u32,
}
pub(crate) struct encode_match_buckets {
    pub(crate) len: encode_int_bucket,
    pub(crate) offset: encode_int_bucket,
}
pub(crate) struct encode_match_data {
    pub(crate) out: *mut output_ctx,
    pub(crate) priv_0: *mut c_void,
}
pub(crate) type encode_int_f<O> =
    unsafe fn(&mut O, i32, *mut c_void, *mut output_ctx, *mut encode_int_bucket) -> f32;
pub(crate) struct encode_match_priv<O> {
    pub(crate) flags_proto: i32,
    pub(crate) flags_notrait: i32,
    pub(crate) lit_num: i32,
    pub(crate) seq_num: i32,
    pub(crate) rle_num: i32,
    pub(crate) lit_bits: f32,
    pub(crate) seq_bits: f32,
    pub(crate) rle_bits: f32,
    pub(crate) offset_f: Option<encode_int_f<O>>,
    pub(crate) len_f: Option<encode_int_f<O>>,
    pub(crate) offset_f_priv: *mut c_void,
    pub(crate) len_f_priv: *mut c_void,
}

#[derive(Clone)]
pub(crate) struct interval_node {
    pub(crate) start: i32,
    pub(crate) score: i32,
    pub(crate) next: *mut interval_node,
    pub(crate) prefix: i8,
    pub(crate) bits: i8,
    pub(crate) depth: i8,
    pub(crate) flags: i8,
}
pub(crate) struct optimize_arg {
    pub(crate) cache: radix_root,
    pub(crate) stats: *mut i32,
    pub(crate) stats2: *mut i32,
    pub(crate) max_depth: i32,
    pub(crate) flags: i32,
    pub(crate) in_pool: chunkpool,
}
unsafe fn interval_node_init(inp: *mut interval_node, start: i32, depth: i32, flags: i32) {
    unsafe {
        (*inp).start = start;
        (*inp).flags = flags as i8;
        (*inp).depth = depth as i8;
        (*inp).bits = 0_i32 as i8;
        (*inp).prefix = (if flags >= 0_i32 { flags } else { depth + 1_i32 }) as i8;
        (*inp).score = -1_i32;
        (*inp).next = core::ptr::null_mut::<interval_node>();
    }
}
unsafe fn interval_node_clone(inp: *mut interval_node) -> *mut interval_node {
    unsafe {
        let mut inp2: *mut interval_node = core::ptr::null_mut::<interval_node>();
        if !inp.is_null() {
            inp2 = malloc(::core::mem::size_of::<interval_node>()) as *mut interval_node;
            if inp2.is_null() {
                panic!("out of memory error",);
            }
            (*inp2).clone_from(&*inp);
            (*inp2).next = interval_node_clone((*inp).next);
        }
        inp2
    }
}
unsafe fn interval_node_delete(mut inp: *mut interval_node) {
    unsafe {
        let mut inp2: *mut interval_node = core::ptr::null_mut::<interval_node>();
        while !inp.is_null() {
            inp2 = inp;
            inp = (*inp).next;
            free(inp2 as *mut c_void);
        }
    }
}
unsafe fn interval_node_dump<O: Output>(
    output: &mut O,
    is_dump: bool,
    mut inp: *mut interval_node,
) {
    unsafe {
        let mut end: i32 = 0;
        end = 0_i32;
        while !inp.is_null() {
            end = (*inp).start + (1_i32 << (*inp).bits as i32);
            if is_dump {
                output.log_dump(format_args!("{:x}", (*inp).bits as i32));
            } else {
                output.log_debug(format_args!("{:x}", (*inp).bits as i32));
            }
            inp = (*inp).next;
        }
        if is_dump {
            output.log_dump_ln(format_args!("[eol@{}]", end));
        } else {
            output.log_debug_ln(format_args!("[eol@{}]", end));
        }
    }
}
pub(crate) unsafe fn optimal_encode_int<O: Output>(
    output: &mut O,
    arg: i32,
    priv_0: *mut c_void,
    out: *mut output_ctx,
    eibp: *mut encode_int_bucket,
) -> f32 {
    unsafe {
        let mut inp: *mut interval_node = core::ptr::null_mut::<interval_node>();
        let mut end: i32 = 0;
        let mut val: f32 = 0.;
        inp = priv_0 as *mut interval_node;
        val = 100000000.0f64 as f32;
        end = 0_i32;
        while !inp.is_null() {
            end = (*inp).start + (1_i32 << (*inp).bits as i32);
            if arg >= (*inp).start && arg < end {
                break;
            }
            inp = (*inp).next;
        }
        if !inp.is_null() {
            val = ((*inp).prefix as i32 + (*inp).bits as i32) as f32;
            if !eibp.is_null() {
                (*eibp).start = (*inp).start as u32;
                (*eibp).end = end as u32;
            }
        } else {
            val += (arg - end) as f32;
            if !eibp.is_null() {
                (*eibp).start = 0_i32 as u32;
                (*eibp).end = 0_i32 as u32;
            }
        }
        output.log_dump_ln(format_args!("encoding {} to {:.1} bits", arg, val as f64,));
        if !out.is_null() {
            output_bits(output, out, (*inp).bits as i32, arg - (*inp).start);
            if ((*inp).flags as i32) < 0_i32 {
                output.log_dump_ln(format_args!("gamma prefix code = {}", (*inp).depth as i32,));
                output_gamma_code(output, out, (*inp).depth as i32);
            } else {
                output.log_dump_ln(format_args!("flat prefix {} bits", (*inp).depth as i32,));
                output_bits(output, out, (*inp).prefix as i32, (*inp).depth as i32);
            }
        }
        val
    }
}
pub(crate) unsafe fn optimal_encode<O: Output>(
    output: &mut O,
    mp: *const match_0,
    emd: *mut encode_match_data,
    prev_offset: u32,
    embp: *mut encode_match_buckets,
) -> f32 {
    unsafe {
        let mut offset: *mut *mut interval_node = core::ptr::null_mut::<*mut interval_node>();
        let mut bits: f32 = 0.;
        let mut data: *mut encode_match_priv<O> = core::ptr::null_mut::<encode_match_priv<O>>();
        let mut eib_len: *mut encode_int_bucket = core::ptr::null_mut::<encode_int_bucket>();
        let mut eib_offset: *mut encode_int_bucket = core::ptr::null_mut::<encode_int_bucket>();
        if !embp.is_null() {
            eib_len = &mut (*embp).len;
            eib_offset = &mut (*embp).offset;
        }
        data = (*emd).priv_0 as *mut encode_match_priv<O>;
        offset = (*data).offset_f_priv as *mut *mut interval_node;
        bits = 0.0f64 as f32;
        if (*mp).len as i32 > 255_i32
            && (*data).flags_notrait & 1_i32 << 2_i32 != 0
            && ((*mp).len as i32 & 255_i32)
                < (if (*data).flags_proto & 1_i32 << 4_i32 != 0 {
                    4_i32
                } else {
                    3_i32
                })
        {
            bits = (bits as f64 + 100000000.0f64) as f32;
        }
        if (*mp).offset == 0_i32 as u32 {
            bits += 9.0f32 * (*mp).len as i32 as f32;
            (*data).lit_num += (*mp).len as i32;
            (*data).lit_bits += bits;
        } else {
            bits = (bits as f64 + 1.0f64) as f32;
            if (*mp).offset != prev_offset {
                let current_block_30: usize;
                match (*mp).len as i32 {
                    0 => {
                        panic!("bad len");
                    }
                    1 => {
                        if (*data).flags_notrait & 1_i32 << 1_i32 != 0 {
                            bits = (bits as f64 + 100000000.0f64) as f32;
                        } else {
                            bits += ((*data).offset_f).expect("non-null function pointer")(
                                output,
                                (*mp).offset as i32,
                                *offset.offset(0_i32 as isize) as *mut c_void,
                                (*emd).out,
                                eib_offset,
                            );
                        }
                        current_block_30 = 17281240262373992796;
                    }
                    2 => {
                        bits += ((*data).offset_f).expect("non-null function pointer")(
                            output,
                            (*mp).offset as i32,
                            *offset.offset(1_i32 as isize) as *mut c_void,
                            (*emd).out,
                            eib_offset,
                        );
                        current_block_30 = 17281240262373992796;
                    }
                    3 => {
                        if (*data).flags_proto & 1_i32 << 4_i32 != 0 {
                            bits += ((*data).offset_f).expect("non-null function pointer")(
                                output,
                                (*mp).offset as i32,
                                *offset.offset(2_i32 as isize) as *mut c_void,
                                (*emd).out,
                                eib_offset,
                            );
                            current_block_30 = 17281240262373992796;
                        } else {
                            current_block_30 = 15889152483458450077;
                        }
                    }
                    _ => {
                        current_block_30 = 15889152483458450077;
                    }
                }
                if current_block_30 == 15889152483458450077 {
                    bits += ((*data).offset_f).expect("non-null function pointer")(
                        output,
                        (*mp).offset as i32,
                        *offset.offset(7_i32 as isize) as *mut c_void,
                        (*emd).out,
                        eib_offset,
                    );
                }
            }
            if prev_offset > 0_i32 as u32 {
                bits = (bits as f64 + 1.0f64) as f32;
                if !((*emd).out).is_null() {
                    output_bits(
                        output,
                        (*emd).out,
                        1_i32,
                        ((*mp).offset == prev_offset) as i32,
                    );
                }
            }
            bits += ((*data).len_f).expect("non-null function pointer")(
                output,
                (*mp).len as i32,
                (*data).len_f_priv,
                (*emd).out,
                eib_len,
            );
            if bits as f64 > 9.0f64 * (*mp).len as i32 as f64 {
                (*data).lit_num += 1_i32;
                (*data).lit_bits += bits;
            } else if (*mp).offset == 1_i32 as u32 {
                (*data).rle_num += 1_i32;
                (*data).rle_bits += bits;
            } else {
                (*data).seq_num += 1_i32;
                (*data).seq_bits += bits;
            }
        }
        if !embp.is_null()
            && (((*eib_len).start).wrapping_add((*eib_len).end) == 0_i32 as u32
                || ((*eib_offset).start).wrapping_add((*eib_offset).end) == 0_i32 as u32)
        {
            (*eib_len).start = 0_i32 as u32;
            (*eib_len).end = 0_i32 as u32;
            (*eib_offset).start = 0_i32 as u32;
            (*eib_offset).end = 0_i32 as u32;
        }
        bits
    }
}
unsafe fn optimize1<O: Output>(
    output: &mut O,
    arg: *mut optimize_arg,
    start: i32,
    depth: i32,
    init: i32,
) -> *mut interval_node {
    unsafe {
        let mut in_0: interval_node = interval_node {
            start: 0,
            score: 0,
            next: core::ptr::null_mut::<interval_node>(),
            prefix: 0,
            bits: 0,
            depth: 0,
            flags: 0,
        };
        let mut best_inp: *mut interval_node = core::ptr::null_mut::<interval_node>();
        let mut key: i32 = 0;
        let mut end: i32 = 0;
        let mut i: i32 = 0;
        let mut start_count: i32 = 0;
        let mut end_count: i32 = 0;
        output.log_dump_ln(format_args!("IN start {}, depth {}", start, depth,));
        best_inp = core::ptr::null_mut::<interval_node>();
        if *((*arg).stats).offset(start as isize) != 0_i32 {
            key = start * 32_i32 + depth;
            best_inp = radix_node_get(&mut (*arg).cache, key as u32) as *mut interval_node;
            if best_inp.is_null() {
                interval_node_init(&mut in_0, start, depth, (*arg).flags);
                i = 0_i32;
                while i < 16_i32 {
                    in_0.next = core::ptr::null_mut::<interval_node>();
                    in_0.bits = i as i8;
                    end = start + (1_i32 << i);
                    end_count = 0_i32;
                    start_count = end_count;
                    if start < 1000000_i32 {
                        start_count = *((*arg).stats).offset(start as isize);
                        if end < 1000000_i32 {
                            end_count = *((*arg).stats).offset(end as isize);
                        }
                    }
                    in_0.score =
                        (start_count - end_count) * (in_0.prefix as i32 + in_0.bits as i32);
                    output.log_dump_ln(format_args!(
                        "interval score: [{}<<{}[{}",
                        start, i, in_0.score,
                    ));
                    if end_count > 0_i32 {
                        let mut penalty: i32 = 0;
                        if (depth + 1_i32) < (*arg).max_depth {
                            in_0.next = optimize1(output, arg, end, depth + 1_i32, i);
                        }
                        penalty = 100000000_i32;
                        if !((*arg).stats2).is_null() {
                            penalty = *((*arg).stats2).offset(end as isize);
                        }
                        if !(in_0.next).is_null() && (*in_0.next).score < penalty {
                            penalty = (*in_0.next).score;
                        }
                        in_0.score += penalty;
                    }
                    if best_inp.is_null() || in_0.score < (*best_inp).score {
                        if best_inp.is_null() {
                            best_inp =
                                chunkpool_malloc(output, &mut (*arg).in_pool) as *mut interval_node;
                        }
                        (*best_inp).clone_from(&in_0);
                    }
                    i += 1;
                    i;
                }
                if !best_inp.is_null() {
                    radix_node_set(
                        output,
                        &mut (*arg).cache,
                        key as u32,
                        best_inp as *mut c_void,
                    );
                }
            }
        }
        output.log_dump(format_args!("OUT depth {}: ", depth));
        interval_node_dump(output, true, best_inp);
        best_inp
    }
}
unsafe fn optimize<O: Output>(
    output: &mut O,
    stats: *mut i32,
    stats2: *mut i32,
    max_depth: i32,
    flags: i32,
) -> *mut interval_node {
    unsafe {
        let mut arg: optimize_arg = optimize_arg {
            cache: radix_root {
                depth: 0,
                root: core::ptr::null_mut::<radix_node>(),
                mem: chunkpool {
                    item_size: 0,
                    item_pos: 0,
                    item_end: 0,
                    current_chunk: core::ptr::null_mut::<c_void>(),
                    used_chunks: vec {
                        elsize: 0,
                        buf: buf {
                            data: core::ptr::null_mut::<c_void>(),
                            size: 0,
                            capacity: 0,
                        },
                        flags: 0,
                    },
                    alloc_count: 0,
                },
            },
            stats: core::ptr::null_mut::<i32>(),
            stats2: core::ptr::null_mut::<i32>(),
            max_depth: 0,
            flags: 0,
            in_pool: chunkpool {
                item_size: 0,
                item_pos: 0,
                item_end: 0,
                current_chunk: core::ptr::null_mut::<c_void>(),
                used_chunks: vec {
                    elsize: 0,
                    buf: buf {
                        data: core::ptr::null_mut::<c_void>(),
                        size: 0,
                        capacity: 0,
                    },
                    flags: 0,
                },
                alloc_count: 0,
            },
        };
        let mut inp: *mut interval_node = core::ptr::null_mut::<interval_node>();
        arg.stats = stats;
        arg.stats2 = stats2;
        arg.max_depth = max_depth;
        arg.flags = flags;
        chunkpool_init(
            &mut arg.in_pool,
            ::core::mem::size_of::<interval_node>() as i32,
        );
        radix_tree_init(&mut arg.cache);
        inp = optimize1(output, &mut arg, 1_i32, 0_i32, 0_i32);
        inp = interval_node_clone(inp);
        radix_tree_free(&mut arg.cache, None, core::ptr::null_mut::<c_void>());
        chunkpool_free(&mut arg.in_pool);
        inp
    }
}
unsafe fn export_helper(mut np: *mut interval_node, mut depth: i32, target: &mut String) {
    unsafe {
        while !np.is_null() {
            let _ = write!(target, "{:X}", (*np).bits as i32);
            np = (*np).next;
            depth -= 1;
            depth;
        }
        loop {
            let fresh0 = depth;
            depth -= 1;
            if fresh0 <= 0_i32 {
                break;
            }
            target.push('0');
        }
    }
}
pub(crate) unsafe fn optimal_encoding_export<O>(emd: *mut encode_match_data, target: &mut String) {
    unsafe {
        let mut offsets: *mut *mut interval_node = core::ptr::null_mut::<*mut interval_node>();
        let mut data: *mut encode_match_priv<O> = core::ptr::null_mut::<encode_match_priv<O>>();
        target.clear();
        data = (*emd).priv_0 as *mut encode_match_priv<O>;
        offsets = (*data).offset_f_priv as *mut *mut interval_node;
        export_helper((*data).len_f_priv as *mut interval_node, 16_i32, target);
        target.push(',');
        export_helper(*offsets.offset(0_i32 as isize), 4_i32, target);
        target.push(',');
        export_helper(*offsets.offset(1_i32 as isize), 16_i32, target);
        if (*data).flags_proto & 1_i32 << 4_i32 != 0 {
            target.push(',');
            export_helper(*offsets.offset(2_i32 as isize), 16_i32, target);
        }
        target.push(',');
        export_helper(*offsets.offset(7_i32 as isize), 16_i32, target);
    }
}
unsafe fn import_helper<O: Output>(
    output: &mut O,
    mut npp: *mut *mut interval_node,
    encodingp: *mut *const i8,
    flags: i32,
) {
    unsafe {
        let mut c: u8 = 0;
        let mut start: i32 = 1_i32;
        let mut depth: i32 = 0_i32;
        let mut encoding: *const i8 = core::ptr::null::<i8>();
        encoding = *encodingp;
        loop {
            let fresh1 = encoding;
            encoding = encoding.offset(1);
            c = *fresh1 as u8;
            if c == b'\0' {
                break;
            }
            let mut np: *mut interval_node = core::ptr::null_mut::<interval_node>();
            if c == b',' {
                break;
            }
            let bits: u8 = if c <= b'9' {
                c - b'0'
            } else {
                (c & !0x20) - b'A'
            };
            output.log_dump_ln(format_args!("got bits {}", bits));
            np = malloc(::core::mem::size_of::<interval_node>()) as *mut interval_node;
            interval_node_init(np, start, depth, flags);
            (*np).bits = bits as i8;
            depth += 1;
            depth;
            start += 1_i32 << bits;
            *npp = np;
            npp = &mut (*np).next;
        }
        *encodingp = encoding;
    }
}
pub(crate) unsafe fn optimal_encoding_import<O: Output>(
    output: &mut O,
    emd: *mut encode_match_data,
    mut encoding: *const i8,
) {
    unsafe {
        let mut data: *mut encode_match_priv<O> = core::ptr::null_mut::<encode_match_priv<O>>();
        let mut npp: *mut *mut interval_node = core::ptr::null_mut::<*mut interval_node>();
        let mut offsets: *mut *mut interval_node = core::ptr::null_mut::<*mut interval_node>();
        output.log_debug_ln(format_args!(
            "importing encoding: {}",
            CStr::from_ptr(encoding).to_str().unwrap(),
        ));
        data = (*emd).priv_0 as *mut encode_match_priv<O>;
        optimal_free::<O>(emd);
        optimal_init(output, emd, (*data).flags_notrait, (*data).flags_proto);
        data = (*emd).priv_0 as *mut encode_match_priv<O>;
        offsets = (*data).offset_f_priv as *mut *mut interval_node;
        npp = &mut (*data).len_f_priv as *mut *mut c_void as *mut c_void as *mut *mut interval_node;
        import_helper(output, npp, &mut encoding, -1_i32);
        npp = &mut *offsets.offset(0_i32 as isize) as *mut *mut interval_node;
        import_helper(output, npp, &mut encoding, 2_i32);
        npp = &mut *offsets.offset(1_i32 as isize) as *mut *mut interval_node;
        import_helper(output, npp, &mut encoding, 4_i32);
        if (*data).flags_proto & 1_i32 << 4_i32 != 0 {
            npp = &mut *offsets.offset(2_i32 as isize) as *mut *mut interval_node;
            import_helper(output, npp, &mut encoding, 4_i32);
        }
        npp = &mut *offsets.offset(7_i32 as isize) as *mut *mut interval_node;
        import_helper(output, npp, &mut encoding, 4_i32);
        output.log_debug(format_args!("imported encoding: "));
        optimal_dump(output, emd);
    }
}
pub(crate) unsafe fn optimal_init<O: Output>(
    output: &mut O,
    emd: *mut encode_match_data,
    flags_notrait: i32,
    flags_proto: i32,
) {
    unsafe {
        let mut data: *mut encode_match_priv<O> = core::ptr::null_mut::<encode_match_priv<O>>();
        let mut inpp: *mut *mut interval_node = core::ptr::null_mut::<*mut interval_node>();
        (*emd).priv_0 = malloc(::core::mem::size_of::<encode_match_priv<O>>());
        data = (*emd).priv_0 as *mut encode_match_priv<O>;
        memset(
            data as *mut c_void,
            0_i32,
            ::core::mem::size_of::<encode_match_priv<O>>(),
        );
        (*data).offset_f = Some(
            optimal_encode_int
                as unsafe fn(
                    &mut O,
                    i32,
                    *mut c_void,
                    *mut output_ctx,
                    *mut encode_int_bucket,
                ) -> f32,
        );
        (*data).len_f = Some(
            optimal_encode_int
                as unsafe fn(
                    &mut O,
                    i32,
                    *mut c_void,
                    *mut output_ctx,
                    *mut encode_int_bucket,
                ) -> f32,
        );
        inpp = malloc(::core::mem::size_of::<*mut interval_node>().wrapping_mul(8_i32 as usize))
            as *mut *mut interval_node;
        let fresh2 = &mut (*inpp.offset(0_i32 as isize));
        *fresh2 = core::ptr::null_mut::<interval_node>();
        let fresh3 = &mut (*inpp.offset(1_i32 as isize));
        *fresh3 = core::ptr::null_mut::<interval_node>();
        let fresh4 = &mut (*inpp.offset(2_i32 as isize));
        *fresh4 = core::ptr::null_mut::<interval_node>();
        let fresh5 = &mut (*inpp.offset(3_i32 as isize));
        *fresh5 = core::ptr::null_mut::<interval_node>();
        let fresh6 = &mut (*inpp.offset(4_i32 as isize));
        *fresh6 = core::ptr::null_mut::<interval_node>();
        let fresh7 = &mut (*inpp.offset(5_i32 as isize));
        *fresh7 = core::ptr::null_mut::<interval_node>();
        let fresh8 = &mut (*inpp.offset(6_i32 as isize));
        *fresh8 = core::ptr::null_mut::<interval_node>();
        let fresh9 = &mut (*inpp.offset(7_i32 as isize));
        *fresh9 = core::ptr::null_mut::<interval_node>();
        (*data).offset_f_priv = inpp as *mut c_void;
        (*data).len_f_priv = core::ptr::null_mut::<c_void>();
        (*data).flags_notrait = flags_notrait;
        (*data).flags_proto = flags_proto;
    }
}
pub(crate) unsafe fn optimal_free<O>(emd: *mut encode_match_data) {
    unsafe {
        let mut data: *mut encode_match_priv<O> = core::ptr::null_mut::<encode_match_priv<O>>();
        let mut inpp: *mut *mut interval_node = core::ptr::null_mut::<*mut interval_node>();
        let mut inp: *mut interval_node = core::ptr::null_mut::<interval_node>();
        data = (*emd).priv_0 as *mut encode_match_priv<O>;
        inpp = (*data).offset_f_priv as *mut *mut interval_node;
        if !inpp.is_null() {
            interval_node_delete(*inpp.offset(0_i32 as isize));
            interval_node_delete(*inpp.offset(1_i32 as isize));
            interval_node_delete(*inpp.offset(2_i32 as isize));
            interval_node_delete(*inpp.offset(3_i32 as isize));
            interval_node_delete(*inpp.offset(4_i32 as isize));
            interval_node_delete(*inpp.offset(5_i32 as isize));
            interval_node_delete(*inpp.offset(6_i32 as isize));
            interval_node_delete(*inpp.offset(7_i32 as isize));
        }
        free(inpp as *mut c_void);
        inp = (*data).len_f_priv as *mut interval_node;
        interval_node_delete(inp);
        (*data).offset_f_priv = core::ptr::null_mut::<c_void>();
        (*data).len_f_priv = core::ptr::null_mut::<c_void>();
    }
}
pub(crate) unsafe fn optimal_optimize<O: Output>(
    output: &mut O,
    emd: *mut encode_match_data,
    enum_next_f: Option<match_enum_next_f<O>>,
    matchp_enum: *mut c_void,
) {
    unsafe {
        let mut data: *mut encode_match_priv<O> = core::ptr::null_mut::<encode_match_priv<O>>();
        let mut mp: *const match_0 = core::ptr::null::<match_0>();
        let mut offset: *mut *mut interval_node = core::ptr::null_mut::<*mut interval_node>();
        static mut offset_arr: [[i32; 1000000]; 8] = [[0; 1000000]; 8];
        static mut offset_parr: [[i32; 1000000]; 8] = [[0; 1000000]; 8];
        static mut len_arr: [i32; 1000000] = [0; 1000000];
        let mut treshold: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        data = (*emd).priv_0 as *mut encode_match_priv<O>;
        memset(
            offset_arr.as_mut_ptr() as *mut c_void,
            0_i32,
            ::core::mem::size_of::<[[i32; 1000000]; 8]>(),
        );
        memset(
            offset_parr.as_mut_ptr() as *mut c_void,
            0_i32,
            ::core::mem::size_of::<[[i32; 1000000]; 8]>(),
        );
        memset(
            len_arr.as_mut_ptr() as *mut c_void,
            0_i32,
            ::core::mem::size_of::<[i32; 1000000]>(),
        );
        offset = (*data).offset_f_priv as *mut *mut interval_node;
        loop {
            mp = enum_next_f.expect("non-null function pointer")(output, matchp_enum);
            if mp.is_null() {
                break;
            }
            if (*mp).offset > 0_i32 as u32 {
                len_arr[(*mp).len as usize] += 1_i32;
                if len_arr[(*mp).len as usize] < 0_i32 {
                    output.log_error_ln(format_args!("len counter wrapped!"));
                }
            }
        }
        i = 65534_i32;
        while i >= 0_i32 {
            len_arr[i as usize] += len_arr[(i + 1_i32) as usize];
            if len_arr[i as usize] < 0_i32 {
                output.log_error_ln(format_args!("len counter wrapped!"));
            }
            i -= 1;
            i;
        }
        (*data).len_f_priv = optimize(
            output,
            len_arr.as_mut_ptr(),
            core::ptr::null_mut::<i32>(),
            16_i32,
            -1_i32,
        ) as *mut c_void;
        loop {
            mp = enum_next_f.expect("non-null function pointer")(output, matchp_enum);
            if mp.is_null() {
                break;
            }
            if (*mp).offset > 0_i32 as u32 {
                treshold = (*mp).len as i32 * 9_i32;
                treshold -= 1_i32
                    + optimal_encode_int(
                        output,
                        (*mp).len as i32,
                        (*data).len_f_priv,
                        core::ptr::null_mut::<output_ctx>(),
                        core::ptr::null_mut::<encode_int_bucket>(),
                    ) as i32;
                let current_block_86: usize;
                match (*mp).len as i32 {
                    0 => {
                        panic!("bad len");
                    }
                    1 => {
                        offset_parr[0_i32 as usize][(*mp).offset as usize] += treshold;
                        offset_arr[0_i32 as usize][(*mp).offset as usize] += 1_i32;
                        if offset_arr[0_i32 as usize][(*mp).offset as usize] < 0_i32 {
                            output.log_error_ln(format_args!("offset0 counter wrapped!"));
                        }
                        current_block_86 = 1352918242886884122;
                    }
                    2 => {
                        offset_parr[1_i32 as usize][(*mp).offset as usize] += treshold;
                        offset_arr[1_i32 as usize][(*mp).offset as usize] += 1_i32;
                        if offset_arr[1_i32 as usize][(*mp).offset as usize] < 0_i32 {
                            output.log_error_ln(format_args!("offset1 counter wrapped!"));
                        }
                        current_block_86 = 1352918242886884122;
                    }
                    3 => {
                        if (*data).flags_proto & 1_i32 << 4_i32 != 0 {
                            offset_parr[2_i32 as usize][(*mp).offset as usize] += treshold;
                            offset_arr[2_i32 as usize][(*mp).offset as usize] += 1_i32;
                            if offset_arr[2_i32 as usize][(*mp).offset as usize] < 0_i32 {
                                output.log_error_ln(format_args!("offset2 counter wrapped!",));
                            }
                            current_block_86 = 1352918242886884122;
                        } else {
                            current_block_86 = 7964364836200268519;
                        }
                    }
                    _ => {
                        current_block_86 = 7964364836200268519;
                    }
                }
                if current_block_86 == 7964364836200268519 {
                    offset_parr[7_i32 as usize][(*mp).offset as usize] += treshold;
                    offset_arr[7_i32 as usize][(*mp).offset as usize] += 1_i32;
                    if offset_arr[7_i32 as usize][(*mp).offset as usize] < 0_i32 {
                        output.log_error_ln(format_args!("offset7 counter wrapped!"));
                    }
                }
            }
        }
        i = 999998_i32;
        while i >= 0_i32 {
            j = 0_i32;
            while j < 8_i32 {
                offset_arr[j as usize][i as usize] += offset_arr[j as usize][(i + 1_i32) as usize];
                offset_parr[j as usize][i as usize] +=
                    offset_parr[j as usize][(i + 1_i32) as usize];
                j += 1;
                j;
            }
            i -= 1;
            i;
        }
        let fresh10 = &mut (*offset.offset(0_i32 as isize));
        *fresh10 = optimize(
            output,
            (offset_arr[0_i32 as usize]).as_mut_ptr(),
            (offset_parr[0_i32 as usize]).as_mut_ptr(),
            1_i32 << 2_i32,
            2_i32,
        );
        let fresh11 = &mut (*offset.offset(1_i32 as isize));
        *fresh11 = optimize(
            output,
            (offset_arr[1_i32 as usize]).as_mut_ptr(),
            (offset_parr[1_i32 as usize]).as_mut_ptr(),
            1_i32 << 4_i32,
            4_i32,
        );
        let fresh12 = &mut (*offset.offset(2_i32 as isize));
        *fresh12 = optimize(
            output,
            (offset_arr[2_i32 as usize]).as_mut_ptr(),
            (offset_parr[2_i32 as usize]).as_mut_ptr(),
            1_i32 << 4_i32,
            4_i32,
        );
        let fresh13 = &mut (*offset.offset(3_i32 as isize));
        *fresh13 = optimize(
            output,
            (offset_arr[3_i32 as usize]).as_mut_ptr(),
            (offset_parr[3_i32 as usize]).as_mut_ptr(),
            1_i32 << 4_i32,
            4_i32,
        );
        let fresh14 = &mut (*offset.offset(4_i32 as isize));
        *fresh14 = optimize(
            output,
            (offset_arr[4_i32 as usize]).as_mut_ptr(),
            (offset_parr[4_i32 as usize]).as_mut_ptr(),
            1_i32 << 4_i32,
            4_i32,
        );
        let fresh15 = &mut (*offset.offset(5_i32 as isize));
        *fresh15 = optimize(
            output,
            (offset_arr[5_i32 as usize]).as_mut_ptr(),
            (offset_parr[5_i32 as usize]).as_mut_ptr(),
            1_i32 << 4_i32,
            4_i32,
        );
        let fresh16 = &mut (*offset.offset(6_i32 as isize));
        *fresh16 = optimize(
            output,
            (offset_arr[6_i32 as usize]).as_mut_ptr(),
            (offset_parr[6_i32 as usize]).as_mut_ptr(),
            1_i32 << 4_i32,
            4_i32,
        );
        let fresh17 = &mut (*offset.offset(7_i32 as isize));
        *fresh17 = optimize(
            output,
            (offset_arr[7_i32 as usize]).as_mut_ptr(),
            (offset_parr[7_i32 as usize]).as_mut_ptr(),
            1_i32 << 4_i32,
            4_i32,
        );
        optimal_dump(output, emd);
    }
}
pub(crate) unsafe fn optimal_dump<O: Output>(output: &mut O, emd: *mut encode_match_data) {
    unsafe {
        let mut data: *mut encode_match_priv<O> = core::ptr::null_mut::<encode_match_priv<O>>();
        let mut offset: *mut *mut interval_node = core::ptr::null_mut::<*mut interval_node>();
        let mut len: *mut interval_node = core::ptr::null_mut::<interval_node>();
        data = (*emd).priv_0 as *mut encode_match_priv<O>;
        offset = (*data).offset_f_priv as *mut *mut interval_node;
        len = (*data).len_f_priv as *mut interval_node;
        output.log_debug(format_args!("lens:             "));
        interval_node_dump(output, false, len);
        output.log_debug(format_args!("offsets (len =1): "));
        interval_node_dump(output, false, *offset.offset(0_i32 as isize));
        output.log_debug(format_args!("offsets (len =2): "));
        interval_node_dump(output, false, *offset.offset(1_i32 as isize));
        if (*data).flags_proto & 1_i32 << 4_i32 != 0 {
            output.log_debug(format_args!("offsets (len =3): "));
            interval_node_dump(output, false, *offset.offset(2_i32 as isize));
        }
        output.log_debug(format_args!("offsets (len =8): "));
        interval_node_dump(output, false, *offset.offset(7_i32 as isize));
    }
}
unsafe fn interval_out<O: Output>(
    output: &mut O,
    out: *mut output_ctx,
    inp1: *mut interval_node,
    mut size: i32,
    flags_proto: i32,
) {
    unsafe {
        let mut buffer: [u8; 256] = [0; 256];
        let mut count: u8 = 0;
        let mut inp: *mut interval_node = core::ptr::null_mut::<interval_node>();
        count = 0_i32 as u8;
        memset(
            buffer.as_mut_ptr() as *mut c_void,
            0_i32,
            ::core::mem::size_of::<[u8; 256]>(),
        );
        inp = inp1;
        while !inp.is_null() {
            count = count.wrapping_add(1);
            count;
            output.log_dump_ln(format_args!(
                "bits {}, lo {}, hi {}",
                (*inp).bits as i32,
                (*inp).start & 0xff_i32,
                (*inp).start >> 8_i32,
            ));
            buffer[::core::mem::size_of::<[u8; 256]>().wrapping_sub(count as usize)] =
                (*inp).bits as u8;
            inp = (*inp).next;
        }
        while size > 0_i32 {
            let mut b: i32 = 0;
            b = buffer[::core::mem::size_of::<[u8; 256]>().wrapping_sub(size as usize)] as i32;
            output.log_dump_ln(format_args!("outputting nibble {}", b));
            if flags_proto & 1_i32 << 1_i32 != 0 {
                output_bits(output, out, 1_i32, b >> 3_i32);
                output_bits(output, out, 3_i32, b & 7_i32);
            } else {
                output_bits(output, out, 4_i32, b);
            }
            size -= 1;
            size;
        }
    }
}
pub(crate) unsafe fn optimal_out<O: Output>(
    output: &mut O,
    out: *mut output_ctx,
    emd: *mut encode_match_data,
) {
    unsafe {
        let mut data: *mut encode_match_priv<O> = core::ptr::null_mut::<encode_match_priv<O>>();
        let mut offset: *mut *mut interval_node = core::ptr::null_mut::<*mut interval_node>();
        let mut len: *mut interval_node = core::ptr::null_mut::<interval_node>();
        data = (*emd).priv_0 as *mut encode_match_priv<O>;
        offset = (*data).offset_f_priv as *mut *mut interval_node;
        len = (*data).len_f_priv as *mut interval_node;
        interval_out(
            output,
            out,
            *offset.offset(0_i32 as isize),
            4_i32,
            (*data).flags_proto,
        );
        interval_out(
            output,
            out,
            *offset.offset(1_i32 as isize),
            16_i32,
            (*data).flags_proto,
        );
        if (*data).flags_proto & 1_i32 << 4_i32 != 0 {
            interval_out(
                output,
                out,
                *offset.offset(2_i32 as isize),
                16_i32,
                (*data).flags_proto,
            );
        }
        interval_out(
            output,
            out,
            *offset.offset(7_i32 as isize),
            16_i32,
            (*data).flags_proto,
        );
        interval_out(output, out, len, 16_i32, (*data).flags_proto);
    }
}
