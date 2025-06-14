/*
 * Copyright (c) 2002, 2003 Magnus Lind.
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

use crate::original::converted::chunkpool::*;
use crate::original::output::Output;
use crate::original::replacement::*;

pub(crate) struct radix_root {
    pub(crate) depth: i32,
    pub(crate) root: *mut radix_node,
    pub(crate) mem: chunkpool,
}
pub(crate) struct radix_node {
    pub(crate) rn: *mut radix_node,
}
pub(crate) type free_callback = unsafe fn(*mut c_void, *mut c_void) -> ();
pub(crate) unsafe fn radix_tree_init(rr: *mut radix_root) {
    unsafe {
        (*rr).depth = 0_i32;
        (*rr).root = core::ptr::null_mut::<radix_node>();
        chunkpool_init(
            &mut (*rr).mem,
            ((1_i32 << 8_u32) as usize).wrapping_mul(::core::mem::size_of::<*mut c_void>()) as i32,
        );
    }
}
unsafe fn radix_tree_free_helper(
    depth: i32,
    rnp: *mut radix_node,
    f: Option<free_callback>,
    priv_0: *mut c_void,
) {
    unsafe {
        let mut i: i32 = 0;
        if depth == 0_i32 {
            if f.is_some() {
                f.expect("non-null function pointer")(rnp as *mut c_void, priv_0);
            }
        } else if !rnp.is_null() {
            i = (1_u32 << 8_u32).wrapping_sub(1_u32) as i32;
            while i >= 0_i32 {
                radix_tree_free_helper(depth - 1_i32, (*rnp.offset(i as isize)).rn, f, priv_0);
                let fresh0 = &mut (*rnp.offset(i as isize)).rn;
                *fresh0 = core::ptr::null_mut::<radix_node>();
                i -= 1;
                i;
            }
        }
    }
}
pub(crate) unsafe fn radix_tree_free(
    rr: *mut radix_root,
    f: Option<free_callback>,
    priv_0: *mut c_void,
) {
    unsafe {
        radix_tree_free_helper((*rr).depth, (*rr).root, f, priv_0);
        (*rr).depth = 0_i32;
        (*rr).root = core::ptr::null_mut::<radix_node>();
        chunkpool_free(&mut (*rr).mem);
    }
}
pub(crate) unsafe fn radix_node_set<O: Output>(
    output: &mut O,
    rrp: *mut radix_root,
    index: u32,
    data: *mut c_void,
) {
    unsafe {
        let mut rnp: *mut radix_node = core::ptr::null_mut::<radix_node>();
        let mut rnpp: *mut *mut radix_node = core::ptr::null_mut::<*mut radix_node>();
        let mut mask: u32 = 0;
        let mut depth: i32 = 0;
        mask = !0_u32 << 8_u32.wrapping_mul((*rrp).depth as u32);
        while index & mask != 0 {
            rnp = chunkpool_calloc(output, &mut (*rrp).mem) as *mut radix_node;
            let fresh1 = &mut (*rnp.offset(0_i32 as isize)).rn;
            *fresh1 = (*rrp).root;
            (*rrp).root = rnp;
            (*rrp).depth += 1_i32;
            mask = !0_u32 << 8_u32.wrapping_mul((*rrp).depth as u32);
        }
        rnpp = &mut (*rrp).root;
        depth = (*rrp).depth - 1_i32;
        while depth >= 0_i32 {
            let mut node_index: u32 = 0;
            if (*rnpp).is_null() {
                *rnpp = chunkpool_calloc(output, &mut (*rrp).mem) as *mut radix_node;
            }
            node_index =
                index >> 8_u32.wrapping_mul(depth as u32) & (1_u32 << 8_u32).wrapping_sub(1_u32);
            rnpp = &mut (*(*rnpp).offset(node_index as isize)).rn;
            depth -= 1;
            depth;
        }
        *rnpp = data as *mut radix_node;
    }
}
pub(crate) unsafe fn radix_node_get(rr: *mut radix_root, index: u32) -> *mut c_void {
    unsafe {
        let mut rnp: *mut radix_node = core::ptr::null_mut::<radix_node>();
        let mut depth: u16 = 0;
        rnp = (*rr).root;
        depth = ((*rr).depth - 1_i32) as u16;
        while (depth as i32) < 0xffff_i32 {
            let mut node_index: u16 = 0;
            if rnp.is_null() {
                break;
            }
            node_index = (index >> 8_u32.wrapping_mul(depth as u32)
                & (1_u32 << 8_u32).wrapping_sub(1_u32)) as u16;
            rnp = (*rnp.offset(node_index as isize)).rn;
            depth = depth.wrapping_sub(1);
            depth;
        }
        rnp as *mut c_void
    }
}
