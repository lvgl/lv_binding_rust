use crate::lv_core::style::Style;
use alloc::boxed::Box;
use core::convert::{TryFrom, TryInto};
use core::ptr;
use core::ptr::NonNull;
use embedded_graphics::pixelcolor::{Rgb565, Rgb888};

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
}

impl Object for GenericObject {
    type SpecialEvent = ();
    type Part = ObjPart;

    unsafe fn from_raw(raw: ptr::NonNull<lvgl_sys::lv_obj_t>) -> Self {
        Self { raw: raw.as_ptr() }
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

impl Default for GenericObject {
    fn default() -> Self {
        Self {
            raw: unsafe { lvgl_sys::lv_obj_create(ptr::null_mut(), ptr::null_mut()) },
        }
    }
}

macro_rules! define_object {
    ($item:ident) => {
        define_object!($item, event = (), part = $crate::support::ObjPart);
    };
    ($item:ident, event = $event_type:ty) => {
        define_object!($item, event = $event_type, part = $crate::support::ObjPart);
    };
    ($item:ident, part = $part_type:ty) => {
        define_object!($item, event = (), part = $part_type);
    };
    ($item:ident, part = $part_type:ty, event = $event_type:ty) => {
        define_object!($item, event = $event_type, part = $part_type);
    };
    ($item:ident, event = $event_type:ty, part = $part_type:ty) => {
        pub struct $item {
            core: $crate::support::GenericObject,
        }

        impl $item {
            pub fn on_event<F, S>(&mut self, f: F)
            where
                F: FnMut(
                    Self,
                    $crate::support::Event<<Self as $crate::support::Object>::SpecialEvent>,
                ),
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

        impl $crate::support::NativeObject for $item {
            fn raw(&self) -> core::ptr::NonNull<lvgl_sys::lv_obj_t> {
                self.core.raw()
            }
        }

        impl $crate::support::Object for $item {
            type SpecialEvent = $event_type;
            type Part = $part_type;

            unsafe fn from_raw(raw_pointer: core::ptr::NonNull<lvgl_sys::lv_obj_t>) -> Self {
                Self {
                    core: $crate::support::GenericObject::from_raw(raw_pointer),
                }
            }
        }
    };
}

#[derive(Clone)]
pub struct Color {
    pub(crate) raw: lvgl_sys::lv_color_t,
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

/// Events are triggered in LittlevGL when something happens which might be interesting to
/// the user, e.g. if an object:
///  - is clicked
///  - is dragged
///  - its value has changed, etc.
///
/// All objects (such as Buttons/Labels/Sliders etc.) receive these generic events
/// regardless of their type.
pub enum Event<T> {
    /// The object has been pressed
    Pressed,

    /// The object is being pressed (sent continuously while pressing)
    Pressing,

    /// The input device is still being pressed but is no longer on the object
    PressLost,

    /// Released before `long_press_time` config time. Not called if dragged.
    ShortClicked,

    /// Called on release if not dragged (regardless to long press)
    Clicked,

    /// Pressing for `long_press_time` config time. Not called if dragged.
    LongPressed,

    /// Called after `long_press_time` config in every `long_press_rep_time` ms. Not
    /// called if dragged.
    LongPressedRepeat,

    /// Called in every case when the object has been released even if it was dragged. Not called
    /// if slid from the object while pressing and released outside of the object. In this
    /// case, `Event<_>::PressLost` is sent.
    Released,

    /// Pointer-like input devices events (E.g. mouse or touchpad)
    Pointer(PointerEvent),

    /// Special event for the object type
    Special(T),
}

impl<S> TryFrom<lvgl_sys::lv_event_t> for Event<S> {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as u32 {
            lvgl_sys::LV_EVENT_PRESSED => Ok(Event::Pressed),
            lvgl_sys::LV_EVENT_PRESSING => Ok(Event::Pressing),
            lvgl_sys::LV_EVENT_PRESS_LOST => Ok(Event::PressLost),
            lvgl_sys::LV_EVENT_SHORT_CLICKED => Ok(Event::ShortClicked),
            lvgl_sys::LV_EVENT_CLICKED => Ok(Event::Clicked),
            lvgl_sys::LV_EVENT_LONG_PRESSED => Ok(Event::LongPressed),
            lvgl_sys::LV_EVENT_LONG_PRESSED_REPEAT => Ok(Event::LongPressedRepeat),
            lvgl_sys::LV_EVENT_RELEASED => Ok(Event::Released),
            _ => Err(()),
            // _ => {
            //     if let Ok(special_event_type) = S::try_from(value) {
            //         Ok(Event::Special(special_event_type))
            //     } else {
            //         Err(())
            //     }
            // }
        }
    }
}

impl<S> From<Event<S>> for lvgl_sys::lv_event_t {
    fn from(event: Event<S>) -> Self {
        let native_event = match event {
            Event::Pressed => lvgl_sys::LV_EVENT_PRESSED,
            Event::Pressing => lvgl_sys::LV_EVENT_PRESSING,
            Event::PressLost => lvgl_sys::LV_EVENT_PRESS_LOST,
            Event::ShortClicked => lvgl_sys::LV_EVENT_SHORT_CLICKED,
            Event::Clicked => lvgl_sys::LV_EVENT_CLICKED,
            Event::LongPressed => lvgl_sys::LV_EVENT_LONG_PRESSED,
            Event::LongPressedRepeat => lvgl_sys::LV_EVENT_LONG_PRESSED_REPEAT,
            Event::Released => lvgl_sys::LV_EVENT_RELEASED,
            // TODO: handle all types...
            _ => lvgl_sys::LV_EVENT_CLICKED,
        };
        native_event as lvgl_sys::lv_event_t
    }
}

/// These events are sent only by pointer-like input devices (E.g. mouse or touchpad)
pub enum PointerEvent {
    DragBegin,
    DragEnd,
    DragThrowBegin,
}

pub(crate) unsafe extern "C" fn event_callback<T, F>(
    obj: *mut lvgl_sys::lv_obj_t,
    event: lvgl_sys::lv_event_t,
) where
    T: Object + Sized,
    F: FnMut(T, Event<T::SpecialEvent>),
{
    // convert the lv_event_t to lvgl-rs Event type
    if let Ok(event) = event.try_into() {
        if let Some(obj_ptr) = NonNull::new(obj) {
            let object = T::from_raw(obj_ptr);
            // get the pointer from the Rust callback closure FnMut provided by users
            let user_closure = &mut *((*obj).user_data as *mut F);
            // call user callback closure
            user_closure(object, event);
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
