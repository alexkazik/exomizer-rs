/*
 * Copyright (c) 2005, 2013, 2015 Magnus Lind.
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
use crate::original::converted::exodec::*;
use crate::original::converted::r#match::*;
use crate::original::converted::optimal::*;
use crate::original::converted::output::*;
use crate::original::converted::search::*;
use crate::original::converted::vec::*;
use crate::original::crunch_options::CrunchOptions;
use crate::original::output::Output;
use crate::original::replacement::*;
use alloc::string::String;
use core::ffi::CStr;
use core::marker::PhantomData;

#[derive(Clone)]
pub(crate) struct crunch_options<'a> {
    pub(crate) imported_encoding: *const i8,
    pub(crate) max_passes: i32,
    pub(crate) max_len: i32,
    pub(crate) max_offset: i32,
    pub(crate) favor_speed: i32,
    pub(crate) output_header: i32,
    pub(crate) flags_proto: i32,
    pub(crate) flags_notrait: i32,
    pub(crate) direction_forward: i32,
    pub(crate) write_reverse: i32,
    // this is to guarantee that the CString imported_encoding is pointing to is not dropped before this struct
    pub(crate) imported_encoding_source: PhantomData<&'a ()>,
}
pub(crate) struct crunch_info {
    pub(crate) traits_used: i32,
    pub(crate) max_len: i32,
    pub(crate) max_offset: i32,
    pub(crate) needed_safety_offset: i32,
}
pub(crate) struct io_bufs {
    pub(crate) in_0: buf,
    pub(crate) in_off: i32,
    pub(crate) out: buf,
    pub(crate) info: crunch_info,
}
pub(crate) struct decrunch_options<'a> {
    pub(crate) imported_encoding: *const i8,
    pub(crate) flags_proto: i32,
    pub(crate) direction_forward: i32,
    pub(crate) write_reverse: i32,
    // this is to guarantee that the CString imported_encoding is pointing to is not dropped before this struct
    pub(crate) imported_encoding_source: PhantomData<&'a ()>,
}
unsafe fn do_output_backwards<O: Output>(
    output: &mut O,
    ctx: *mut match_ctx,
    mut snp: *mut search_node,
    emd: *mut encode_match_data,
    options: *const crunch_options,
    outbuf: *mut buf,
    infop: *mut crunch_info,
) {
    unsafe {
        let mut pos: i32 = 0;
        let mut pos_diff: i32 = 0;
        let mut max_diff: i32 = 0;
        let mut diff: i32 = 0;
        let mut traits_used: i32 = 0_i32;
        let mut max_len: i32 = 0_i32;
        let mut max_offset: i32 = 0_i32;
        let mut old: *mut output_ctx = core::ptr::null_mut::<output_ctx>();
        let mut out: output_ctx = output_ctx {
            bitbuf: 0,
            bitcount: 0,
            pos: 0,
            buf: core::ptr::null_mut::<buf>(),
            flags_proto: 0,
        };
        let mut initial_snp: *mut search_node = core::ptr::null_mut::<search_node>();
        let mut initial_len: i32 = 0;
        let mut alignment: i32 = 0_i32;
        let mut measure_alignment: i32 = 0;
        let mut len0123skip: i32 = 3_i32;
        old = (*emd).out;
        (*emd).out = &mut out;
        initial_len = buf_size(outbuf);
        initial_snp = snp;
        measure_alignment = (*options).flags_proto & 1_i32 << 3_i32;
        if (*options).flags_proto & 1_i32 << 4_i32 != 0 {
            len0123skip = 4_i32;
        }
        loop {
            buf_remove(outbuf, initial_len, -1_i32);
            snp = initial_snp;
            output_ctx_init(&mut out, (*options).flags_proto, outbuf);
            output_bits(output, &mut out, alignment, 0_i32);
            pos = output_get_pos(&mut out) as i32;
            pos_diff = pos;
            max_diff = 0_i32;
            if !snp.is_null() {
                output.log_dump_ln(format_args!("pos ${:04x}", out.pos));
                output_gamma_code(output, &mut out, 16_i32);
                output_bits(output, &mut out, 1_i32, 0_i32);
                diff = (output_get_pos(&mut out)).wrapping_sub(pos_diff as u32) as i32;
                if diff > max_diff {
                    max_diff = diff;
                }
                output.log_dump_ln(format_args!("pos ${:04x}", out.pos));
                output.log_dump_ln(format_args!("------------"));
            }
            while !snp.is_null() {
                let mut mp: *const match_0 = core::ptr::null::<match_0>();
                mp = &mut (*snp).match_0;
                if !mp.is_null() && (*mp).len as i32 > 0_i32 {
                    if (*mp).offset == 0_i32 as u32 {
                        let splitLitSeq: i32 = ((*(*snp).prev).match_0.len as i32 == 0_i32
                            && (*options).flags_proto & 1_i32 << 2_i32 != 0)
                            as i32;
                        let mut i: i32 = 0_i32;
                        if (*mp).len as i32 > 1_i32 {
                            let mut len: i32 = (*mp).len as i32;
                            if splitLitSeq != 0 {
                                len -= 1;
                                len;
                            }
                            while i < len {
                                output_byte(
                                    &mut out,
                                    *((*ctx).buf).offset(((*snp).index + i) as isize),
                                );
                                i += 1;
                                i;
                            }
                            output_bits(output, &mut out, 16_i32, len);
                            output_gamma_code(output, &mut out, 17_i32);
                            output_bits(output, &mut out, 1_i32, 0_i32);
                            output.log_dump_ln(format_args!(
                                "[{}] literal copy len {}",
                                out.pos, len,
                            ));
                            traits_used |= 1_i32 << 0_i32;
                            if len > max_len {
                                max_len = len;
                            }
                        }
                        if i < (*mp).len as i32 {
                            output.log_dump_ln(format_args!(
                                "[{}] literal ${:02x}",
                                out.pos,
                                *((*ctx).buf).offset(((*snp).index + i) as isize) as i32,
                            ));
                            output_byte(
                                &mut out,
                                *((*ctx).buf).offset(((*snp).index + i) as isize),
                            );
                            if splitLitSeq == 0 {
                                output_bits(output, &mut out, 1_i32, 1_i32);
                            }
                        }
                    } else {
                        let latest_offset: u32 = (*(*snp).prev).latest_offset;
                        if latest_offset > 0_i32 as u32 {
                            output.log_dump_ln(format_args!(
                                "[{}] offset reuse bit = {}, latest = {}",
                                out.pos,
                                ((*mp).offset == latest_offset) as i32,
                                latest_offset,
                            ));
                        }
                        output.log_dump_ln(format_args!(
                            "[{}] sequence offset = {}, len = {}",
                            out.pos,
                            (*mp).offset,
                            (*mp).len as i32,
                        ));
                        optimal_encode(
                            output,
                            mp,
                            emd,
                            latest_offset,
                            core::ptr::null_mut::<encode_match_buckets>(),
                        );
                        output_bits(output, &mut out, 1_i32, 0_i32);
                        if (*mp).len as i32 == 1_i32 {
                            traits_used |= 1_i32 << 1_i32;
                        } else {
                            let lo: i32 = (*mp).len as i32 & 255_i32;
                            let hi: i32 = (*mp).len as i32 & !255_i32;
                            if hi > 0_i32 && lo < len0123skip {
                                traits_used |= 1_i32 << 2_i32;
                            }
                        }
                        if (*mp).offset > max_offset as u32 {
                            max_offset = (*mp).offset as i32;
                        }
                        if (*mp).len as i32 > max_len {
                            max_len = (*mp).len as i32;
                        }
                    }
                    pos_diff += (*mp).len as i32;
                    diff = (output_get_pos(&mut out)).wrapping_sub(pos_diff as u32) as i32;
                    if diff > max_diff {
                        max_diff = diff;
                    }
                }
                output.log_dump_ln(format_args!("------------"));
                snp = (*snp).prev;
            }
            output.log_dump_ln(format_args!("pos ${:04x}", out.pos));
            if (*options).output_header != 0 {
                optimal_out(output, &mut out, emd);
                output.log_dump_ln(format_args!("pos ${:04x}", out.pos));
            }
            if measure_alignment == 0 {
                break;
            }
            alignment = output_bits_alignment(output, &mut out);
            measure_alignment = 0_i32;
        }
        output_bits_flush(
            output,
            &mut out,
            ((*options).flags_proto & 1_i32 << 3_i32 == 0) as i32,
        );
        (*emd).out = old;
        if !infop.is_null() {
            (*infop).traits_used = traits_used.into();
            (*infop).max_len = max_len;
            (*infop).max_offset = max_offset;
            (*infop).needed_safety_offset = max_diff;
        }
    }
}
unsafe fn read_encoding_to_buf<O: Output>(
    output: &mut O,
    imported_enc: *const i8,
    flags_proto: i32,
    mut needs_reversing: i32,
    enc_buf: *mut buf,
) {
    unsafe {
        let mut emd: encode_match_data = encode_match_data {
            out: core::ptr::null_mut::<output_ctx>(),
            priv_0: core::ptr::null_mut::<c_void>(),
        };
        let options = CrunchOptions::default();
        let mut options = options.to_crunch_options();
        options.flags_proto = flags_proto;
        options.output_header = 1_i32;
        options.imported_encoding = imported_enc;
        emd.out = core::ptr::null_mut::<output_ctx>();
        optimal_init(output, &mut emd, options.flags_notrait, options.flags_proto);
        optimal_encoding_import(output, &mut emd, options.imported_encoding);
        do_output_backwards(
            output,
            core::ptr::null_mut::<match_ctx>(),
            core::ptr::null_mut::<search_node>(),
            &mut emd,
            &mut options,
            enc_buf,
            core::ptr::null_mut::<crunch_info>(),
        );
        optimal_free::<O>(&mut emd);
        needs_reversing = 1_i32;

        if needs_reversing != 0 {
            reverse_buffer(buf_data(enc_buf) as *mut i8, buf_size(enc_buf));
        }
    }
}
unsafe fn read_encoding_to_emd<O: Output>(
    output: &mut O,
    emd: *mut encode_match_data,
    options: *const crunch_options,
) {
    unsafe {
        let mut enc_buf: buf = {
            buf {
                data: core::ptr::null_mut::<c_void>(),
                size: 0_i32,
                capacity: 0_i32,
            }
        };
        let imported_enc: *const i8 = (*options).imported_encoding;
        output.log_normal_ln(format_args!(" Using imported encoding",));
        output.log_normal_ln(format_args!(
            " Enc: {:?}",
            CStr::from_ptr(imported_enc).to_str().unwrap()
        ));

        optimal_encoding_import(output, emd, imported_enc);
        buf_free(&mut enc_buf);
    }
}
unsafe fn do_compress_backwards<O: Output>(
    output: &mut O,
    ctxp: *mut match_ctx,
    ctx_count: i32,
    emd: *mut encode_match_data,
    options: *const crunch_options,
    enc: &mut String,
) -> *mut *mut search_node {
    unsafe {
        let mut snpev: vec = {
            vec {
                elsize: ::core::mem::size_of::<match_snp_enum>(),
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
        let mut mpcce: match_concat_enum<O> = match_concat_enum {
            enum_iterator: vec_iterator {
                vec: core::ptr::null::<vec>(),
                pos: 0,
            },
            next_f: None,
            enum_current: core::ptr::null_mut::<c_void>(),
        };
        let mut snpp: *mut *mut search_node = core::ptr::null_mut::<*mut search_node>();
        let mut pass: i32 = 0;
        let mut i: i32 = 0;
        let mut last_waltz: i32 = 0_i32;
        let mut size: f32 = 0.;
        let mut old_size: f32 = 0.;
        let mut prev_enc = String::with_capacity(100);
        snpp = calloc(
            ctx_count as usize,
            ::core::mem::size_of::<*mut search_node>(),
        ) as *mut *mut search_node;
        pass = 1_i32;
        output.log_normal_ln(format_args!(" pass {}:", pass));
        if !((*options).imported_encoding).is_null() {
            read_encoding_to_emd(output, emd, options);
            if (*options).max_passes == 1_i32 {
                pass += 1;
                pass;
            }
        } else {
            let mut mpcev: vec = {
                vec {
                    elsize: ::core::mem::size_of::<match_cache_enum>(),
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
            output.log_normal_ln(format_args!("optimizing .."));
            i = 0_i32;
            while i < ctx_count {
                let mp_enum: *mut match_cache_enum =
                    vec_push(&mut mpcev, core::ptr::null::<c_void>()) as *mut match_cache_enum;
                match_cache_get_enum(ctxp.offset(i as isize), mp_enum);
                i += 1;
                i;
            }
            match_concat_get_enum(
                Some(match_cache_enum_get_next as unsafe fn(&mut O, *mut c_void) -> *const match_0),
                &mut mpcev,
                &mut mpcce,
            );
            optimal_optimize(
                output,
                emd,
                Some(
                    match_concat_enum_get_next as unsafe fn(&mut O, *mut c_void) -> *const match_0,
                ),
                &mut mpcce as *mut match_concat_enum<O> as *mut c_void,
            );
            vec_free(&mut mpcev);
        }
        optimal_encoding_export::<O>(emd, enc);
        prev_enc.clone_from(enc);
        old_size = 100000000.0f64 as f32;
        loop {
            size = 0.0f64 as f32;
            i = 0_i32;
            while i < ctx_count {
                if !(*snpp.offset(i as isize)).is_null() {
                    free(*snpp.offset(i as isize) as *mut c_void);
                    let fresh0 = &mut (*snpp.offset(i as isize));
                    *fresh0 = core::ptr::null_mut::<search_node>();
                }
                search_buffer(
                    output,
                    ctxp.offset(i as isize),
                    emd,
                    (*options).flags_proto,
                    (*options).flags_notrait,
                    (*options).max_len,
                    (pass & 1_i32 == 0) as i32,
                    &mut *snpp.offset(i as isize),
                );
                if (*snpp.offset(i as isize)).is_null() {
                    panic!("error: search_buffer() returned NULL",);
                }
                size += (**snpp.offset(i as isize)).total_score;
                i += 1;
                i;
            }
            output.log_normal_ln(format_args!(
                "  size {:.1} bits ~{} bytes",
                size as f64,
                (size as i32 + 7_i32) >> 3_i32,
            ));
            if last_waltz != 0 {
                break;
            }
            pass += 1;
            pass;
            if size >= old_size {
                last_waltz = 1_i32;
            } else {
                old_size = size;
                if pass > (*options).max_passes {
                    break;
                }
                optimal_free::<O>(emd);
                optimal_init(
                    output,
                    emd,
                    (*options).flags_notrait,
                    (*options).flags_proto,
                );
                output.log_normal_ln(format_args!(" pass {}: optimizing ..", pass,));
                i = 0_i32;
                while i < ctx_count {
                    let mp_enum_0: *mut match_snp_enum =
                        vec_push(&mut snpev, core::ptr::null::<c_void>()) as *mut match_snp_enum;
                    match_snp_get_enum(*snpp.offset(i as isize), mp_enum_0);
                    i += 1;
                    i;
                }
                match_concat_get_enum(
                    Some(
                        match_snp_enum_get_next as unsafe fn(&mut O, *mut c_void) -> *const match_0,
                    ),
                    &mut snpev,
                    &mut mpcce,
                );
                optimal_optimize(
                    output,
                    emd,
                    Some(
                        match_concat_enum_get_next
                            as unsafe fn(&mut O, *mut c_void) -> *const match_0,
                    ),
                    &mut mpcce as *mut match_concat_enum<O> as *mut c_void,
                );
                vec_clear(&mut snpev);
                optimal_encoding_export::<O>(emd, enc);
                if enc == &prev_enc {
                    break;
                }
                prev_enc.clone_from(enc);
            }
        }
        vec_free(&mut snpev);
        snpp
    }
}
pub(crate) unsafe fn crunch_multi<O: Output>(
    output: &mut O,
    io_bufs: *mut vec,
    noread_in: *mut buf,
    enc_buf: *mut buf,
    options: *const crunch_options,
    infop: *mut crunch_info,
) -> String {
    unsafe {
        let mut ctxp: *mut match_ctx = core::ptr::null_mut::<match_ctx>();
        let mut emd: encode_match_data = encode_match_data {
            out: core::ptr::null_mut::<output_ctx>(),
            priv_0: core::ptr::null_mut::<c_void>(),
        };
        let mut snpp: *mut *mut search_node = core::ptr::null_mut::<*mut search_node>();
        let mut merged_info: crunch_info = {
            crunch_info {
                traits_used: 0_i32,
                max_len: 0_i32,
                max_offset: 0_i32,
                needed_safety_offset: 0,
            }
        };
        let mut exported_enc = String::with_capacity(72);
        let buf_count: i32 = vec_size(io_bufs);
        let mut outlen: i32 = 0_i32;
        let mut inlen: i32 = 0_i32;
        let mut i: i32 = 0;
        let mut outpos: *mut i32 = core::ptr::null_mut::<i32>();
        outpos = malloc(::core::mem::size_of::<i32>().wrapping_mul(buf_count as usize)) as *mut i32;
        ctxp = malloc(::core::mem::size_of::<match_ctx>().wrapping_mul(buf_count as usize))
            as *mut match_ctx;
        output.log_normal_ln(format_args!(""));
        output.log_normal_ln(format_args!(
            "Phase 1: Preprocessing file{}",
            if buf_count == 1_i32 { "" } else { "s" },
        ));
        output.log_normal_ln(format_args!(
            "---------------------------{}",
            if buf_count == 1_i32 { "" } else { "-" },
        ));
        i = 0_i32;
        while i < buf_count {
            let mut nor_view: buf = buf {
                data: core::ptr::null_mut::<c_void>(),
                size: 0,
                capacity: 0,
            };
            let io: *mut io_bufs = vec_get(io_bufs, i) as *mut io_bufs;
            let in_0: *mut buf = &mut (*io).in_0;
            let mut nor: *const buf = core::ptr::null::<buf>();
            if !noread_in.is_null() {
                let growth_req: i32 = buf_size(in_0) + (*io).in_off - buf_size(noread_in);
                if growth_req > 0_i32 {
                    memset(
                        buf_append(noread_in, core::ptr::null::<c_void>(), growth_req),
                        0_i32,
                        growth_req as usize,
                    );
                }
                nor = buf_view(&mut nor_view, noread_in, (*io).in_off, buf_size(in_0));
            }
            if (*options).direction_forward == 1_i32 {
                let out: *mut buf = &mut (*io).out;
                reverse_buffer(buf_data(in_0) as *mut i8, buf_size(in_0));
                *outpos.offset(i as isize) = buf_size(out);
            }
            inlen += buf_size(in_0);
            match_ctx_init(
                output,
                ctxp.offset(i as isize),
                in_0,
                nor,
                (*options).max_len,
                (*options).max_offset,
                (*options).favor_speed,
            );
            i += 1;
            i;
        }
        output.log_normal_ln(format_args!(" Length of indata: {} bytes.", inlen,));
        output.log_normal_ln(format_args!(
            " Preprocessing file{}, done.",
            if buf_count == 1_i32 { "" } else { "s" },
        ));
        emd.out = core::ptr::null_mut::<output_ctx>();
        optimal_init(
            output,
            &mut emd,
            (*options).flags_notrait,
            (*options).flags_proto,
        );
        output.log_normal_ln(format_args!("",));
        output.log_normal_ln(format_args!("Phase 2: Calculating encoding",));
        output.log_normal_ln(format_args!("-----------------------------",));
        snpp = do_compress_backwards(
            output,
            ctxp,
            buf_count,
            &mut emd,
            options,
            &mut exported_enc,
        );
        output.log_normal_ln(format_args!(" Calculating encoding, done."));
        output.log_normal_ln(format_args!("",));
        output.log_normal_ln(format_args!(
            "Phase 3: Generating output file{}",
            if buf_count == 1_i32 { "" } else { "s" },
        ));
        output.log_normal_ln(format_args!(
            "-------------------------------{}",
            if buf_count == 1_i32 { "" } else { "-" },
        ));
        output.log_normal_ln(format_args!(" Enc: {}", exported_enc));
        if !enc_buf.is_null() {
            let mut enc_opts: crunch_options = (*options).clone();
            enc_opts.output_header = 1_i32;
            do_output_backwards(
                output,
                core::ptr::null_mut::<match_ctx>(),
                core::ptr::null_mut::<search_node>(),
                &mut emd,
                &mut enc_opts,
                enc_buf,
                core::ptr::null_mut::<crunch_info>(),
            );
            if (*options).direction_forward == 1_i32 {
                reverse_buffer(buf_data(enc_buf) as *mut i8, buf_size(enc_buf));
            }
        }
        i = 0_i32;
        while i < buf_count {
            let io_0: *mut io_bufs = vec_get(io_bufs, i) as *mut io_bufs;
            let in_1: *const buf = &mut (*io_0).in_0;
            let out_0: *mut buf = &mut (*io_0).out;
            let info: *mut crunch_info = &mut (*io_0).info;
            outlen -= buf_size(out_0);
            do_output_backwards(
                output,
                ctxp.offset(i as isize),
                *snpp.offset(i as isize),
                &mut emd,
                options,
                out_0,
                info,
            );
            outlen += buf_size(out_0);
            if (*options).direction_forward == 1_i32 {
                reverse_buffer(buf_data(in_1) as *mut i8, buf_size(in_1));
                reverse_buffer(
                    (buf_data(out_0) as *mut i8).offset(*outpos.offset(i as isize) as isize),
                    buf_size(out_0) - *outpos.offset(i as isize),
                );
            }
            merged_info.traits_used |= (*info).traits_used;
            if merged_info.max_len < (*info).max_len {
                merged_info.max_len = (*info).max_len;
            }
            if merged_info.max_offset < (*info).max_offset {
                merged_info.max_offset = (*info).max_offset;
            }
            if merged_info.needed_safety_offset < (*info).needed_safety_offset {
                merged_info.needed_safety_offset = (*info).needed_safety_offset;
            }
            i += 1;
            i;
        }
        output.log_normal_ln(format_args!(" Length of crunched data: {} bytes.", outlen,));
        if inlen - outlen >= 0_i32 {
            output.log_brief_ln(format_args!(
                " Crunched data reduced {} bytes ({:.2}%).",
                inlen - outlen,
                100.0f64 * (inlen - outlen) as f64 / inlen as f64,
            ));
        } else if inlen == 0_i32 {
            output.log_brief_ln(format_args!(
                " Crunched data enlarged {} bytes",
                outlen - inlen,
            ));
        } else {
            output.log_brief_ln(format_args!(
                " Crunched data enlarged {} bytes ({:.2}%)",
                outlen - inlen,
                100.0f64 * (outlen - inlen) as f64 / inlen as f64,
            ));
        }
        optimal_free::<O>(&mut emd);
        i = 0_i32;
        while i < buf_count {
            free(*snpp.offset(i as isize) as *mut c_void);
            match_ctx_free(ctxp.offset(i as isize));
            i += 1;
            i;
        }
        free(snpp as *mut c_void);
        free(ctxp as *mut c_void);
        free(outpos as *mut c_void);
        if !infop.is_null() {
            *infop = merged_info;
        }
        exported_enc
    }
}
pub(crate) unsafe fn reverse_buffer(buf: *mut i8, len: i32) {
    unsafe {
        let mut start: i32 = 0_i32;
        let mut end: i32 = len - 1_i32;
        let mut tmp: i8 = 0;
        while start < end {
            tmp = *buf.offset(start as isize);
            *buf.offset(start as isize) = *buf.offset(end as isize);
            *buf.offset(end as isize) = tmp;
            start += 1;
            start;
            end -= 1;
            end;
        }
    }
}
pub(crate) unsafe fn decrunch<O: Output>(
    output: &mut O,
    inbuf: *mut buf,
    in_off: i32,
    outbuf: *mut buf,
    dopts: *mut decrunch_options,
) {
    unsafe {
        let mut ctx: dec_ctx = dec_ctx {
            inpos: 0,
            inend: 0,
            inbuf: core::ptr::null_mut::<u8>(),
            outbuf: core::ptr::null_mut::<buf>(),
            bitbuf: 0,
            t: dec_table {
                table_bit: [0; 8],
                table_off: [0; 8],
                table_bi: [0; 100],
                table_lo: [0; 100],
                table_hi: [0; 100],
            },
            bits_read: 0,
            flags_proto: 0,
        };
        let mut enc_buf: buf = {
            buf {
                data: core::ptr::null_mut::<c_void>(),
                size: 0_i32,
                capacity: 0_i32,
            }
        };
        let mut encp: *mut buf = core::ptr::null_mut::<buf>();
        let mut outpos: i32 = 0;
        if (*dopts).direction_forward == 0_i32 {
            reverse_buffer(buf_data(inbuf) as *mut i8, buf_size(inbuf));
        }
        outpos = buf_size(outbuf);
        encp = core::ptr::null_mut::<buf>();
        if !((*dopts).imported_encoding).is_null() {
            read_encoding_to_buf(
                output,
                (*dopts).imported_encoding,
                (*dopts).flags_proto,
                ((*dopts).direction_forward == 0) as i32 ^ ((*dopts).write_reverse != 0_i32) as i32,
                &mut enc_buf,
            );
            encp = &mut enc_buf;
        }
        dec_ctx_init(&mut ctx, encp, inbuf, outbuf, (*dopts).flags_proto);
        buf_clear(&mut enc_buf);
        dec_ctx_table_dump(&mut ctx, &mut enc_buf);
        output.log_normal_ln(format_args!(
            " Enc: {}",
            CStr::from_ptr(buf_data(&mut enc_buf) as *const _)
                .to_str()
                .unwrap(),
        ));
        buf_free(&mut enc_buf);
        dec_ctx_decrunch(output, &mut ctx);
        dec_ctx_free(&mut ctx);
        if (*dopts).direction_forward == 0_i32 {
            reverse_buffer(buf_data(inbuf) as *mut i8, buf_size(inbuf));
            reverse_buffer(
                (buf_data(outbuf) as *mut i8).offset(outpos as isize),
                buf_size(outbuf) - outpos,
            );
        }
    }
}
