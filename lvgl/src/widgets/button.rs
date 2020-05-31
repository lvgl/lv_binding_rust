use crate::{NativeObject, ObjectX};
use core::ptr;

define_object!(Button);

impl Button {
    pub fn new<C>(parent: &mut C) -> Self
    where
        C: NativeObject,
    {
        let raw = unsafe {
            let ptr = lvgl_sys::lv_btn_create(parent.raw().as_mut(), ptr::null_mut());
            ptr::NonNull::new_unchecked(ptr)
        };
        let core = ObjectX::from_raw(raw);
        Self { core }
    }
}
