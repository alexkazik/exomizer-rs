/*
 * Copyright (c) 2002 2005 Magnus Lind.
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

use crate::original::replacement::*;

pub(crate) struct buf {
    pub(crate) data: *mut c_void,
    pub(crate) size: i32,
    pub(crate) capacity: i32,
}
pub(crate) unsafe fn buf_init(b: *mut buf) {
    unsafe {
        (*b).data = core::ptr::null_mut::<c_void>();
        (*b).size = 0_i32;
        (*b).capacity = 0_i32;
    }
}
pub(crate) unsafe fn buf_free(b: *mut buf) {
    unsafe {
        if (*b).capacity == -1_i32 {
            panic!("error, can't free a buf view");
        }
        if !((*b).data).is_null() {
            free((*b).data);
            (*b).data = core::ptr::null_mut::<c_void>();
        }
        (*b).size = 0_i32;
        (*b).capacity = 0_i32;
    }
}
pub(crate) unsafe fn buf_size(b: *const buf) -> i32 {
    unsafe { (*b).size }
}
pub(crate) unsafe fn buf_data(b: *const buf) -> *mut c_void {
    unsafe { (*b).data }
}
pub(crate) unsafe fn buf_reserve(b: *mut buf, new_capacity: i32) -> *mut c_void {
    unsafe {
        let mut capacity: i32 = (*b).capacity;
        if capacity == -1_i32 {
            panic!("error, can't reserve capacity for a buf view");
        }
        if capacity == 0_i32 {
            capacity = 1_i32;
        }
        while capacity < new_capacity {
            capacity <<= 1_i32;
        }
        if capacity > (*b).capacity {
            (*b).data = realloc((*b).data, capacity as usize);
            if ((*b).data).is_null() {
                panic!("error, can't reallocate memory");
            }
            (*b).capacity = capacity;
        }
        (*b).data
    }
}
pub(crate) unsafe fn buf_clear(b: *mut buf) {
    unsafe {
        buf_replace(b, 0_i32, (*b).size, core::ptr::null::<c_void>(), 0_i32);
    }
}
pub(crate) unsafe fn buf_remove(b: *mut buf, b_off: i32, b_n: i32) {
    unsafe {
        buf_replace(b, b_off, b_n, core::ptr::null::<c_void>(), 0_i32);
    }
}
pub(crate) unsafe fn buf_append(b: *mut buf, m: *const c_void, m_n: i32) -> *mut c_void {
    unsafe { buf_replace(b, (*b).size, 0_i32, m, m_n) }
}
pub(crate) unsafe fn buf_append_char(b: *mut buf, mut c: i8) {
    unsafe {
        buf_replace(
            b,
            (*b).size,
            0_i32,
            &mut c as *mut i8 as *const c_void,
            1_i32,
        );
    }
}
pub(crate) unsafe fn buf_replace(
    b: *mut buf,
    mut b_off: i32,
    mut b_n: i32,
    m: *const c_void,
    m_n: i32,
) -> *mut c_void {
    unsafe {
        let mut new_size: i32 = 0;
        let mut rest_off: i32 = 0;
        let mut rest_n: i32 = 0;
        if (*b).capacity == -1_i32 {
            panic!("error, can't modify a buf view");
        }
        if b_off < 0_i32 {
            b_off += (*b).size + 1_i32;
        }
        if b_n == -1_i32 {
            b_n = (*b).size - b_off;
        }
        if b_off < 0_i32 || b_off > (*b).size {
            panic!("error, b_off {} must be within [0 - {}].", b_off, (*b).size,);
        }
        if b_n < 0_i32 || b_n > (*b).size - b_off {
            panic!(
                "error, b_n {} must be within [0 and {}] for b_off {}.",
                b_n,
                (*b).size - b_off,
                b_off,
            );
        }
        if m_n < 0_i32 {
            panic!("error, m_n {} must be >= 0.", m_n,);
        }
        new_size = (*b).size - b_n + m_n;
        if new_size > (*b).capacity {
            buf_reserve(b, new_size);
        }
        rest_off = b_off + b_n;
        rest_n = (*b).size - rest_off;
        if rest_n > 0_i32 {
            memmove(
                ((*b).data as *mut i8)
                    .offset(b_off as isize)
                    .offset(m_n as isize) as *mut c_void,
                ((*b).data as *mut i8).offset(rest_off as isize) as *const c_void,
                rest_n as usize,
            );
        }
        if m_n > 0_i32 && !m.is_null() {
            memcpy(
                ((*b).data as *mut i8).offset(b_off as isize) as *mut c_void,
                m,
                m_n as usize,
            );
        }
        (*b).size = new_size;
        ((*b).data as *mut i8).offset(b_off as isize) as *mut c_void
    }
}
pub(crate) unsafe fn buf_view(
    v: *mut buf,
    b: *const buf,
    mut b_off: i32,
    mut b_n: i32,
) -> *const buf {
    unsafe {
        if b_off < 0_i32 {
            b_off += (*b).size + 1_i32;
        }
        if b_n == -1_i32 {
            b_n = (*b).size - b_off;
        }
        if b_off < 0_i32 || b_off > (*b).size {
            panic!("error, b_off {} must be within [0 - {}].", b_off, (*b).size,);
        }
        if b_n < 0_i32 || b_n > (*b).size - b_off {
            panic!(
                "error, b_n {} must be within [0 - {}].",
                b_n,
                (*b).size - b_off,
            );
        }
        if !((*b).data).is_null() {
            (*v).data = ((*b).data as *mut i8).offset(b_off as isize) as *mut c_void;
        } else {
            (*v).data = core::ptr::null_mut::<c_void>();
        }
        (*v).size = b_n;
        (*v).capacity = -1_i32;
        v
    }
}
