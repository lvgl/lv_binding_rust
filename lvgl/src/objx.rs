use core::ptr;
use cty;
use lvgl_sys;

pub trait Container {
    fn raw(&self) -> ptr::NonNull<lvgl_sys::lv_obj_t>;
}

pub struct Object {
    raw: ptr::NonNull<lvgl_sys::lv_obj_t>,
}

impl Object {
    pub(crate) fn new(raw: ptr::NonNull<lvgl_sys::lv_obj_t>) -> Self {
        Self { raw }
    }
}

impl Container for Object {
    fn raw(&self) -> ptr::NonNull<lvgl_sys::lv_obj_t> {
        unsafe { ptr::NonNull::new_unchecked(self.raw.as_ptr()) }
    }
}

pub struct Button {
    core: Object,
}

impl Button {
    pub fn new(parent: &mut dyn Container) -> Self {
        let raw = unsafe {
            let ptr = lvgl_sys::lv_btn_create(parent.raw().as_mut(), ptr::null_mut());
            ptr::NonNull::new_unchecked(ptr)
        };
        let core = Object::new(raw);
        Self { core }
    }

    pub fn set_pos(&mut self, x: u16, y: u16) {
        unsafe {
            lvgl_sys::lv_obj_set_pos(
                self.raw().as_mut(),
                x as lvgl_sys::lv_coord_t,
                y as lvgl_sys::lv_coord_t,
            );
        }
    }
}

impl Container for Button {
    fn raw(&self) -> ptr::NonNull<lvgl_sys::lv_obj_t> {
        self.core.raw()
    }
}

pub struct Label {
    core: Object,
}

impl Label {
    pub fn new(parent: &mut dyn Container) -> Self {
        let raw = unsafe {
            let ptr = lvgl_sys::lv_label_create(parent.raw().as_mut(), ptr::null_mut());
            ptr::NonNull::new_unchecked(ptr)
        };
        let core = Object::new(raw);
        Self { core }
    }

    pub fn set_text(&mut self, text: &str) {
        unsafe {
            lvgl_sys::lv_label_set_text(
                self.core.raw().as_mut(),
                text.as_ptr() as *const cty::c_char,
            );
        }
    }
}

impl Container for Label {
    fn raw(&self) -> ptr::NonNull<lvgl_sys::lv_obj_t> {
        self.core.raw()
    }
}
