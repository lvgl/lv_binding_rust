use crate::support::{Animation, NativeObject, ObjectX, Style};
use crate::Object;
use alloc::boxed::Box;
use core::ptr;
use lvgl_sys;

define_object!(Bar);

impl Bar {
    pub fn new<C>(parent: &mut C) -> Self
    where
        C: NativeObject,
    {
        unsafe {
            let ptr = lvgl_sys::lv_bar_create(parent.raw().as_mut(), ptr::null_mut());
            let raw = ptr::NonNull::new_unchecked(ptr);
            let core = ObjectX::from_raw(raw);
            Self { core }
        }
    }

    /// Set minimum and the maximum values of the bar
    pub fn set_range(&mut self, min: i16, max: i16) {
        unsafe {
            lvgl_sys::lv_bar_set_range(self.core.raw().as_mut(), min, max);
        }
    }

    /// Set a new value on the bar
    pub fn set_value(&mut self, value: i16, anim: Animation) {
        unsafe {
            lvgl_sys::lv_bar_set_value(self.core.raw().as_mut(), value, anim.into());
        }
    }

    /// Set the style, for the given `BarComponent`
    pub fn set_bar_style(&mut self, component: BarComponent, style: Style) {
        let boxed = Box::new(style.raw);
        unsafe {
            lvgl_sys::lv_bar_set_style(
                self.core.raw().as_mut(),
                component.into(),
                Box::into_raw(boxed),
            );
        }
    }
}

/// The different components, of a bar object.
pub enum BarComponent {
    /// The background of the bar.
    Background,
    /// The indicator of the bar.
    /// This is what moves/changes, depending on the bar's value.
    Indicator,
}

impl From<BarComponent> for lvgl_sys::lv_bar_style_t {
    fn from(component: BarComponent) -> Self {
        match component {
            BarComponent::Background => lvgl_sys::LV_BAR_STYLE_BG as u8,
            BarComponent::Indicator => lvgl_sys::LV_BAR_STYLE_INDIC as u8,
        }
    }
}
