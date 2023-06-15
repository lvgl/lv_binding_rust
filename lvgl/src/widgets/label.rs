use crate::widgets::Label;
use crate::{LabelLongMode, LvResult, NativeObject};

#[cfg(feature = "alloc")]
mod alloc_imp {
    use crate::widgets::Label;
    //use crate::LvError;
    use cstr_core::CString;
    //use core::convert::TryFrom;

    impl<S: AsRef<str>> From<S> for Label {
        fn from(text: S) -> Self {
            // text.try_into().unwrap()
            let text_cstr = CString::new(text.as_ref()).unwrap();
            let mut label = Label::new().unwrap();
            label.set_text(text_cstr.as_c_str()).unwrap();
            label
        }
    }

    // Issue link: https://github.com/rust-lang/rust/issues/50133
    //
    // impl<S: AsRef<str>> TryFrom<S> for Label {
    //     type Error = LvError;
    //     fn try_from(text: S) -> Result<Self, Self::Error> {
    //         let text_cstr = CString::new(text.as_ref())?;
    //         let mut label = Label::new()?;
    //         label.set_text(text_cstr.as_c_str())?;
    //         Ok(label)
    //     }
    // }
}

impl Label {
    pub fn set_long_mode(&mut self, long_mode: LabelLongMode) -> LvResult<()> {
        unsafe {
            Ok(lvgl_sys::lv_label_set_long_mode(
                self.raw()?.as_mut(),
                long_mode.into(),
            ))
        }
    }

    pub fn get_long_mode(&self) -> LvResult<u8> {
        unsafe { Ok(lvgl_sys::lv_label_get_long_mode(self.raw()?.as_ref())) }
    }
}
