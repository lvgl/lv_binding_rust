use crate::objx::ObjectX;
use core::ptr;

pub fn get_active_screen() -> ObjectX {
    let raw =
        unsafe { ptr::NonNull::new_unchecked(lvgl_sys::lv_disp_get_scr_act(ptr::null_mut())) };
    ObjectX::new(raw)
}
