use crate::{GenericObject, NativeObject, Object};
use core::ptr;
use cstr_core::CString;

pub enum LabelAlign {
    Left,
    Center,
    Right,
    Auto,
}

define_object!(Label);

impl Label {
    pub fn new<C>(parent: &mut C) -> Self
    where
        C: NativeObject,
    {
        unsafe {
            let ptr = lvgl_sys::lv_label_create(parent.raw().as_mut(), ptr::null_mut());
            let raw = ptr::NonNull::new_unchecked(ptr);
            let core = GenericObject::from_raw(raw);
            Self { core }
        }
    }

    pub fn set_text(&mut self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe {
            lvgl_sys::lv_label_set_text(self.core.raw().as_mut(), text.as_ptr());
        }
    }

    pub fn set_label_align(&mut self, align: LabelAlign) {
        let align = match align {
            LabelAlign::Left => lvgl_sys::LV_LABEL_ALIGN_LEFT,
            LabelAlign::Center => lvgl_sys::LV_LABEL_ALIGN_CENTER,
            LabelAlign::Right => lvgl_sys::LV_LABEL_ALIGN_RIGHT,
            LabelAlign::Auto => lvgl_sys::LV_LABEL_ALIGN_AUTO,
        } as lvgl_sys::lv_label_align_t;
        unsafe {
            lvgl_sys::lv_label_set_align(self.core.raw().as_mut(), align);
        }
    }

    pub fn set_recolor(&mut self, recolor: bool) {
        unsafe {
            lvgl_sys::lv_label_set_recolor(self.core.raw().as_mut(), recolor);
        }
    }
}
