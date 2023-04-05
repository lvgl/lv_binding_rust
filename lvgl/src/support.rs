use crate::display::DisplayError;
use crate::Widget;
use core::convert::{TryFrom, TryInto};
use core::ptr::NonNull;

#[cfg(feature = "embedded_graphics")]
use embedded_graphics::pixelcolor::{Rgb565, Rgb888};

pub type LvResult<T> = Result<T, LvError>;

/// Generic LVGL error. All other errors can be coerced into it.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum LvError {
    InvalidReference,
    Uninitialized,
    LvOOMemory,
    AlreadyInUse,
}

impl From<DisplayError> for LvError {
    fn from(err: DisplayError) -> Self {
        use LvError::*;
        match err {
            DisplayError::NotAvailable => Uninitialized,
            DisplayError::FailedToRegister => InvalidReference,
            DisplayError::NotRegistered => Uninitialized,
        }
    }
}

impl From<LvError> for DisplayError {
    fn from(err: LvError) -> Self {
        use DisplayError::*;
        match err {
            LvError::InvalidReference => FailedToRegister,
            LvError::Uninitialized => NotAvailable,
            LvError::LvOOMemory => FailedToRegister,
            LvError::AlreadyInUse => FailedToRegister,
        }
    }
}

/// An LVGL color. Equivalent to `lv_color_t`.
#[derive(Copy, Clone, Default)]
pub struct Color {
    pub(crate) raw: lvgl_sys::lv_color_t,
}

impl Color {
    /// Creates a `Color` from red, green, and blue values.
    pub fn from_rgb((r, g, b): (u8, u8, u8)) -> Self {
        let raw = unsafe { lvgl_sys::_LV_COLOR_MAKE(r, g, b) };
        Self { raw }
    }
    /// Creates a `Color` from a native `lv_color_t` instance.
    pub fn from_raw(raw: lvgl_sys::lv_color_t) -> Self {
        Self { raw }
    }
    /// Returns the value of the red channel.
    pub fn r(&self) -> u8 {
        unsafe { lvgl_sys::_LV_COLOR_GET_R(self.raw) as u8 }
    }
    /// Returns the value of the green channel.
    pub fn g(&self) -> u8 {
        unsafe { lvgl_sys::_LV_COLOR_GET_G(self.raw) as u8 }
    }
    /// Returns the value of the blue channel.
    pub fn b(&self) -> u8 {
        unsafe { lvgl_sys::_LV_COLOR_GET_B(self.raw) as u8 }
    }
}

#[cfg(feature = "embedded_graphics")]
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

#[cfg(feature = "embedded_graphics")]
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

impl From<Color> for lvgl_sys::lv_color_t {
    fn from(val: Color) -> Self {
        val.raw
    }
}

/// Events are triggered in LVGL when something happens which might be interesting to
/// the user, e.g. if an object:
///  - is clicked
///  - is dragged
///  - its value has changed, etc.
///
/// All objects (such as Buttons/Labels/Sliders etc.) receive these generic events
/// regardless of their type.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
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

impl<S> TryFrom<lvgl_sys::lv_event_code_t> for Event<S> {
    type Error = ();

    fn try_from(value: lvgl_sys::lv_event_code_t) -> Result<Self, Self::Error> {
        const LV_EVENT_PRESSED: u32 = lvgl_sys::lv_event_code_t_LV_EVENT_PRESSED;
        const LV_EVENT_PRESSING: u32 = lvgl_sys::lv_event_code_t_LV_EVENT_PRESSING;
        const LV_EVENT_PRESS_LOST: u32 = lvgl_sys::lv_event_code_t_LV_EVENT_PRESS_LOST;
        const LV_EVENT_SHORT_CLICKED: u32 = lvgl_sys::lv_event_code_t_LV_EVENT_SHORT_CLICKED;
        const LV_EVENT_CLICKED: u32 = lvgl_sys::lv_event_code_t_LV_EVENT_CLICKED;
        const LV_EVENT_LONG_PRESSED: u32 = lvgl_sys::lv_event_code_t_LV_EVENT_LONG_PRESSED;
        const LV_EVENT_LONG_PRESSED_REPEAT: u32 =
            lvgl_sys::lv_event_code_t_LV_EVENT_LONG_PRESSED_REPEAT;
        const LV_EVENT_RELEASED: u32 = lvgl_sys::lv_event_code_t_LV_EVENT_RELEASED;

        match value {
            LV_EVENT_PRESSED => Ok(Event::Pressed),
            LV_EVENT_PRESSING => Ok(Event::Pressing),
            LV_EVENT_PRESS_LOST => Ok(Event::PressLost),
            LV_EVENT_SHORT_CLICKED => Ok(Event::ShortClicked),
            LV_EVENT_CLICKED => Ok(Event::Clicked),
            LV_EVENT_LONG_PRESSED => Ok(Event::LongPressed),
            LV_EVENT_LONG_PRESSED_REPEAT => Ok(Event::LongPressedRepeat),
            LV_EVENT_RELEASED => Ok(Event::Released),
            _ => Err(()),
        }
    }
}

