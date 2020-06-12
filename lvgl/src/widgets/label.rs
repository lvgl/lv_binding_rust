use crate::widgets::Label;
use crate::{LvResult, NativeObject};

impl Label {
    pub fn set_label_align(&mut self, align: LabelAlign) -> LvResult<()> {
        unsafe {
            lvgl_sys::lv_label_set_align(self.core.raw()?.as_mut(), align as u8);
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum LabelAlign {
    Left = lvgl_sys::LV_LABEL_ALIGN_LEFT as u8,
    Center = lvgl_sys::LV_LABEL_ALIGN_CENTER as u8,
    Right = lvgl_sys::LV_LABEL_ALIGN_RIGHT as u8,
    Auto = lvgl_sys::LV_LABEL_ALIGN_AUTO as u8,
}
