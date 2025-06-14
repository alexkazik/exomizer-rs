/*
 * Copyright (c) 2003 - 2005 Magnus Lind.
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
use crate::original::replacement::*;

pub(crate) struct vec {
    pub(crate) elsize: usize,
    pub(crate) buf: buf,
    pub(crate) flags: i32,
}
pub(crate) struct vec_iterator {
    pub(crate) vec: *const vec,
    pub(crate) pos: i32,
}
pub(crate) unsafe fn vec_init(p: *mut vec, elsize: usize) {
    unsafe {
        (*p).elsize = elsize;
        buf_init(&mut (*p).buf);
        (*p).flags = 1_i32;
    }
}
pub(crate) unsafe fn vec_clear(p: *mut vec) {
    unsafe {
        let mut i: vec_iterator = vec_iterator {
            vec: core::ptr::null::<vec>(),
            pos: 0,
        };
        let d: *const c_void = core::ptr::null::<c_void>();
        vec_get_iterator(p, &mut i);
        buf_clear(&mut (*p).buf);
        (*p).flags = 1_i32;
    }
}
pub(crate) unsafe fn vec_free(p: *mut vec) {
    unsafe {
        vec_clear(p);
        buf_free(&mut (*p).buf);
    }
}
pub(crate) unsafe fn vec_size(p: *const vec) -> i32 {
    unsafe {
        let mut size: i32 = 0;
        size = (buf_size(&(*p).buf) as usize / (*p).elsize) as i32;
        size
    }
}
pub(crate) unsafe fn vec_get(p: *const vec, index: i32) -> *mut c_void {
    unsafe {
        let mut buf: *mut i8 = core::ptr::null_mut::<i8>();
        if index >= 0_i32 && index < vec_size(p) {
            buf = buf_data(&(*p).buf) as *mut i8;
            buf = buf.add(index as usize * (*p).elsize);
        }
        buf as *mut c_void
    }
}
pub(crate) unsafe fn vec_push(p: *mut vec, in_0: *const c_void) -> *mut c_void {
    unsafe {
        let mut out: *mut c_void = core::ptr::null_mut::<c_void>();
        out = buf_append(&mut (*p).buf, in_0, (*p).elsize as i32);
        (*p).flags &= !1_i32;
        out
    }
}
pub(crate) unsafe fn vec_get_iterator(p: *const vec, i: *mut vec_iterator) {
    unsafe {
        (*i).vec = p;
        (*i).pos = 0_i32;
    }
}
pub(crate) unsafe fn vec_iterator_next(i: *mut vec_iterator) -> *mut c_void {
    unsafe {
        let mut out: *mut c_void = core::ptr::null_mut::<c_void>();
        let size: i32 = vec_size((*i).vec);
        if (*i).pos >= size {
            (*i).pos = 0_i32;
            return core::ptr::null_mut::<c_void>();
        }
        out = vec_get((*i).vec, (*i).pos);
        (*i).pos += 1_i32;
        out
    }
}
