use crate::{LvError, LvResult};
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
    pub fn new(value: T) -> LvResult<Box<T>> {
        let size = mem::size_of::<T>();
        let inner = unsafe {
            let ptr = lvgl_sys::lv_mem_alloc(size as lvgl_sys::size_t) as *mut T;

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
                .ok_or(LvError::LvOOMemory)?
        };
        Ok(Box(inner))
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

#[cfg(test)]
mod test {
    use super::*;
    use core::mem::MaybeUninit;
    use std::sync::Once;
    use std::vec::Vec;

    static INIT_LVGL: Once = Once::new();

    fn init() {
        INIT_LVGL.call_once(|| {
            unsafe {
                lvgl_sys::lv_init();
            };
        });
    }

    #[test]
    fn place_value_in_lv_mem() {
        init();

        let v = Box::new(5).unwrap();
        drop(v);
        let v = Box::new(10).unwrap();
        drop(v);
    }

    #[test]
    fn place_complex_value_in_lv_mem() {
        init();

        #[repr(C)]
        #[derive(Debug)]
        struct Point {
            x: u64,
            y: i8,
            t: i32,
            disp: i32,
        }

        print_mem_info();
        let total_mem_available_initially = get_mem_info().free_size;

        let mut keep = Vec::new();
        for i in 0..100 {
            let p = Point {
                x: i,
                y: 42,
                t: 0,
                disp: -100,
            };

            println!("{:?}", p);
            let mut b = Box::new(p).unwrap_or_else(|_| {
                print_mem_info();
                panic!("OOM");
            });

            println!("memory address is {:p}", b.as_mut());

            let point = b.as_mut();
            if point.x != i {
                print_mem_info();

                println!("{:?}", point);
            }
            assert_eq!(point.x, i);

            print_mem_info();
            keep.push(b);
        }
        drop(keep);

        print_mem_info();
        unsafe {
            lvgl_sys::lv_mem_defrag();
        }
        print_mem_info();

        // If this fails, we are leaking memory! BOOM! \o/
        assert_eq!(total_mem_available_initially, get_mem_info().free_size)
    }

    fn get_mem_info() -> lvgl_sys::lv_mem_monitor_t {
        let mut info: MaybeUninit<lvgl_sys::lv_mem_monitor_t> = MaybeUninit::uninit();
        unsafe {
            lvgl_sys::lv_mem_monitor(info.as_mut_ptr());
        }
        if !info.as_ptr().is_null() {
            unsafe { info.assume_init() }
        } else {
            panic!("Could not get memory info from LVGL! :(");
        }
    }

    fn print_mem_info() {
        println!("mem info: {:?}", get_mem_info());
    }
}
