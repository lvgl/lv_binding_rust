use crate::{GenericObject, NativeObject, Object};
use core::ptr;

define_object!(Gauge, part = GaugePart);

impl Gauge {
    pub fn new<C>(parent: &mut C) -> Self
    where
        C: NativeObject,
    {
        unsafe {
            let ptr = lvgl_sys::lv_gauge_create(parent.raw().as_mut(), ptr::null_mut());
            let raw = ptr::NonNull::new_unchecked(ptr);
            let core = GenericObject::from_raw(raw);
            Self { core }
        }
    }

    /// Set a new value on the gauge
    pub fn set_value(&mut self, needle_id: u8, value: i32) {
        unsafe {
            lvgl_sys::lv_gauge_set_value(self.core.raw().as_mut(), needle_id, value);
        }
    }
}

pub enum GaugePart {
    Main,
    Major,
    Needle,
}

impl From<GaugePart> for u8 {
    fn from(part: GaugePart) -> Self {
        match part {
            GaugePart::Main => lvgl_sys::LV_GAUGE_PART_MAIN as u8,
            GaugePart::Major => lvgl_sys::LV_GAUGE_PART_MAJOR as u8,
            GaugePart::Needle => lvgl_sys::LV_GAUGE_PART_NEEDLE as u8,
        }
    }
}
