use core::mem;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

/// Places a sized `T` into LVGL memory.
///
/// This is useful for cases when we need to allocate memory on Rust side
/// and handover the management of that memory to LVGL. May also be used in cases we
/// want to use dynamic memory in the Rust side.
pub(crate) struct Box<T>(NonNull<T>);

impl<T> Box<T> {
    /// Allocate memory using LVGL memory API and place `T` in the LVGL tracked memory.
    pub fn new(value: T) -> Box<T> {
        let size = mem::size_of::<T>();
        let inner = unsafe {
            let ptr = lvgl_sys::lv_mem_alloc(size as cty::size_t) as *mut T;

            // LVGL should align the memory address for us!
            assert_eq!(
                ptr as usize % mem::align_of::<T>(),
                0,
                "Memory address not aligned!"
            );

            NonNull::new(ptr)
                .map(|p| {
                    p.as_ptr().write(value);
                    p
                })
                .unwrap_or_else(|| {
                    panic!("Could not allocate memory {} bytes", size);
                })
        };
        Box(inner)
    }

    pub fn into_raw(self) -> *mut T {
        let b = mem::ManuallyDrop::new(self);
        b.0.as_ptr()
    }
}

impl<T> Drop for Box<T> {
    fn drop(&mut self) {
        unsafe {
            lvgl_sys::lv_mem_free(self.0.as_ptr() as *const cty::c_void);
        }
    }
}

impl<T> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T> Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<T> AsMut<T> for Box<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
    }
}

impl<T: Clone> Clone for Box<T> {
    fn clone(&self) -> Self {
        unsafe { Self::new(self.0.as_ref().clone()) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::vec::Vec;
    use core::mem::MaybeUninit;

    fn init() {
        unsafe {
            lvgl_sys::lv_init();
        };
    }

    fn teardown() {
        unsafe {
            lvgl_sys::lv_deinit();
        }
    }

    #[test]
    fn place_value_in_lv_mem() {
        crate::lvgl_init();

        let v = Box::new(5);
        drop(v);
        let v = Box::new(10);
        drop(v);

        teardown();
    }

    #[test]
    fn place_complex_value_in_lv_mem() {
        crate::lvgl_init();

        #[repr(C)]
        #[derive(Debug)]
        struct Point {
            x: u64,
            y: i8,
            t: i32,
            disp: i32,
        }

        let initial_mem_info = mem_info();

        let mut keep = Vec::new();
        for i in 0..100 {
            let p = Point {
                x: i,
                y: 42,
                t: 0,
                disp: -100,
            };

            println!("{:?}", p);
            let mut b = Box::new(p);

            println!("memory address is {:p}", b.as_mut());

            let point = b.as_mut();
            if point.x != i {
                println!("{:?}", point);
            }
            assert_eq!(point.x, i);

            let info = mem_info();
            println!("mem info: {:?}", &info);
            keep.push(b);
        }
        drop(keep);

        unsafe {
            lvgl_sys::lv_mem_defrag();
        }

        let final_info = mem_info();
        println!("mem info: {:?}", &final_info);

        // If this fails, we are leaking memory! BOOM! \o/
        assert_eq!(initial_mem_info.free_size, final_info.free_size);

        teardown();
    }

    #[test]
    fn clone_object_in_lv_mem() {
        crate::lvgl_init();

        let v1 = Box::new(5);
        let v2 = v1.clone();

        // Ensure that the two objects have identical values.
        assert_eq!(*v1, *v2);
        // They should have different memory addresses, however.
        assert_ne!(v1.into_raw() as usize, v2.into_raw() as usize);
    }

    fn mem_info() -> lvgl_sys::lv_mem_monitor_t {
        let mut info = lvgl_sys::lv_mem_monitor_t {
            total_size: 0,
            free_cnt: 0,
            free_size: 0,
            free_biggest_size: 0,
            used_cnt: 0,
            max_used: 0,
            used_pct: 0,
            frag_pct: 0,
        };
        unsafe {
            lvgl_sys::lv_mem_monitor(&mut info as *mut _);
        }
        info
    }
}
