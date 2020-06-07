use crate::{LvResult, NativeObject};
use cstr_core::CString;

define_object!(Label, lv_label_create);

impl Label {
    pub fn set_text(&mut self, text: &str) -> LvResult<()> {
        let text = CString::new(text).unwrap();
        unsafe {
            lvgl_sys::lv_label_set_text(self.core.raw()?.as_mut(), text.as_ptr());
        }
        Ok(())
    }

    pub fn set_label_align(&mut self, align: LabelAlign) -> LvResult<()> {
        let align = match align {
            LabelAlign::Left => lvgl_sys::LV_LABEL_ALIGN_LEFT,
            LabelAlign::Center => lvgl_sys::LV_LABEL_ALIGN_CENTER,
            LabelAlign::Right => lvgl_sys::LV_LABEL_ALIGN_RIGHT,
            LabelAlign::Auto => lvgl_sys::LV_LABEL_ALIGN_AUTO,
        } as lvgl_sys::lv_label_align_t;
        unsafe {
            lvgl_sys::lv_label_set_align(self.core.raw()?.as_mut(), align);
        }
        Ok(())
    }

    pub fn set_recolor(&mut self, recolor: bool) -> LvResult<()> {
        unsafe {
            lvgl_sys::lv_label_set_recolor(self.core.raw()?.as_mut(), recolor);
        }
        Ok(())
    }
}

pub enum LabelAlign {
    Left,
    Center,
    Right,
    Auto,
}
