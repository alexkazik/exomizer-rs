/*
 * Copyright (c) 2002 - 2018 Magnus Lind.
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

use crate::original::converted::r#match::*;
use crate::original::converted::optimal::*;
use crate::original::converted::progress::*;
use crate::original::output::Output;
use crate::original::replacement::*;

pub(crate) struct search_node {
    pub(crate) index: i32,
    pub(crate) match_0: match_0,
    pub(crate) total_offset: u32,
    pub(crate) total_score: f32,
    pub(crate) prev: *mut search_node,
    pub(crate) latest_offset: u32,
}

pub(crate) struct match_snp_enum {
    pub(crate) startp: *const search_node,
    pub(crate) currp: *const search_node,
}
unsafe fn update_snp(
    snp: *mut search_node,
    total_score: f32,
    total_offset: u32,
    prev: *mut search_node,
    match_0: *mut match_0,
    flags_proto: i32,
) {
    unsafe {
        let mut latest_offset: u32 = 0_i32 as u32;
        (*snp).total_score = total_score;
        (*snp).total_offset = total_offset;
        (*snp).prev = prev;
        ((*snp).match_0).clone_from(&*match_0);
        if flags_proto & 1_i32 << 5_i32 != 0_i32 && (*match_0).offset == 0_i32 as u32 {
            let prev_match: *mut match_0 = &mut (*prev).match_0;
            if (*prev_match).offset > 0_i32 as u32 {
                latest_offset = (*prev_match).offset;
            }
        }
        (*snp).latest_offset = latest_offset;
    }
}
pub(crate) unsafe fn search_buffer<O: Output>(
    output: &mut O,
    ctx: *mut match_ctx,
    emd: *mut encode_match_data,
    flags_proto: i32,
    flags_notrait: i32,
    max_sequence_length: i32,
    greedy: i32,
    result: *mut *mut search_node,
) {
    unsafe {
        let mut prog: progress = progress {
            msg: core::ptr::null_mut::<i8>(),
            factor: 0.,
            offset: 0,
            last: 0,
        };
        let mut sn_arr: *mut search_node = core::ptr::null_mut::<search_node>();
        let mut mp: *const match_0 = core::ptr::null::<match_0>();
        let mut snp: *mut search_node = core::ptr::null_mut::<search_node>();
        let mut best_copy_snp: *mut search_node = core::ptr::null_mut::<search_node>();
        let mut best_copy_len: i32 = 0;
        let mut best_rle_snp: *mut search_node = core::ptr::null_mut::<search_node>();
        let use_literal_sequences: i32 = (flags_notrait & 1_i32 << 0_i32 == 0) as i32;
        let mut skip_len0123_mirrors: i32 = flags_notrait & 1_i32 << 2_i32;
        let mut len: i32 = (*ctx).len + 1_i32;
        if skip_len0123_mirrors != 0 {
            if flags_proto & 1_i32 << 4_i32 != 0 {
                skip_len0123_mirrors = 4_i32;
            } else {
                skip_len0123_mirrors = 3_i32;
            }
        }
        progress_init(
            output,
            &mut prog,
            b"finding.shortest.path.\0" as *const u8 as *const i8 as *mut i8,
            len,
            0_i32,
        );
        sn_arr = malloc((len as usize).wrapping_mul(::core::mem::size_of::<search_node>()))
            as *mut search_node;
        memset(
            sn_arr as *mut c_void,
            0_i32,
            (len as usize).wrapping_mul(::core::mem::size_of::<search_node>()),
        );
        len -= 1;
        len;
        snp = &mut *sn_arr.offset(len as isize) as *mut search_node;
        (*snp).index = len;
        (*snp).match_0.offset = 0_i32 as u32;
        (*snp).match_0.len = 0_i32 as u16;
        (*snp).total_offset = 0_i32 as u32;
        (*snp).total_score = 0_i32 as f32;
        (*snp).prev = core::ptr::null_mut::<search_node>();
        (*snp).latest_offset = 0_i32 as u32;
        best_copy_snp = snp;
        best_copy_len = 0_i32;
        best_rle_snp = core::ptr::null_mut::<search_node>();
        loop {
            let mut prev_score: f32 = 0.;
            let mut latest_offset_sum: f32 = 0.;
            if use_literal_sequences != 0 {
                snp = &mut *sn_arr.offset(len as isize) as *mut search_node;
                if ((*snp).match_0.offset != 0_i32 as u32 || (*snp).match_0.len as i32 != 1_i32)
                    && ((*best_copy_snp).total_score as f64 + best_copy_len as f64 * 8.0f64
                        - (*snp).total_score as f64
                        > 0.0f64
                        || best_copy_len > max_sequence_length)
                {
                    output.log_debug_ln(format_args!(
                        "best copy start moved to index {}",
                        (*snp).index,
                    ));
                    best_copy_snp = snp;
                    best_copy_len = 0_i32;
                } else {
                    let copy_score: f32 =
                        (best_copy_len as f64 * 8.0f64 + (1.0f64 + 17.0f64 + 17.0f64)) as f32;
                    let total_copy_score: f32 = (*best_copy_snp).total_score + copy_score;
                    output.log_debug_ln(format_args!(
                        "total score {:.1}, copy total score {:.1}",
                        (*snp).total_score as f64,
                        total_copy_score as f64,
                    ));
                    if (*snp).total_score > total_copy_score
                        && best_copy_len <= max_sequence_length
                        && !(skip_len0123_mirrors != 0
                            && best_copy_len > 255_i32
                            && (best_copy_len & 255_i32) < 2_i32)
                    {
                        let mut local_m: match_0 = match_0 {
                            offset: 0,
                            len: 0,
                            next: core::ptr::null::<match_0>(),
                        };
                        output.log_debug_ln(format_args!(
                            "copy index {}, len {}, total {:.1}, copy {:.1}",
                            (*snp).index,
                            best_copy_len,
                            (*snp).total_score as f64,
                            total_copy_score as f64,
                        ));
                        local_m.len = best_copy_len as u16;
                        local_m.offset = 0_i32 as u32;
                        local_m.next = core::ptr::null::<match_0>();
                        update_snp(
                            snp,
                            total_copy_score,
                            (*best_copy_snp).total_offset,
                            best_copy_snp,
                            &mut local_m,
                            flags_proto,
                        );
                    }
                }
            }
            snp = &mut *sn_arr.offset(len as isize) as *mut search_node;
            if best_rle_snp.is_null()
                || (*snp).index + max_sequence_length < (*best_rle_snp).index
                || ((*snp).index + *((*ctx).rle_r).offset((*snp).index as isize) as i32)
                    < (*best_rle_snp).index
            {
                if *((*ctx).rle).offset((*snp).index as isize) as i32 > 0_i32 {
                    best_rle_snp = snp;
                    output.log_debug_ln(format_args!(
                        "resetting best_rle at index {}, len {}",
                        (*snp).index,
                        *((*ctx).rle).offset((*snp).index as isize) as i32,
                    ));
                } else {
                    best_rle_snp = core::ptr::null_mut::<search_node>();
                }
            } else if *((*ctx).rle).offset((*snp).index as isize) as i32 > 0_i32
                && (*snp).index + *((*ctx).rle_r).offset((*snp).index as isize) as i32
                    >= (*best_rle_snp).index
            {
                let mut best_rle_score: f32 = 0.;
                let mut total_best_rle_score: f32 = 0.;
                let mut snp_rle_score: f32 = 0.;
                let mut total_snp_rle_score: f32 = 0.;
                let mut rle_m: match_0 = match_0 {
                    offset: 0,
                    len: 0,
                    next: core::ptr::null::<match_0>(),
                };
                output.log_debug_ln(format_args!(
                    "challenger len {}, index {}, ruling len {}, index {}",
                    *((*ctx).rle_r).offset((*snp).index as isize) as i32,
                    (*snp).index,
                    *((*ctx).rle_r).offset((*best_rle_snp).index as isize) as i32,
                    (*best_rle_snp).index,
                ));
                rle_m.len = *((*ctx).rle).offset((*best_rle_snp).index as isize);
                rle_m.offset = 1_i32 as u32;
                best_rle_score = optimal_encode(
                    output,
                    &mut rle_m,
                    emd,
                    (*best_rle_snp).latest_offset,
                    core::ptr::null_mut::<encode_match_buckets>(),
                );
                total_best_rle_score = (*best_rle_snp).total_score + best_rle_score;
                rle_m.len = *((*ctx).rle).offset((*snp).index as isize);
                rle_m.offset = 1_i32 as u32;
                snp_rle_score = optimal_encode(
                    output,
                    &mut rle_m,
                    emd,
                    (*snp).latest_offset,
                    core::ptr::null_mut::<encode_match_buckets>(),
                );
                total_snp_rle_score = (*snp).total_score + snp_rle_score;
                if total_snp_rle_score <= total_best_rle_score {
                    output.log_debug_ln(format_args!(
                        "prospect len {}, index {}, ({:.1}+{:.1}) ruling len {}, index {} ({:.1}+{:.1})"
                        ,
                        *((*ctx).rle).offset((*snp).index as isize) as i32,
                        (*snp).index,
                        (*snp).total_score as f64,
                        snp_rle_score as f64,
                        *((*ctx).rle).offset((*best_rle_snp).index as isize)
                            as i32,
                        (*best_rle_snp).index,
                        (*best_rle_snp).total_score as f64,
                        best_rle_score as f64,
                    ));
                    best_rle_snp = snp;
                    output.log_debug_ln(format_args!(
                        "setting current best_rle: index {}, len {}",
                        (*snp).index,
                        rle_m.len as i32,
                    ));
                }
            }
            if !best_rle_snp.is_null() && best_rle_snp != snp {
                let mut rle_score: f32 = 0.;
                let mut total_rle_score: f32 = 0.;
                let mut local_m_0: match_0 = match_0 {
                    offset: 0,
                    len: 0,
                    next: core::ptr::null::<match_0>(),
                };
                local_m_0.len = ((*best_rle_snp).index - (*snp).index) as u16;
                local_m_0.offset = 1_i32 as u32;
                rle_score = optimal_encode(
                    output,
                    &mut local_m_0,
                    emd,
                    (*best_rle_snp).latest_offset,
                    core::ptr::null_mut::<encode_match_buckets>(),
                );
                total_rle_score = (*best_rle_snp).total_score + rle_score;
                output.log_debug_ln(format_args!(
                    "comparing index {} ({:.1}) with rle index {}, len {}, total score {:.1} {:.1}",
                    (*snp).index,
                    (*snp).total_score as f64,
                    (*best_rle_snp).index,
                    local_m_0.len as i32,
                    (*best_rle_snp).total_score as f64,
                    rle_score as f64,
                ));
                if (*snp).total_score > total_rle_score {
                    output.log_debug_ln(format_args!(
                        "rle index {}, len {}, total {:.1}, rle {:.1}",
                        (*snp).index,
                        local_m_0.len as i32,
                        (*snp).total_score as f64,
                        total_rle_score as f64,
                    ));
                    update_snp(
                        snp,
                        total_rle_score,
                        ((*best_rle_snp).total_offset).wrapping_add(1_i32 as u32),
                        best_rle_snp,
                        &mut local_m_0,
                        flags_proto,
                    );
                }
            }
            if len == 0_i32 {
                break;
            }
            mp = matches_get(ctx, len - 1_i32);
            output.log_dump_ln(format_args!(
                "matches for index {} with total score {:.1}",
                len - 1_i32,
                (*snp).total_score as f64,
            ));
            prev_score = (*sn_arr.offset(len as isize)).total_score;
            latest_offset_sum = (*sn_arr.offset(len as isize)).total_offset as f32;
            while !mp.is_null() {
                let mut next: *const match_0 = core::ptr::null::<match_0>();
                let mut end_len: i32 = 0;
                let mut tmp: match_0 = match_0 {
                    offset: 0,
                    len: 0,
                    next: core::ptr::null::<match_0>(),
                };
                let mut bucket_len_start: i32 = 0;
                let mut score: f32 = 0.;
                let mut prev_snp: *mut search_node = core::ptr::null_mut::<search_node>();
                next = (*mp).next;
                end_len = 1_i32;
                tmp.clone_from(&*mp);
                tmp.next = core::ptr::null::<match_0>();
                bucket_len_start = 0_i32;
                prev_snp = &mut *sn_arr.offset(len as isize) as *mut search_node;
                tmp.len = (*mp).len;
                while tmp.len as i32 >= end_len {
                    let mut total_score: f32 = 0.;
                    let mut total_offset: u32 = 0;
                    let mut match_buckets: encode_match_buckets = {
                        encode_match_buckets {
                            len: {
                                encode_int_bucket {
                                    start: 0_i32 as u32,
                                    end: 0_i32 as u32,
                                }
                            },
                            offset: {
                                encode_int_bucket {
                                    start: 0_i32 as u32,
                                    end: 0_i32 as u32,
                                }
                            },
                        }
                    };
                    output.log_dump_ln(format_args!(
                        "mp[{}, {}], tmp[{}, {}]",
                        (*mp).offset,
                        (*mp).len as i32,
                        tmp.offset,
                        tmp.len as i32,
                    ));
                    if bucket_len_start == 0_i32
                        || (tmp.len as i32) < 4_i32
                        || (tmp.len as i32) < bucket_len_start
                        || skip_len0123_mirrors != 0
                            && tmp.len as i32 > 255_i32
                            && (tmp.len as i32 & 255_i32) < skip_len0123_mirrors
                    {
                        score = optimal_encode(
                            output,
                            &mut tmp,
                            emd,
                            (*prev_snp).latest_offset,
                            &mut match_buckets,
                        );
                        bucket_len_start = match_buckets.len.start as i32;
                    }
                    total_score = prev_score + score;
                    total_offset = (latest_offset_sum + tmp.offset as f32) as u32;
                    snp = &mut *sn_arr.offset((len - tmp.len as i32) as isize) as *mut search_node;
                    output.log_dump(format_args!(
                        "[{:05}] cmp [{:05}, {:05} score {:.1} + {:.1}] with {:.1}",
                        len,
                        tmp.offset,
                        tmp.len as i32,
                        prev_score as f64,
                        score as f64,
                        (*snp).total_score as f64,
                    ));
                    if (total_score as f64) < 100000000.0f64
                        && ((*snp).match_0.len as i32 == 0_i32
                            || total_score < (*snp).total_score
                            || total_score == (*snp).total_score
                                && total_offset < (*snp).total_offset
                                && (greedy != 0
                                    || (*snp).match_0.len as i32 == 1_i32
                                        && (*snp).match_0.offset > 8_i32 as u32
                                    || tmp.offset > 48_i32 as u32
                                    || tmp.len as i32 > 15_i32))
                    {
                        output.log_dump(format_args!(", replaced"));
                        (*snp).index = len - tmp.len as i32;
                        update_snp(
                            snp,
                            total_score,
                            total_offset,
                            prev_snp,
                            &mut tmp,
                            flags_proto,
                        );
                    }
                    output.log_dump_ln(format_args!(""));
                    tmp.len = (tmp.len).wrapping_sub(1);
                    tmp.len;
                }
                output.log_dump_ln(format_args!(
                    "tmp.len {}, ctx->rle[{}] {}",
                    tmp.len as i32,
                    len - tmp.len as i32,
                    *((*ctx).rle).offset((len - tmp.len as i32) as isize) as i32,
                ));
                mp = next;
            }
            len -= 1;
            len;
            best_copy_len += 1;
            best_copy_len;
            progress_bump(output, &mut prog, len);
        }
        if len > 0_i32 && mp.is_null() {
            output.log_error_ln(format_args!("No matches at len {}.", len));
        }
        progress_free(output, &mut prog);
        *result = sn_arr;
    }
}
pub(crate) unsafe fn match_snp_get_enum(snp: *const search_node, snpe: *mut match_snp_enum) {
    unsafe {
        (*snpe).startp = snp;
        (*snpe).currp = snp;
    }
}
pub(crate) unsafe fn match_snp_enum_get_next<O>(
    output: &mut O,
    match_snp_enum: *mut c_void,
) -> *const match_0 {
    unsafe {
        let snpe: *mut match_snp_enum = match_snp_enum as *mut match_snp_enum;
        let mut val: *const match_0 = core::ptr::null::<match_0>();
        let current_block_3: usize;
        if ((*snpe).currp).is_null() {
            current_block_3 = 14381721519910822911;
        } else {
            val = &(*(*snpe).currp).match_0;
            if (*val).len as i32 == 0_i32 {
                current_block_3 = 14381721519910822911;
            } else {
                (*snpe).currp = (*(*snpe).currp).prev;
                current_block_3 = 4906268039856690917;
            }
        }
        if current_block_3 == 14381721519910822911 {
            val = core::ptr::null::<match_0>();
            (*snpe).currp = (*snpe).startp;
        }
        val
    }
}
