// MIT License
//
// Copyright (c) 2018 Redox OS
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use core::{mem, ptr, slice, usize};
use cty::*;

#[no_mangle]
pub unsafe extern "C" fn strchr(mut s: *const c_char, c: c_int) -> *mut c_char {
    let c = c as c_char;
    while *s != 0 {
        if *s == c {
            return s as *mut c_char;
        }
        s = s.offset(1);
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int {
    strncmp(s1, s2, usize::MAX)
}

#[no_mangle]
pub unsafe extern "C" fn strcoll(s1: *const c_char, s2: *const c_char) -> c_int {
    // relibc has no locale stuff (yet)
    strcmp(s1, s2)
}

#[no_mangle]
pub unsafe extern "C" fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char {
    let mut i = 0;

    loop {
        let byte = *src.offset(i);
        *dst.offset(i) = byte;

        if byte == 0 {
            break;
        }

        i += 1;
    }

    dst
}

#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const c_char) -> size_t {
    strnlen(s, usize::MAX)
}

#[no_mangle]
pub unsafe extern "C" fn strnlen(s: *const c_char, size: size_t) -> size_t {
    let mut i = 0;
    while i < size {
        if *s.add(i) == 0 {
            break;
        }
        i += 1;
    }
    i as size_t
}

#[no_mangle]
pub unsafe extern "C" fn strnlen_s(s: *const c_char, size: size_t) -> size_t {
    if s.is_null() {
        0
    } else {
        strnlen(s, size)
    }
}

#[no_mangle]
pub unsafe extern "C" fn strcat(s1: *mut c_char, s2: *const c_char) -> *mut c_char {
    strncat(s1, s2, usize::MAX)
}

#[no_mangle]
pub unsafe extern "C" fn strncat(s1: *mut c_char, s2: *const c_char, n: size_t) -> *mut c_char {
    let len = strlen(s1 as *const c_char);
    let mut i = 0;
    while i < n {
        let b = *s2.add(i);
        if b == 0 {
            break;
        }

        *s1.add(len + i) = b;
        i += 1;
    }
    *s1.add(len + i) = 0;

    s1
}

#[no_mangle]
pub unsafe extern "C" fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int {
    let s1 = core::slice::from_raw_parts(s1 as *const c_uchar, n);
    let s2 = core::slice::from_raw_parts(s2 as *const c_uchar, n);

    for (&a, &b) in s1.iter().zip(s2.iter()) {
        let val = (a as c_int) - (b as c_int);
        if a != b || a == 0 {
            return val;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char {
    let mut i = 0;

    while *src.add(i) != 0 && i < n {
        *dst.add(i) = *src.add(i);
        i += 1;
    }

    for i in i..n {
        *dst.add(i) = 0;
    }

    dst
}

#[no_mangle]
pub unsafe extern "C" fn strrchr(s: *const c_char, c: c_int) -> *mut c_char {
    let len = strlen(s) as isize;
    let c = c as i8;
    let mut i = len - 1;
    while i >= 0 {
        if *s.offset(i) == c {
            return s.offset(i) as *mut c_char;
        }
        i -= 1;
    }
    ptr::null_mut()
}

unsafe fn inner_strstr(
    mut haystack: *const c_char,
    needle: *const c_char,
    mask: c_char,
) -> *mut c_char {
    while *haystack != 0 {
        let mut i = 0;
        loop {
            if *needle.offset(i) == 0 {
                // We reached the end of the needle, everything matches this far
                return haystack as *mut c_char;
            }
            if *haystack.offset(i) & mask != *needle.offset(i) & mask {
                break;
            }

            i += 1;
        }

        haystack = haystack.offset(1);
    }
    ptr::null_mut()
}
