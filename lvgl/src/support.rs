use alloc::boxed::Box;
use core::mem;
use core::ptr;
use embedded_graphics::pixelcolor::{Rgb565, Rgb888};
use lvgl_sys;

pub trait NativeObject {
    fn raw(&self) -> ptr::NonNull<lvgl_sys::lv_obj_t>;
}

pub struct ObjectX {
    raw: ptr::NonNull<lvgl_sys::lv_obj_t>,
}

impl ObjectX {
    pub(crate) fn from_raw(raw: ptr::NonNull<lvgl_sys::lv_obj_t>) -> Self {
        Self { raw }
    }
}

impl NativeObject for ObjectX {
    fn raw(&self) -> ptr::NonNull<lvgl_sys::lv_obj_t> {
        unsafe { ptr::NonNull::new_unchecked(self.raw.as_ptr()) }
    }
}

pub trait Object: NativeObject {
    fn set_pos(&mut self, x: i16, y: i16) {
        unsafe {
            lvgl_sys::lv_obj_set_pos(
                self.raw().as_mut(),
                x as lvgl_sys::lv_coord_t,
                y as lvgl_sys::lv_coord_t,
            );
        }
    }

    fn set_size(&mut self, w: i16, h: i16) {
        unsafe {
            lvgl_sys::lv_obj_set_size(
                self.raw().as_mut(),
                w as lvgl_sys::lv_coord_t,
                h as lvgl_sys::lv_coord_t,
            );
        }
    }

    fn set_width(&mut self, w: u32) {
        unsafe {
            lvgl_sys::lv_obj_set_width(self.raw().as_mut(), w as lvgl_sys::lv_coord_t);
        }
    }

    fn set_height(&mut self, h: u32) {
        unsafe {
            lvgl_sys::lv_obj_set_height(self.raw().as_mut(), h as lvgl_sys::lv_coord_t);
        }
    }

    fn set_align<C>(&mut self, base: &mut C, align: Align, x_mod: i32, y_mod: i32)
    where
        C: NativeObject,
    {
        let align = match align {
            Align::Center => lvgl_sys::LV_ALIGN_CENTER,
            Align::InTopLeft => lvgl_sys::LV_ALIGN_IN_TOP_LEFT,
            Align::InTopMid => lvgl_sys::LV_ALIGN_IN_TOP_MID,
            Align::InTopRight => lvgl_sys::LV_ALIGN_IN_TOP_RIGHT,
            Align::InBottomLeft => lvgl_sys::LV_ALIGN_IN_BOTTOM_LEFT,
            Align::InBottomMid => lvgl_sys::LV_ALIGN_IN_BOTTOM_MID,
            Align::InBottomRight => lvgl_sys::LV_ALIGN_IN_BOTTOM_RIGHT,
            Align::InLeftMid => lvgl_sys::LV_ALIGN_IN_LEFT_MID,
            Align::InRightMid => lvgl_sys::LV_ALIGN_IN_RIGHT_MID,
            Align::OutTopLeft => lvgl_sys::LV_ALIGN_OUT_TOP_LEFT,
            Align::OutTopMid => lvgl_sys::LV_ALIGN_OUT_TOP_MID,
            Align::OutTopRight => lvgl_sys::LV_ALIGN_OUT_TOP_RIGHT,
            Align::OutBottomLeft => lvgl_sys::LV_ALIGN_OUT_BOTTOM_LEFT,
            Align::OutBottomMid => lvgl_sys::LV_ALIGN_OUT_BOTTOM_MID,
            Align::OutBottomRight => lvgl_sys::LV_ALIGN_OUT_BOTTOM_RIGHT,
            Align::OutLeftTop => lvgl_sys::LV_ALIGN_OUT_LEFT_TOP,
            Align::OutLeftMid => lvgl_sys::LV_ALIGN_OUT_LEFT_MID,
            Align::OutLeftBottom => lvgl_sys::LV_ALIGN_OUT_LEFT_BOTTOM,
            Align::OutRightTop => lvgl_sys::LV_ALIGN_OUT_RIGHT_TOP,
            Align::OutRightMid => lvgl_sys::LV_ALIGN_OUT_RIGHT_MID,
            Align::OutRightBottom => lvgl_sys::LV_ALIGN_OUT_RIGHT_BOTTOM,
        } as lvgl_sys::lv_align_t;
        unsafe {
            lvgl_sys::lv_obj_align(
                self.raw().as_mut(),
                base.raw().as_mut(),
                align,
                x_mod as lvgl_sys::lv_coord_t,
                y_mod as lvgl_sys::lv_coord_t,
            );
        }
    }

    fn set_style(&mut self, style: Style);
}

impl Object for ObjectX {
    fn set_style(&mut self, style: Style) {
        unsafe {
            let boxed = Box::new(style.raw);
            lvgl_sys::lv_obj_set_style(self.raw().as_mut(), Box::into_raw(boxed));
        };
    }
}

