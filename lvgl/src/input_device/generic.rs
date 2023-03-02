use super::pointer::*;
use crate::LvResult;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Data {
    Pointer(PointerInputData),
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum InputState {
    Released(Data),
    Pressed(Data),
}

impl InputState {
    pub fn once(self) -> BufferStatus {
        BufferStatus::Once(self)
    }

    pub fn and_continued(self) -> BufferStatus {
        BufferStatus::Buffered(self)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum BufferStatus {
    Once(InputState),
    Buffered(InputState),
}

pub trait InputDriver<D> {
    fn new<F>(handler: F) -> D
    where
        F: Fn() -> BufferStatus;

    fn get_driver(&self) -> lvgl_sys::lv_indev_drv_t;
    unsafe fn set_descriptor(&mut self, descriptor: *mut lvgl_sys::lv_indev_t) -> LvResult<()>;
}
