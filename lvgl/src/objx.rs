use core::ptr;
use cty;
use lvgl_sys;
use core::mem;

pub trait NativeObject {
    fn raw(&self) -> ptr::NonNull<lvgl_sys::lv_obj_t>;
}

pub struct ObjectX<'a> {
    raw: ptr::NonNull<lvgl_sys::lv_obj_t>,
    style: Option<Style<'a>>,
}

impl<'a> ObjectX<'a> {
    pub(crate) fn new(raw: ptr::NonNull<lvgl_sys::lv_obj_t>) -> Self {
        Self { raw, style: None }
    }
}

impl<'a> NativeObject for ObjectX<'a> {
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

    fn set_align<C>(
        &mut self,
        base: &mut C,
        align: Align,
        x_mod: i32,
        y_mod: i32,
    ) where C: NativeObject {
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

    //fn set_style(&mut self, style: Style<'static>);
    fn set_style(&mut self, style: &mut lvgl_sys::lv_style_t);
}

macro_rules! define_object {
    ($item:ident) => {
        pub struct $item<'a> {
            core: ObjectX<'a>,
        }

        impl<'a> NativeObject for $item<'a> {
            fn raw(&self) -> ptr::NonNull<lvgl_sys::lv_obj_t> {
                self.core.raw()
            }
        }

        impl<'a> Object for $item<'a> {
            fn set_style(&mut self, style: &mut lvgl_sys::lv_style_t) {
                unsafe {
                    lvgl_sys::lv_obj_set_style(self.raw().as_mut(), style);
                };
                //self.core.style = Some(style);
            }
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

define_object!(Button);

impl<'a> Button<'a> {
    pub fn new<C>(parent: &mut C) -> Self where C: NativeObject {
        let raw = unsafe {
            let ptr = lvgl_sys::lv_btn_create(parent.raw().as_mut(), ptr::null_mut());
            ptr::NonNull::new_unchecked(ptr)
        };
        let core = ObjectX::new(raw);
        Self { core }
    }
}

pub enum LabelAlign {
    Left,
    Center,
    Right,
    Auto,
}

define_object!(Label);

impl<'a> Label<'a> {
    pub fn new<C>(parent: &mut C) -> Self where C: NativeObject {
        let raw = unsafe {
            let ptr = lvgl_sys::lv_label_create(parent.raw().as_mut(), ptr::null_mut());
            ptr::NonNull::new_unchecked(ptr)
        };
        let core = ObjectX::new(raw);
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
}

pub enum Themes {
    Pretty,
}

#[derive(Default)]
pub struct Style<'a> {
    raw: Option<lvgl_sys::lv_style_t>,
    pub text: TextStyle<'a>,
}

#[derive(Default)]
pub struct TextStyle<'a> {
    pub font: Option<&'a lvgl_sys::lv_font_t>,
}

impl<'a> Style<'a> {
    fn raw(&mut self) -> *const lvgl_sys::lv_style_t {
        match self.raw {
            Some(mut native_pointer) => unsafe {
                &mut native_pointer
            }
            None => unsafe {
                // TODO: Create the native struct and save to self
                let mut native_style = mem::MaybeUninit::<lvgl_sys::lv_style_t>::uninit().assume_init();
                lvgl_sys::lv_style_copy(&mut native_style, &lvgl_sys::lv_style_pretty);
                if let Some(text_font) = self.text.font {
                    native_style.text.font = text_font;
                }
                self.raw = Some(native_style);
                &mut self.raw.unwrap()
            }
        }
    }
}

