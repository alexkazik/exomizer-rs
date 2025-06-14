/*
 * Copyright (c) 2002 - 2005, 2013 Magnus Lind.
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
use crate::original::converted::progress::*;
use crate::original::converted::vec::*;
use crate::original::output::Output;
use crate::original::replacement::*;

#[derive(Clone)]
pub(crate) struct match_0 {
    pub(crate) offset: u32,
    pub(crate) len: u16,
    pub(crate) next: *const match_0,
}
pub(crate) struct pre_calc {
    pub(crate) single: *mut match_node,
    pub(crate) cache: *const match_0,
}
pub(crate) struct match_node {
    pub(crate) index: i32,
    pub(crate) next: *mut match_node,
}
pub(crate) struct match_ctx {
    pub(crate) m_pool: chunkpool,
    pub(crate) info: *mut pre_calc,
    pub(crate) rle: *mut u16,
    pub(crate) rle_r: *mut u16,
    pub(crate) buf: *const u8,
    pub(crate) noread_buf: *const u8,
    pub(crate) len: i32,
    pub(crate) max_offset: i32,
    pub(crate) max_len: i32,
}

pub(crate) struct match_cache_enum {
    pub(crate) ctx: *const match_ctx,
    pub(crate) tmp1: match_0,
    pub(crate) tmp2: match_0,
    pub(crate) pos: i32,
}
pub(crate) type match_enum_next_f<O> = unsafe fn(&mut O, *mut c_void) -> *const match_0;
pub(crate) struct match_concat_enum<O> {
    pub(crate) enum_iterator: vec_iterator,
    pub(crate) next_f: Option<match_enum_next_f<O>>,
    pub(crate) enum_current: *mut c_void,
}
pub(crate) unsafe fn match_new<O: Output>(
    output: &mut O,
    ctx: *mut match_ctx,
    mpp: *mut *mut match_0,
    mut len: i32,
    offset: i32,
) -> *mut match_0 {
    unsafe {
        let m: *mut match_0 = chunkpool_malloc(output, &mut (*ctx).m_pool) as *mut match_0;
        if len == 0_i32 {
            output.log_error_ln(format_args!("tried to allocate len0 match."));
            core::ptr::null_mut::<i32>();
        }
        if len > (*ctx).max_len {
            len = (*ctx).max_len;
        }
        (*m).len = len as u16;
        (*m).offset = offset as u32;
        (*m).next = *mpp;
        *mpp = m;
        m
    }
}
pub(crate) unsafe fn match_ctx_init<O: Output>(
    output: &mut O,
    ctx: *mut match_ctx,
    inbuf: *const buf,
    noread_inbuf: *const buf,
    max_len: i32,
    max_offset: i32,
    favor_speed: i32,
) {
    unsafe {
        let mut np: *mut match_node = core::ptr::null_mut::<match_node>();
        let mut prog: progress = progress {
            msg: core::ptr::null_mut::<i8>(),
            factor: 0.,
            offset: 0,
            last: 0,
        };
        let buf_len: i32 = buf_size(inbuf);
        let buf: *const u8 = buf_data(inbuf) as *const u8;
        let mut noread_buf: *const u8 = core::ptr::null::<u8>();
        let mut rle_map: *mut i8 = core::ptr::null_mut::<i8>();
        let mut c: i32 = 0;
        let mut i: i32 = 0;
        let mut val: i32 = 0;
        if !noread_inbuf.is_null() {
            noread_buf = buf_data(noread_inbuf) as *const u8;
        }
        (*ctx).info = calloc(
            (buf_len + 1_i32) as usize,
            ::core::mem::size_of::<pre_calc>(),
        ) as *mut pre_calc;
        (*ctx).rle = calloc((buf_len + 1_i32) as usize, ::core::mem::size_of::<u16>()) as *mut u16;
        (*ctx).rle_r =
            calloc((buf_len + 1_i32) as usize, ::core::mem::size_of::<u16>()) as *mut u16;
        chunkpool_init(&mut (*ctx).m_pool, ::core::mem::size_of::<match_0>() as i32);
        (*ctx).max_offset = max_offset;
        (*ctx).max_len = max_len;
        (*ctx).buf = buf;
        (*ctx).len = buf_len;
        (*ctx).noread_buf = noread_buf;
        if buf_len > 0_i32 {
            val = if !noread_buf.is_null() && *noread_buf.offset(0_i32 as isize) as i32 != 0_i32 {
                -1_i32
            } else {
                *buf.offset(0_i32 as isize) as i32
            };
            i = 1_i32;
            while i < buf_len {
                if val != -1_i32 && *buf.offset(i as isize) as i32 == val {
                    let mut len: i32 = *((*ctx).rle).offset((i - 1_i32) as isize) as i32 + 1_i32;
                    if len > (*ctx).max_len {
                        len = (*ctx).max_len;
                    }
                    *((*ctx).rle).offset(i as isize) = len as u16;
                } else {
                    *((*ctx).rle).offset(i as isize) = 0_i32 as u16;
                }
                val = if !noread_buf.is_null() && *noread_buf.offset(i as isize) as i32 != 0_i32 {
                    -1_i32
                } else {
                    *buf.offset(i as isize) as i32
                };
                i += 1;
                i;
            }
            val = if !noread_buf.is_null() && *noread_buf.offset(0_i32 as isize) as i32 != 0_i32 {
                -1_i32
            } else {
                *buf.offset(0_i32 as isize) as i32
            };
            i = buf_len - 2_i32;
            while i >= 0_i32 {
                if val != -1_i32 && *buf.offset(i as isize) as i32 == val {
                    let mut len_0: i32 =
                        *((*ctx).rle_r).offset((i + 1_i32) as isize) as i32 + 1_i32;
                    if len_0 > (*ctx).max_len {
                        len_0 = (*ctx).max_len;
                    }
                    *((*ctx).rle_r).offset(i as isize) = len_0 as u16;
                } else {
                    *((*ctx).rle_r).offset(i as isize) = 0_i32 as u16;
                }
                val = if !noread_buf.is_null() && *noread_buf.offset(i as isize) as i32 != 0_i32 {
                    -1_i32
                } else {
                    *buf.offset(i as isize) as i32
                };
                i -= 1;
                i;
            }
        }
        rle_map = malloc(65536_i32 as usize) as *mut i8;
        c = 0_i32;
        while c < 256_i32 {
            let mut prev_np: *mut match_node = core::ptr::null_mut::<match_node>();
            let mut trailing_np: *mut match_node = core::ptr::null_mut::<match_node>();
            let mut rle_len: u16 = 0;
            memset(rle_map as *mut c_void, 0_i32, 65536_i32 as usize);
            prev_np = core::ptr::null_mut::<match_node>();
            trailing_np = core::ptr::null_mut::<match_node>();
            i = 0_i32;
            while i < buf_len {
                if *buf.offset(i as isize) as i32 == c {
                    rle_len = *((*ctx).rle).offset(i as isize);
                    if !(*rle_map.offset(rle_len as isize) == 0
                        && *((*ctx).rle_r).offset(i as isize) as i32 > 16_i32)
                        && !(favor_speed != 0
                            && *((*ctx).rle_r).offset(i as isize) as i32 != 0_i32
                            && *((*ctx).rle).offset(i as isize) as i32 != 0_i32)
                    {
                        np = chunkpool_malloc(output, &mut (*ctx).m_pool) as *mut match_node;
                        (*np).index = i;
                        (*np).next = core::ptr::null_mut::<match_node>();
                        *rle_map.offset(rle_len as isize) = 1_i32 as i8;
                        output.log_dump_ln(format_args!(
                            "0) c = {}, added np idx {} -> {}",
                            c, i, 0_i32,
                        ));
                        if !prev_np.is_null() {
                            output.log_dump_ln(format_args!(
                                "1) c = {}, pointed np idx {} -> {}",
                                c,
                                (*prev_np).index,
                                i,
                            ));
                            (*prev_np).next = np;
                            if noread_buf.is_null()
                                || *noread_buf.offset((*prev_np).index as isize) as i32 == 0_i32
                            {
                                trailing_np = prev_np;
                            }
                        }
                        if !trailing_np.is_null()
                            && (noread_buf.is_null()
                                || *noread_buf.offset((*np).index as isize) as i32 == 0_i32)
                        {
                            while trailing_np != prev_np {
                                let tmp: *mut match_node = (*trailing_np).next;
                                (*trailing_np).next = np;
                                trailing_np = tmp;
                            }
                            trailing_np = core::ptr::null_mut::<match_node>();
                        }
                        let fresh0 = &mut (*((*ctx).info).offset(i as isize)).single;
                        *fresh0 = np;
                        prev_np = np;
                    }
                }
                i += 1;
                i;
            }
            while !trailing_np.is_null() {
                let tmp_0: *mut match_node = (*trailing_np).next;
                (*trailing_np).next = core::ptr::null_mut::<match_node>();
                trailing_np = tmp_0;
            }
            memset(rle_map as *mut c_void, 0_i32, 65536_i32 as usize);
            prev_np = core::ptr::null_mut::<match_node>();
            i = buf_len - 1_i32;
            while i >= 0_i32 {
                if *buf.offset(i as isize) as i32 == c {
                    rle_len = *((*ctx).rle_r).offset(i as isize);
                    np = (*((*ctx).info).offset(i as isize)).single;
                    if np.is_null() {
                        if *rle_map.offset(rle_len as isize) as i32 != 0
                            && !prev_np.is_null()
                            && rle_len as i32 > 0_i32
                        {
                            np = chunkpool_malloc(output, &mut (*ctx).m_pool) as *mut match_node;
                            (*np).index = i;
                            (*np).next = prev_np;
                            let fresh1 = &mut (*((*ctx).info).offset(i as isize)).single;
                            *fresh1 = np;
                            output.log_debug_ln(format_args!(
                                "2) c = {}, added np idx {} -> {}",
                                c,
                                i,
                                (*prev_np).index,
                            ));
                        }
                    } else if noread_buf.is_null()
                        || *noread_buf.offset((*np).index as isize) as i32 == 0_i32
                    {
                        prev_np = np;
                    }
                    if *((*ctx).rle_r).offset(i as isize) as i32 <= 0_i32 {
                        rle_len = (*((*ctx).rle).offset(i as isize) as i32 + 1_i32) as u16;
                        *rle_map.offset(rle_len as isize) = 1_i32 as i8;
                    }
                }
                i -= 1;
                i;
            }
            c += 1;
            c;
        }
        free(rle_map as *mut c_void);
        progress_init(
            output,
            &mut prog,
            b"building.directed.acyclic.graph.\0" as *const u8 as *const i8 as *mut i8,
            buf_len - 1_i32,
            0_i32,
        );
        i = buf_len - 1_i32;
        while i >= 0_i32 {
            let mut matches: *const match_0 = core::ptr::null::<match_0>();
            matches = matches_calc(output, ctx, i, favor_speed);
            let fresh2 = &mut (*((*ctx).info).offset(i as isize)).cache;
            *fresh2 = matches;
            progress_bump(output, &mut prog, i);
            i -= 1;
            i;
        }
        progress_free(output, &mut prog);
    }
}
pub(crate) unsafe fn match_ctx_free(ctx: *mut match_ctx) {
    unsafe {
        chunkpool_free(&mut (*ctx).m_pool);
        free((*ctx).info as *mut c_void);
        free((*ctx).rle as *mut c_void);
        free((*ctx).rle_r as *mut c_void);
    }
}
pub(crate) unsafe fn dump_matches<O: Output>(output: &mut O, mp: *const match_0) {
    unsafe {
        if mp.is_null() {
            output.log_debug_ln(format_args!(" (NULL)"));
        } else {
            if (*mp).offset > 0_i32 as u32 {
                output.log_debug_ln(format_args!(
                    " offset {}, len {}",
                    (*mp).offset,
                    (*mp).len as i32,
                ));
            }
            if !((*mp).next).is_null() {
                dump_matches(output, (*mp).next);
            }
        };
    }
}
pub(crate) unsafe fn matches_get(ctx: *const match_ctx, index: i32) -> *const match_0 {
    unsafe { (*((*ctx).info).offset(index as isize)).cache }
}
unsafe fn matches_calc<O: Output>(
    output: &mut O,
    ctx: *mut match_ctx,
    index: i32,
    favor_speed: i32,
) -> *const match_0 {
    unsafe {
        let mut buf: *const u8 = core::ptr::null::<u8>();
        let mut noread_buf: *const u8 = core::ptr::null::<u8>();
        let mut matches: *mut match_0 = core::ptr::null_mut::<match_0>();
        let mut mp: *mut match_0 = core::ptr::null_mut::<match_0>();
        let mut np: *const match_node = core::ptr::null::<match_node>();
        buf = (*ctx).buf;
        noread_buf = (*ctx).noread_buf;
        matches = core::ptr::null_mut::<match_0>();
        output.log_dump_ln(format_args!(
            "index {}, char '{}', rle {}, rle_r {}",
            index,
            *buf.offset(index as isize) as i32,
            *((*ctx).rle).offset(index as isize) as i32,
            *((*ctx).rle_r).offset(index as isize) as i32,
        ));
        mp = match_new(output, ctx, &mut matches, 1_i32, 0_i32);
        np = (*((*ctx).info).offset(index as isize)).single;
        if !np.is_null() {
            np = (*np).next;
        }
        while !np.is_null() {
            let mut mp_len: i32 = 0;
            let mut len: i32 = 0;
            let mut pos: i32 = 0;
            let mut offset: i32 = 0;
            if (*np).index > index + (*ctx).max_offset {
                break;
            }
            output.log_dump_ln(format_args!(
                "find lengths for index {} to index {}",
                index,
                (*np).index,
            ));
            mp_len = if (*mp).offset > 0_i32 as u32 {
                (*mp).len as i32
            } else {
                0_i32
            };
            output.log_dump_ln(format_args!(
                "0) comparing with current best [{}] off {} len {}",
                index,
                (*mp).offset,
                mp_len,
            ));
            offset = (*np).index - index;
            len = mp_len;
            pos = index + 1_i32 - len;
            while len > 1_i32
                && *buf.offset(pos as isize) as i32
                    == (if !noread_buf.is_null()
                        && *noread_buf.offset((pos + offset) as isize) as i32 != 0_i32
                    {
                        -1_i32
                    } else {
                        *buf.offset((pos + offset) as isize) as i32
                    })
            {
                let offset1: i32 = *((*ctx).rle_r).offset(pos as isize) as i32;
                let offset2: i32 = *((*ctx).rle_r).offset((pos + offset) as isize) as i32;
                let offset_0: i32 = if offset1 < offset2 { offset1 } else { offset2 };
                output.log_dump_ln(format_args!(
                    "1) compared sucesssfully [{}] {} {}",
                    index,
                    pos,
                    pos + offset_0,
                ));
                len -= 1_i32 + offset_0;
                pos += 1_i32 + offset_0;
            }
            if len <= 1_i32 {
                if offset < 17_i32 {
                    mp = match_new(output, ctx, &mut matches, 1_i32, offset);
                }
                len = mp_len;
                pos = index - len;
                while len <= (*ctx).max_len
                    && pos >= 0_i32
                    && *buf.offset(pos as isize) as i32
                        == (if !noread_buf.is_null()
                            && *noread_buf.offset((pos + offset) as isize) as i32 != 0_i32
                        {
                            -1_i32
                        } else {
                            *buf.offset((pos + offset) as isize) as i32
                        })
                {
                    output.log_dump_ln(format_args!(
                        "2) compared sucesssfully [{}] {} {}",
                        index,
                        pos,
                        pos + offset,
                    ));
                    len += 1;
                    len;
                    pos -= 1;
                    pos;
                }
                if len > mp_len || favor_speed == 0 && len == mp_len {
                    mp = match_new(output, ctx, &mut matches, index - pos, offset);
                }
                if len > (*ctx).max_len {
                    break;
                }
                if pos < 0_i32 {
                    break;
                }
            }
            np = (*np).next;
        }
        output.log_debug_ln(format_args!("adding matches for index {} to cache", index,));
        dump_matches(output, matches);
        matches
    }
}
unsafe fn match_keep_this(mp: *const match_0) -> i32 {
    unsafe {
        let mut val: i32 = 1_i32;
        if (*mp).len as i32 == 1_i32 && (*mp).offset > 34_i32 as u32 {
            val = 0_i32;
        }
        val
    }
}
unsafe fn match_cache_peek<O: Output>(
    output: &mut O,
    ctx: *const match_ctx,
    pos: i32,
    litpp: *mut *const match_0,
    seqpp: *mut *const match_0,
    lit_tmp: *mut match_0,
    val_tmp: *mut match_0,
) {
    unsafe {
        let mut litp: *const match_0 = core::ptr::null::<match_0>();
        let mut seqp: *const match_0 = core::ptr::null::<match_0>();
        let mut val: *const match_0 = core::ptr::null::<match_0>();
        seqp = core::ptr::null::<match_0>();
        litp = core::ptr::null::<match_0>();
        if pos >= 0_i32 {
            val = matches_get(ctx, pos);
            litp = val;
            while (*litp).offset != 0_i32 as u32 {
                litp = (*litp).next;
            }
            if *((*ctx).rle_r).offset(pos as isize) as i32 > 0_i32
                && *((*ctx).rle).offset((pos + 1_i32) as isize) as i32 > 0_i32
            {
                (*val_tmp).offset = 1_i32 as u32;
                (*val_tmp).len = *((*ctx).rle).offset((pos + 1_i32) as isize);
                (*val_tmp).next = val as *mut match_0;
                val = val_tmp;
                output.log_debug_ln(format_args!(
                    "injecting rle val({},{})",
                    (*val).len as i32,
                    (*val).offset,
                ));
            }
            while !val.is_null() {
                if (*val).offset != 0_i32 as u32 {
                    if match_keep_this(val) != 0
                        && (seqp.is_null()
                            || (*val).len as i32 > (*seqp).len as i32
                            || (*val).len as i32 == (*seqp).len as i32
                                && (*val).offset < (*seqp).offset)
                    {
                        seqp = val;
                    }
                    if (*litp).offset == 0_i32 as u32 || (*litp).offset > (*val).offset {
                        output.log_debug(format_args!(
                            "val({},{})",
                            (*val).len as i32,
                            (*val).offset,
                        ));
                        if !lit_tmp.is_null() {
                            let mut diff: i32 = 0;
                            let mut tmp2: match_0 = match_0 {
                                offset: 0,
                                len: 0,
                                next: core::ptr::null::<match_0>(),
                            };
                            tmp2.clone_from(&*val);
                            tmp2.len = 1_i32 as u16;
                            diff = *((*ctx).rle)
                                .offset((pos as u32).wrapping_add((*val).offset) as isize)
                                as i32;
                            if tmp2.offset > diff as u32 {
                                tmp2.offset = (tmp2.offset).wrapping_sub(diff as u32);
                            } else {
                                tmp2.offset = 1_i32 as u32;
                            }
                            output.log_debug(format_args!(
                                "=> litp({},{})",
                                tmp2.len as i32, tmp2.offset,
                            ));
                            if match_keep_this(&mut tmp2) != 0 {
                                output.log_debug(format_args!(", keeping"));
                                *lit_tmp = tmp2;
                                litp = lit_tmp;
                            }
                        }
                        output.log_debug_ln(format_args!(""));
                    }
                }
                val = (*val).next;
            }
        }
        if !litpp.is_null() {
            *litpp = litp;
        }
        if !seqpp.is_null() {
            *seqpp = seqp;
        }
    }
}
pub(crate) unsafe fn match_cache_get_enum(ctx: *mut match_ctx, mpce: *mut match_cache_enum) {
    unsafe {
        (*mpce).ctx = ctx;
        (*mpce).pos = (*ctx).len - 1_i32;
    }
}
pub(crate) unsafe fn match_cache_enum_get_next<O: Output>(
    output: &mut O,
    match_cache_enum: *mut c_void,
) -> *const match_0 {
    unsafe {
        let mut val: *const match_0 = core::ptr::null::<match_0>();
        let mut lit: *const match_0 = core::ptr::null::<match_0>();
        let mut seq: *const match_0 = core::ptr::null::<match_0>();
        let mut mpce: *mut match_cache_enum = core::ptr::null_mut::<match_cache_enum>();
        mpce = match_cache_enum as *mut match_cache_enum;
        match_cache_peek(
            output,
            (*mpce).ctx,
            (*mpce).pos,
            &mut lit,
            &mut seq,
            &mut (*mpce).tmp1,
            &mut (*mpce).tmp2,
        );
        val = lit;
        if lit.is_null() {
            (*mpce).pos = (*(*mpce).ctx).len - 1_i32;
        } else if !seq.is_null() {
            let mut t1: match_0 = match_0 {
                offset: 0,
                len: 0,
                next: core::ptr::null::<match_0>(),
            };
            let mut t2: match_0 = match_0 {
                offset: 0,
                len: 0,
                next: core::ptr::null::<match_0>(),
            };
            let mut next: *const match_0 = core::ptr::null::<match_0>();
            match_cache_peek(
                output,
                (*mpce).ctx,
                (*mpce).pos - 1_i32,
                core::ptr::null_mut::<*const match_0>(),
                &mut next,
                &mut t1,
                &mut t2,
            );
            if next.is_null()
                || (*seq).len as i32
                    >= (*next).len as i32
                        + ((*mpce).pos & 1_i32 != 0 && ((*next).len as i32) < 3_i32) as i32
            {
                val = seq;
            }
        }
        if !val.is_null() {
            output.log_debug_ln(format_args!(
                "Using len {:05}, offset, {:05}",
                (*val).len as i32,
                (*val).offset,
            ));
            (*mpce).pos -= (*val).len as i32;
        }
        val
    }
}
pub(crate) unsafe fn match_concat_get_enum<O>(
    next_f: Option<match_enum_next_f<O>>,
    data_vec: *mut vec,
    mpcce: *mut match_concat_enum<O>,
) {
    unsafe {
        vec_get_iterator(data_vec, &mut (*mpcce).enum_iterator);
        (*mpcce).next_f = next_f;
        (*mpcce).enum_current = vec_iterator_next(&mut (*mpcce).enum_iterator);
    }
}
pub(crate) unsafe fn match_concat_enum_get_next<O>(
    output: &mut O,
    match_concat_enum: *mut c_void,
) -> *const match_0 {
    unsafe {
        let e: *mut match_concat_enum<O> = match_concat_enum as *mut match_concat_enum<O>;
        let mut mp: *const match_0 = core::ptr::null::<match_0>();
        loop {
            if ((*e).enum_current).is_null() {
                (*e).enum_current = vec_iterator_next(&mut (*e).enum_iterator);
                break;
            } else {
                mp = ((*e).next_f).expect("non-null function pointer")(output, (*e).enum_current);
                if !mp.is_null() {
                    break;
                }
                (*e).enum_current = vec_iterator_next(&mut (*e).enum_iterator);
            }
        }
        mp
    }
}
