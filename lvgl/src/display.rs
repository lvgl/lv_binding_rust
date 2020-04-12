
use crate::objx::Object;
use core::ptr;

pub fn get_active_screen() -> Object {
    let raw = unsafe {
        ptr::NonNull::new_unchecked(lvgl_sys::lv_disp_get_scr_act(ptr::null_mut()))
    };
    Object::new(raw)
}
