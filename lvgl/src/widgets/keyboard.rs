use crate::widgets::{Keyboard, Textarea};
use crate::LvResult;
use crate::NativeObject;

impl Keyboard<'_> {
    /// Associates a given `Textarea` to the keyboard.
    pub fn set_textarea(&mut self, textarea: &mut Textarea) -> LvResult<()> {
        unsafe {
            lvgl_sys::lv_keyboard_set_textarea(
                self.raw()?.as_mut() as *mut lvgl_sys::lv_obj_t,
                textarea.raw()?.as_mut() as *mut lvgl_sys::lv_obj_t,
            )
        }
        Ok(())
    }
}
