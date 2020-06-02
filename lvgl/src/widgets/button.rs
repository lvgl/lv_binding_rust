use crate::{GenericObject, NativeObject, Object};
use core::ptr;

define_object!(Button);

impl Button {
    pub fn new<C>(parent: &mut C) -> Self
    where
        C: NativeObject,
    {
        unsafe {
            let ptr = lvgl_sys::lv_btn_create(parent.raw().as_mut(), ptr::null_mut());
            let raw = ptr::NonNull::new_unchecked(ptr);
            let core = GenericObject::from_raw(raw);
            Self { core }
        }
    }
}