macro_rules! define_object {
    ($item:ident) => {
        pub struct $item {
            core: $crate::support::ObjectX,
        }

        impl $crate::support::NativeObject for $item {
            fn raw(&self) -> core::ptr::NonNull<lvgl_sys::lv_obj_t> {
                self.core.raw()
            }
        }

        impl $crate::support::Object for $item {
            fn set_style(&mut self, style: $crate::support::Style) {
                unsafe {
                    let boxed = alloc::boxed::Box::new(style.raw);
                    lvgl_sys::lv_obj_set_style(
                        self.raw().as_mut(),
                        alloc::boxed::Box::into_raw(boxed),
                    );
                };
            }
        }
    };
}

pub enum Themes {
    Pretty,
}

pub struct Style {
    pub(crate) raw: lvgl_sys::lv_style_t,
}

impl Style {
    pub fn new() -> Self {
        let raw = unsafe {
            let mut native_style = mem::MaybeUninit::<lvgl_sys::lv_style_t>::uninit();
            lvgl_sys::lv_style_copy(native_style.as_mut_ptr(), &lvgl_sys::lv_style_pretty);
            native_style.assume_init()
        };
        Self { raw }
    }

    /// Object's main background color.
    pub fn set_body_main_color(&mut self, color: Color) {
        self.raw.body.main_color = color.raw;
    }

    /// Second color. If not equal to `set_body_main_color` a gradient will be drawn for the background.
    pub fn set_body_grad_color(&mut self, color: Color) {
        self.raw.body.grad_color = color.raw;
    }

    /// Text color.
    pub fn set_text_color(&mut self, color: Color) {
        self.raw.text.color = color.raw;
    }

    /// Font used for displaying the text.
    pub fn set_text_font(&mut self, font: &lvgl_sys::lv_font_t) {
        self.raw.text.font = font;
    }

    /// Body radius for rounded corners.
    pub fn set_body_radius(&mut self, radius: i16) {
        self.raw.body.radius = radius;
    }

    /// Border color.
    pub fn set_body_border_color(&mut self, color: Color) {
        self.raw.body.border.color = color.raw;
    }
}

impl Clone for Style {
    fn clone(&self) -> Self {
        let mut native_style = mem::MaybeUninit::<lvgl_sys::lv_style_t>::uninit();
        unsafe {
            lvgl_sys::lv_style_copy(
                native_style.as_mut_ptr(),
                &self.raw as *const lvgl_sys::lv_style_t,
            );
            Self {
                raw: native_style.assume_init(),
            }
        }
    }
}

#[derive(Clone)]
pub struct Color {
    raw: lvgl_sys::lv_color_t,
}

impl Color {
    pub fn from_rgb((r, g, b): (u8, u8, u8)) -> Self {
        let raw = unsafe { lvgl_sys::_LV_COLOR_MAKE(r, g, b) };
        Self { raw }
    }

    pub fn from_raw(raw: lvgl_sys::lv_color_t) -> Self {
        Self { raw }
    }
}

impl From<Color> for Rgb888 {
    fn from(color: Color) -> Self {
        unsafe {
            Rgb888::new(
                lvgl_sys::_LV_COLOR_GET_R(color.raw) as u8,
                lvgl_sys::_LV_COLOR_GET_G(color.raw) as u8,
                lvgl_sys::_LV_COLOR_GET_B(color.raw) as u8,
            )
        }
    }
}

impl From<Color> for Rgb565 {
    fn from(color: Color) -> Self {
        unsafe {
            Rgb565::new(
                lvgl_sys::_LV_COLOR_GET_R(color.raw) as u8,
                lvgl_sys::_LV_COLOR_GET_G(color.raw) as u8,
                lvgl_sys::_LV_COLOR_GET_B(color.raw) as u8,
            )
        }
    }
}

pub enum Align {
    Center,
    InTopLeft,
    InTopMid,
    InTopRight,
    InBottomLeft,
    InBottomMid,
    InBottomRight,
    InLeftMid,
    InRightMid,
    OutTopLeft,
    OutTopMid,
    OutTopRight,
    OutBottomLeft,
    OutBottomMid,
    OutBottomRight,
    OutLeftTop,
    OutLeftMid,
    OutLeftBottom,
    OutRightTop,
    OutRightMid,
    OutRightBottom,
}

pub enum Animation {
    ON,
    OFF,
}

impl From<Animation> for lvgl_sys::lv_anim_enable_t {
    fn from(anim: Animation) -> Self {
        match anim {
            Animation::ON => lvgl_sys::LV_ANIM_ON as u8,
            Animation::OFF => lvgl_sys::LV_ANIM_OFF as u8,
        }
    }
}
