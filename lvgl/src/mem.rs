use crate::{LvError, LvResult};
use core::mem;
use core::ops::{Deref, DerefMut};
use core::ptr;
use core::ptr::NonNull;

/// Places `T` into LVGL memory.
pub struct Box<T: Sized>(NonNull<T>);

impl<T: Sized> Box<T> {
    pub fn new(inner: T) -> LvResult<Box<T>> {
        let layout = mem::size_of::<T>();
        let inner = unsafe {
            let ptr = lvgl_sys::lv_mem_alloc(layout as lvgl_sys::size_t) as *mut T;
            match NonNull::new(ptr) {
                Some(v) => {
                    // Place value in new mem
                    ptr::write(ptr, inner);
                    Ok(v)
                }
                None => Err(LvError::LvOOMemory),
            }
        };
        Ok(Box(inner?))
    }

    pub fn into_raw(b: Box<T>) -> *mut T {
        let b = mem::ManuallyDrop::new(b);
        b.0.as_ptr()
    }
}

impl<T: Sized> Drop for Box<T> {
    fn drop(&mut self) {
        unsafe {
            lvgl_sys::lv_mem_free(self.0.as_ptr() as *const cty::c_void);
        }
    }
}

impl<T: Sized> Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}

impl<T: Sized> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
    }
}

impl<T: Sized> AsMut<T> for Box<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn place_value_in_lv_mem() {
        unsafe {
            lvgl_sys::_lv_mem_init();
        };
        let v = Box::new(5).unwrap();
        drop(v);
        let v = Box::new(10).unwrap();
        drop(v);
    }

    #[test]
    fn place_complex_value_in_lv_mem() {
        unsafe {
            lvgl_sys::_lv_mem_init();
        };

        struct Point {
            x: u64,
            y: u64,
            disp: i32,
        }

        let p = Point {
            x: 32,
            y: 240,
            disp: -100,
        };

        let b = Box::new(p).unwrap();

        assert_eq!(b.x, 32);
        assert_eq!(b.y, 240);
        assert_eq!(b.disp, -100);
    }
}
