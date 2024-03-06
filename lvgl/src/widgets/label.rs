use crate::widgets::Label;
use crate::{LabelLongMode, NativeObject};

impl<'a> Label<'a> {
    pub fn set_long_mode(&mut self, long_mode: LabelLongMode) {
        unsafe {
            lvgl_sys::lv_label_set_long_mode(self.raw().as_mut(), long_mode.into());
        }
    }

    pub fn get_long_mode(&self) -> u8 {
        unsafe { lvgl_sys::lv_label_get_long_mode(self.raw().as_ref()) }
    }

    #[cfg(feature = "alloc")]
    pub fn from_str<S: AsRef<str>>(text: S, parent: &'a mut impl NativeObject) -> Self {
        use cstr_core::CString;
        // text.try_into().unwrap()
        let text_cstr = CString::new(text.as_ref()).unwrap();
        let mut label = Label::create(parent).unwrap();
        label.set_text(text_cstr.as_c_str()).unwrap();
        label
    }
}
