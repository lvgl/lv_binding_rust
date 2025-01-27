use crate::widgets::Textarea;
use crate::NativeObject;
use cstr_core::CStr;

#[cfg(feature = "alloc")]
mod alloc_imp {
    use crate::widgets::Textarea;
    //use crate::LvError;
    use cstr_core::CString;
    //use core::convert::TryFrom;

    impl<S: AsRef<str>> From<S> for Textarea<'_> {
        fn from(text: S) -> Self {
            // text.try_into().unwrap()
            let text_cstr = CString::new(text.as_ref()).unwrap();
            let mut ta = Textarea::new().unwrap();
            ta.set_text(text_cstr.as_c_str()).unwrap();
            ta
        }
    }
}

impl Textarea<'_> {
    pub fn get_text(&self) -> &'static str {
        let char_ptr = unsafe { lvgl_sys::lv_textarea_get_text(self.raw().as_ref()) };
        let c_str = unsafe { CStr::from_ptr(char_ptr) };
        c_str.to_str().unwrap_or_default()
    }
}
