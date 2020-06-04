use crate::lv_core::style::Style;
use crate::Align;
use alloc::boxed::Box;
use core::ptr;

const PANIC_MESSAGE: &str = "Value was dropped by LittlevGL";

/// Represents a native LittlevGL object
pub trait NativeObject {
    /// Provide common way to access to the underlying native object pointer.
    fn raw(&self) -> ptr::NonNull<lvgl_sys::lv_obj_t>;
}

/// Generic LVGL object.
///
/// This is the parent object of all widget types. It stores the native LVGL raw pointer.
pub struct GenericObject {
    // We use a raw pointer here because we do not control this memory address, it is controlled
    // by LVGL's global state.
    raw: *mut lvgl_sys::lv_obj_t,
}

impl NativeObject for GenericObject {
    fn raw(&self) -> ptr::NonNull<lvgl_sys::lv_obj_t> {
        ptr::NonNull::new(self.raw).expect(PANIC_MESSAGE)
    }
}

/// A wrapper for all LittlevGL common operations on generic objects.
pub trait Object: NativeObject {
    type SpecialEvent;
    type Part: Into<u8>;

    /// Construct an instance of the object from a raw pointer.
    ///
    /// # Safety
    /// Provided the LVGL library can allocate memory this should be safe.
    ///
    unsafe fn from_raw(raw_pointer: ptr::NonNull<lvgl_sys::lv_obj_t>) -> Self;

    fn add_style(&mut self, part: Self::Part, style: Style) {
        unsafe {
            lvgl_sys::lv_obj_add_style(self.raw().as_mut(), part.into(), Box::into_raw(style.raw));
        };
    }

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
        unsafe {
            lvgl_sys::lv_obj_align(
                self.raw().as_mut(),
                base.raw().as_mut(),
                align.into(),
                x_mod as lvgl_sys::lv_coord_t,
                y_mod as lvgl_sys::lv_coord_t,
            );
        }
    }
}

impl Object for GenericObject {
    type SpecialEvent = ();
    type Part = ObjPart;

    unsafe fn from_raw(raw: ptr::NonNull<lvgl_sys::lv_obj_t>) -> Self {
        Self { raw: raw.as_ptr() }
    }
}

impl Default for GenericObject {
    fn default() -> Self {
        Self {
            raw: unsafe { lvgl_sys::lv_obj_create(ptr::null_mut(), ptr::null_mut()) },
        }
    }
}

macro_rules! define_object {
    ($item:ident) => {
        define_object!($item, event = (), part = $crate::ObjPart);
    };
    ($item:ident, event = $event_type:ty) => {
        define_object!($item, event = $event_type, part = $crate::ObjPart);
    };
    ($item:ident, part = $part_type:ty) => {
        define_object!($item, event = (), part = $part_type);
    };
    ($item:ident, part = $part_type:ty, event = $event_type:ty) => {
        define_object!($item, event = $event_type, part = $part_type);
    };
    ($item:ident, event = $event_type:ty, part = $part_type:ty) => {
        pub struct $item {
            core: $crate::GenericObject,
        }

        impl $item {
            pub fn on_event<F>(&mut self, f: F)
            where
                F: FnMut(Self, $crate::support::Event<<Self as $crate::Object>::SpecialEvent>),
            {
                unsafe {
                    let mut raw = self.raw();
                    let obj = raw.as_mut();
                    let user_closure = alloc::boxed::Box::new(f);
                    obj.user_data = alloc::boxed::Box::into_raw(user_closure) as *mut cty::c_void;
                    lvgl_sys::lv_obj_set_event_cb(
                        obj,
                        lvgl_sys::lv_event_cb_t::Some($crate::support::event_callback::<Self, F>),
                    );
                }
            }
        }

        impl $crate::NativeObject for $item {
            fn raw(&self) -> core::ptr::NonNull<lvgl_sys::lv_obj_t> {
                self.core.raw()
            }
        }

        impl $crate::Object for $item {
            type SpecialEvent = $event_type;
            type Part = $part_type;

            unsafe fn from_raw(raw_pointer: core::ptr::NonNull<lvgl_sys::lv_obj_t>) -> Self {
                Self {
                    core: $crate::GenericObject::from_raw(raw_pointer),
                }
            }
        }
    };
}

bitflags! {
    pub struct State: u32 {
        /// Normal, released
        const DEFAULT  = lvgl_sys::LV_STATE_DEFAULT;
        /// Toggled or checked
        const CHECKED  = lvgl_sys::LV_STATE_CHECKED;
        /// Focused via keypad or encoder or clicked via touchpad/mouse
        const FOCUSED  = lvgl_sys::LV_STATE_FOCUSED;
        /// Edit by an encoder
        const EDITED   = lvgl_sys::LV_STATE_EDITED;
        /// Hovered by mouse (not supported now)
        const HOVERED  = lvgl_sys::LV_STATE_HOVERED;
        /// Pressed
        const PRESSED  = lvgl_sys::LV_STATE_PRESSED;
        /// Disabled or inactive
        const DISABLED = lvgl_sys::LV_STATE_DISABLED;
    }
}

impl State {
    pub(crate) fn get_bits(&self) -> u32 {
        self.bits
    }
}

pub enum ObjPart {
    Main,
    All,
}

impl Into<u8> for ObjPart {
    fn into(self) -> u8 {
        match self {
            ObjPart::Main => lvgl_sys::LV_OBJ_PART_MAIN as u8,
            ObjPart::All => lvgl_sys::LV_OBJ_PART_ALL as u8,
        }
    }
}
