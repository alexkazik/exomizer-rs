/*
 * Copyright (c) 2003 - 2005, 2015 Magnus Lind.
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

use crate::original::converted::vec::*;
use crate::original::output::Output;
use crate::original::replacement::*;

pub(crate) struct chunkpool {
    pub(crate) item_size: i32,
    pub(crate) item_pos: i32,
    pub(crate) item_end: i32,
    pub(crate) current_chunk: *mut c_void,
    pub(crate) used_chunks: vec,
    pub(crate) alloc_count: i32,
}
pub(crate) unsafe fn chunkpool_init(ctx: *mut chunkpool, item_size: i32) {
    unsafe {
        (*ctx).item_size = item_size;
        (*ctx).item_end = 0x1fffff_i32 / item_size * item_size;
        (*ctx).item_pos = (*ctx).item_end;
        (*ctx).current_chunk = core::ptr::null_mut::<c_void>();
        vec_init(
            &mut (*ctx).used_chunks,
            ::core::mem::size_of::<*mut c_void>(),
        );
        (*ctx).alloc_count = 0_i32;
    }
}
unsafe fn chunk_free(chunks: *mut c_void, item_pos: i32, item_size: i32) {}
pub(crate) unsafe fn chunkpool_free2(ctx: *mut chunkpool) {
    unsafe {
        let mut chunkp: *mut *mut c_void = core::ptr::null_mut::<*mut c_void>();
        let mut i: vec_iterator = vec_iterator {
            vec: core::ptr::null::<vec>(),
            pos: 0,
        };
        if !((*ctx).current_chunk).is_null() {
            chunk_free((*ctx).current_chunk, (*ctx).item_pos, (*ctx).item_size);
            free((*ctx).current_chunk);
        }
        vec_get_iterator(&mut (*ctx).used_chunks, &mut i);
        loop {
            chunkp = vec_iterator_next(&mut i) as *mut *mut c_void;
            if chunkp.is_null() {
                break;
            }
            chunk_free(*chunkp, (*ctx).item_end, (*ctx).item_size);
            free(*chunkp);
        }
        (*ctx).item_size = -1_i32;
        (*ctx).item_end = -1_i32;
        (*ctx).item_pos = -1_i32;
        (*ctx).current_chunk = core::ptr::null_mut::<c_void>();
        vec_free(&mut (*ctx).used_chunks);
    }
}
pub(crate) unsafe fn chunkpool_free(ctx: *mut chunkpool) {
    unsafe {
        chunkpool_free2(ctx);
    }
}
pub(crate) unsafe fn chunkpool_malloc<O: Output>(
    output: &mut O,
    ctx: *mut chunkpool,
) -> *mut c_void {
    unsafe {
        let mut p: *mut c_void = core::ptr::null_mut::<c_void>();
        if (*ctx).item_pos == (*ctx).item_end {
            let mut m: *mut c_void = core::ptr::null_mut::<c_void>();
            m = malloc((*ctx).item_end as usize);
            output.log_debug_ln(format_args!("allocating new chunk {:?}", m));
            if m.is_null() {
                panic!(
                    "alloced {} items of size {}",
                    (*ctx).alloc_count,
                    (*ctx).item_size,
                );
            }
            vec_push(
                &mut (*ctx).used_chunks,
                &mut (*ctx).current_chunk as *mut *mut c_void as *const c_void,
            );
            (*ctx).current_chunk = m;
            (*ctx).item_pos = 0_i32;
        }
        p = ((*ctx).current_chunk as *mut i8).offset((*ctx).item_pos as isize) as *mut c_void;
        (*ctx).item_pos += (*ctx).item_size;
        (*ctx).alloc_count += 1;
        (*ctx).alloc_count;
        p
    }
}
pub(crate) unsafe fn chunkpool_calloc<O: Output>(
    output: &mut O,
    ctx: *mut chunkpool,
) -> *mut c_void {
    unsafe {
        let p: *mut c_void = chunkpool_malloc(output, ctx);
        memset(p, 0_i32, (*ctx).item_size as usize);
        p
    }
}
