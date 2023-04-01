use super::pointer::*;
use crate::LvResult;

/// Generic data which can be associated with an input device driver. Varies
/// based on the concrete type of the input device driver
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Data {
    Pointer(PointerInputData),
}

/// Boolean states for an input.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum InputState {
    Released(Data),
    Pressed(Data),
}

impl InputState {
    /// Represents a non-buffered input device.
    pub fn once(self) -> BufferStatus {
        BufferStatus::Once(self)
    }
    /// Represents a buffered input device.
    pub fn and_continued(self) -> BufferStatus {
        BufferStatus::Buffered(self)
    }
}

/// Boolean buffering states for an input device driver.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum BufferStatus {
    Once(InputState),
    Buffered(InputState),
}

/// A generic input driver trait.
pub trait InputDriver<D> {
    fn register<F>(handler: F, display: &crate::Display) -> LvResult<D>
    where
        F: Fn() -> BufferStatus;

    fn get_driver(&mut self) -> &mut lvgl_sys::lv_indev_drv_t;

    /// Creates a new `InputDriver` from raw parts.
    /// 
    /// # Safety
    /// 
    /// The provided functions must not themselves cause undefined behavior
    /// when called by LVGL.
    unsafe fn new_raw(
        read_cb: Option<
            unsafe extern "C" fn(*mut lvgl_sys::lv_indev_drv_t, *mut lvgl_sys::lv_indev_data_t),
        >,
        feedback_cb: Option<unsafe extern "C" fn(*mut lvgl_sys::lv_indev_drv_t, u8)>,
        display: &crate::Display,
    ) -> LvResult<D>;

    /// Sets the descriptor for the input driver wrapper.
    /// 
    /// # Safety
    /// 
    /// `descriptor` must point to an initialized but unregistered and unused
    /// instance of an `lv_indev_t`, and must also be aligned.
    unsafe fn set_descriptor(&mut self, descriptor: *mut lvgl_sys::lv_indev_t) -> LvResult<()>;
}
