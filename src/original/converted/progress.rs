/*
 * Copyright (c) 2005 Magnus Lind.
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

use crate::original::output::Output;
use crate::original::replacement::*;
use alloc::string::String;
use core::iter::repeat_n;

pub(crate) struct progress {
    pub(crate) msg: *mut i8,
    pub(crate) factor: f32,
    pub(crate) offset: i32,
    pub(crate) last: i32,
}

#[inline]
fn progress_bar_length<O: Output>(output: &mut O) -> usize {
    output.progress_bar_length().max(2)
}

pub(crate) unsafe fn progress_init<O: Output>(
    output: &mut O,
    p: *mut progress,
    mut msg: *mut i8,
    start: i32,
    end: i32,
) {
    unsafe {
        let progress_bar_length = progress_bar_length(output) as f32;
        if start > end {
            (*p).factor = progress_bar_length / (end - start) as f32;
            (*p).offset = -start;
        } else {
            (*p).factor = progress_bar_length / (start - end) as f32;
            (*p).offset = start;
        }
        (*p).last = -1_i32;
        if msg.is_null() {
            msg = b"progressing_\0" as *const u8 as *const i8 as *mut i8;
        }
        (*p).msg = msg;
        output.log_debug_ln(format_args!(
            "start {}, end {}, pfactor {}, poffset {}",
            start,
            end,
            (*p).factor as f64,
            (*p).offset,
        ));
    }
}
pub(crate) unsafe fn progress_bump<O: Output>(output: &mut O, p: *mut progress, pos: i32) {
    unsafe {
        let fraction: i32 = (((pos + (*p).offset) as f32 * (*p).factor) as f64 + 0.5f64) as i32;
        while fraction > (*p).last {
            if (*p).last == -1_i32 {
                let mut ini = String::with_capacity(progress_bar_length(output) + 6);
                ini.push_str("  ");
                ini.extend(repeat_n(' ', progress_bar_length(output)));
                ini.push_str("]\r [");

                output.progress_init(&ini);
            } else {
                output.progress_bump(
                    *((*p).msg).add(((*p).last as usize).wrapping_rem(strlen((*p).msg))) as u8
                        as char,
                );
            }
            (*p).last += 1_i32;
        }
    }
}
pub(crate) unsafe fn progress_free<O: Output>(output: &mut O, p: *mut progress) {
    output.progress_end();
}
