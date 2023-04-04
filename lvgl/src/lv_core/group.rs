use crate::{LvError, LvResult, NativeObject};
use crate::input_device::InputDriver;
use core::ptr::NonNull;

/// A group of objects, for use with `Encoder` and `Keypad`-type input devices.
pub struct Group {
    raw: *mut lvgl_sys::lv_group_t
}

impl Default for Group {
    fn default() -> Self {
        Group {
            raw: unsafe { lvgl_sys::lv_group_create() }
        }
    }
}

impl Group {
    /// Returns a pointer to the underlying `lv_group_t`.
    pub fn raw(&self) -> LvResult<NonNull<lvgl_sys::lv_group_t>> {
        if let Some(non_null_ptr) = NonNull::new(self.raw) {
            Ok(non_null_ptr)
        }
        else {
            Err(LvError::InvalidReference)
        }
    }

    /// Adds an object to the group.
    pub fn add_obj(&mut self, obj: &impl NativeObject) -> LvResult<()> {
        unsafe {
            lvgl_sys::lv_group_add_obj(self.raw()?.as_mut(), obj.raw()?.as_mut())
        }
        Ok(())
    }

    /// Associates an input device to the group.
    pub fn set_indev<D>(&mut self, indev: &mut impl InputDriver<D>) -> LvResult<()> {
        let dsc = match indev.get_descriptor() {
            Some(d) => d,
            None => return Err(LvError::InvalidReference)
        };
        unsafe {
            lvgl_sys::lv_indev_set_group(dsc as *mut lvgl_sys::lv_indev_t, self.raw()?.as_mut())
        }
        Ok(())
    }
}

impl Drop for Group {
    fn drop(&mut self) {
        unsafe {
            if let Ok(mut p) = self.raw() {
                lvgl_sys::lv_group_del(p.as_mut())
            }
        }
    }
}