impl<S> From<Event<S>> for lvgl_sys::lv_event_code_t {
    fn from(event: Event<S>) -> Self {
        let native_event = match event {
            Event::Pressed => lvgl_sys::lv_event_code_t_LV_EVENT_PRESSED,
            Event::Pressing => lvgl_sys::lv_event_code_t_LV_EVENT_PRESSING,
            Event::PressLost => lvgl_sys::lv_event_code_t_LV_EVENT_PRESS_LOST,
            Event::ShortClicked => lvgl_sys::lv_event_code_t_LV_EVENT_SHORT_CLICKED,
            Event::Clicked => lvgl_sys::lv_event_code_t_LV_EVENT_CLICKED,
            Event::LongPressed => lvgl_sys::lv_event_code_t_LV_EVENT_LONG_PRESSED,
            Event::LongPressedRepeat => lvgl_sys::lv_event_code_t_LV_EVENT_LONG_PRESSED_REPEAT,
            Event::Released => lvgl_sys::lv_event_code_t_LV_EVENT_RELEASED,
            // TODO: handle all types...
            _ => lvgl_sys::lv_event_code_t_LV_EVENT_CLICKED,
        };
        native_event as lvgl_sys::lv_event_code_t
    }
}

/// Events sent only by pointer-like input devices (e.g. mouse or touchpad)
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum PointerEvent {
    DragBegin,
    DragEnd,
    DragThrowBegin,
}

pub(crate) unsafe extern "C" fn event_callback<T, F>(event: *mut lvgl_sys::lv_event_t)
where
    T: Widget + Sized,
    F: FnMut(T, Event<T::SpecialEvent>),
{
    let code = (*event).code;
    let obj = (*event).target;
    // convert the lv_event_code_t to lvgl-rs Event type
    if let Ok(code) = code.try_into() {
        if let Some(obj_ptr) = NonNull::new(obj) {
            let object = T::from_raw(obj_ptr).unwrap();
            // get the pointer from the Rust callback closure FnMut provided by users
            let user_closure = &mut *((*obj).user_data as *mut F);
            // call user callback closure
            user_closure(object, code);
        }
    }
}

/// Possible LVGL alignments for widgets.
pub enum Align {
    Center,
    TopLeft,
    TopMid,
    TopRight,
    BottomLeft,
    BottomMid,
    BottomRight,
    LeftMid,
    RightMid,
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

impl From<Align> for u8 {
    fn from(value: Align) -> u8 {
        let native = match value {
            Align::Center => lvgl_sys::LV_ALIGN_CENTER,
            Align::TopLeft => lvgl_sys::LV_ALIGN_TOP_LEFT,
            Align::TopMid => lvgl_sys::LV_ALIGN_TOP_MID,
            Align::TopRight => lvgl_sys::LV_ALIGN_TOP_RIGHT,
            Align::BottomLeft => lvgl_sys::LV_ALIGN_BOTTOM_LEFT,
            Align::BottomMid => lvgl_sys::LV_ALIGN_BOTTOM_MID,
            Align::BottomRight => lvgl_sys::LV_ALIGN_BOTTOM_RIGHT,
            Align::LeftMid => lvgl_sys::LV_ALIGN_LEFT_MID,
            Align::RightMid => lvgl_sys::LV_ALIGN_RIGHT_MID,
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
        };
        native as u8
    }
}

pub enum TextAlign {
    Auto,
    Center,
    Left,
    Right,
}

impl From<TextAlign> for u8 {
    fn from(value: TextAlign) -> Self {
        let native = match value {
            TextAlign::Auto => lvgl_sys::LV_TEXT_ALIGN_AUTO,
            TextAlign::Center => lvgl_sys::LV_TEXT_ALIGN_CENTER,
            TextAlign::Left => lvgl_sys::LV_TEXT_ALIGN_LEFT,
            TextAlign::Right => lvgl_sys::LV_TEXT_ALIGN_RIGHT,
        };
        native as u8
    }
}

/// Boolean for determining whether animations are enabled.
pub enum Animation {
    ON,
    OFF,
}

impl From<Animation> for lvgl_sys::lv_anim_enable_t {
    fn from(anim: Animation) -> Self {
        match anim {
            Animation::ON => lvgl_sys::lv_anim_enable_t_LV_ANIM_ON,
            Animation::OFF => lvgl_sys::lv_anim_enable_t_LV_ANIM_OFF,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn color_properties_accessible() {
        let color = Color::from_rgb((206, 51, 255));

        if lvgl_sys::LV_COLOR_DEPTH == 32 {
            assert_eq!(color.r(), 206);
            assert_eq!(color.g(), 51);
            assert_eq!(color.b(), 255);
        } else if lvgl_sys::LV_COLOR_DEPTH == 16 {
            assert_eq!(color.r(), 25);
            assert_eq!(color.g(), 12);
            assert_eq!(color.b(), 31);
        }
    }
}
