use alloc::alloc::Layout;
use core::cmp::Ordering;
pub(super) use core::ffi::c_void;
use core::slice::{from_raw_parts, from_raw_parts_mut};

pub(super) unsafe fn memset(p: *mut c_void, v: i32, l: usize) -> *mut c_void {
    unsafe {
        let s = from_raw_parts_mut(p.cast::<u8>(), l);
        s.fill(v as u8);
        p
    }
}

pub(super) unsafe fn memcpy(p1: *mut c_void, p2: *const c_void, l: usize) -> *mut c_void {
    unsafe {
        let s1 = from_raw_parts_mut(p1.cast::<u8>(), l);
        let s2 = from_raw_parts(p2.cast::<u8>(), l);
        s1.copy_from_slice(s2);
        p1
    }
}

pub(super) unsafe fn memmove(dst: *mut c_void, src: *const c_void, l: usize) -> *mut c_void {
    unsafe {
        let ofs = src.byte_offset_from(dst);

        match ofs.cmp(&0) {
            Ordering::Less => {
                // dst is after src
                let ofs = (-ofs) as usize;
                from_raw_parts_mut(src as *mut u8, l).copy_within(..l, ofs);
            }
            Ordering::Equal => {
                // nothing to move
            }
            Ordering::Greater => {
                // src is after dst
                let ofs = ofs as usize;
                from_raw_parts_mut(dst.cast::<u8>(), l).copy_within(ofs..ofs + l, 0);
            }
        }

        dst
    }
}

pub(super) unsafe fn strlen(p: *const i8) -> usize {
    let mut len = 0;

    // SAFETY: Outer caller has provided a pointer to a valid C string.
    while unsafe { *p.add(len) } != 0 {
        len += 1;
    }

    len
}

#[allow(clippy::cast_ptr_alignment)]
pub(super) unsafe fn calloc(la: usize, lb: usize) -> *mut c_void {
    unsafe {
        let l = (la * lb) + 16;
        let a = alloc::alloc::alloc_zeroed(Layout::from_size_align_unchecked(l, 16));
        *a.cast::<usize>() = l;
        a.add(16).cast()
    }
}

#[allow(clippy::cast_ptr_alignment)]
pub(super) unsafe fn free(p: *mut c_void) {
    unsafe {
        if !p.is_null() {
            let p = p.cast::<u8>();
            let p = p.sub(16);
            alloc::alloc::dealloc(
                p,
                Layout::from_size_align_unchecked(*(p as *const usize), 16),
            );
        }
    }
}

#[allow(clippy::cast_ptr_alignment)]
pub(super) unsafe fn malloc(l: usize) -> *mut c_void {
    unsafe {
        let l = l + 16;
        let a = alloc::alloc::alloc(Layout::from_size_align_unchecked(l, 16));
        *a.cast::<usize>() = l;
        a.add(16).cast()
    }
}

#[allow(clippy::cast_ptr_alignment)]
pub(super) unsafe fn realloc(p: *mut c_void, l: usize) -> *mut c_void {
    unsafe {
        if p.is_null() {
            malloc(l)
        } else {
            let p = p.cast::<u8>();
            let p = p.sub(16);
            let l = l + 16;
            let old_l = *(p as *const usize);
            let a = alloc::alloc::realloc(p, Layout::from_size_align_unchecked(old_l, 16), l);
            memset(a.add(old_l).cast(), 0, l - old_l);
            *a.cast::<usize>() = l;
            a.add(16).cast()
        }
    }
}
