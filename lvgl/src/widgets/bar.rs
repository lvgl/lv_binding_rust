use crate::support::Animation;
use crate::support::{NativeObject, ObjectX};
use core::ptr;
use lvgl_sys;

define_object!(Bar);

impl Bar {
    pub fn new<C>(parent: &mut C) -> Self
    where
        C: NativeObject,
    {
        let raw = unsafe {
            let ptr = lvgl_sys::lv_bar_create(parent.raw().as_mut(), ptr::null_mut());
            ptr::NonNull::new_unchecked(ptr)
        };
        let core = ObjectX::from_raw(raw);
        Self { core }
    }

    /// Set minimum and the maximum values of the bar
    pub fn set_range(&mut self, min: i16, max: i16) {
        unsafe {
            lvgl_sys::lv_bar_set_range(self.core.raw().as_mut(), min, max);
        }
    }

    /// Set a new value on the bar
    pub fn set_value(&mut self, value: i16, anim: Animation) {
        unsafe {
            lvgl_sys::lv_bar_set_value(self.core.raw().as_mut(), value, anim.into());
        }
    }
}
