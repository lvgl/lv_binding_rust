use crate::{DisplayDriver, ObjectX};
use alloc::boxed::Box;
use core::marker::PhantomData;
use core::ptr;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, Ordering};
use core::time::Duration;

// There can only be a single reference to LittlevGL library.
static LVGL_IN_USE: AtomicBool = AtomicBool::new(false);

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum LvError {
    Uninitialized,
    AlreadyInUse,
}

type LvResult<T> = Result<T, LvError>;

pub struct UI {
    // LittlevGL is not thread-safe by default.
    _not_sync: PhantomData<*mut ()>,
}

// LittlevGL does not use thread locals.
unsafe impl Send for UI {}

impl UI {
    pub fn init() -> Result<Self, LvError> {
        if LVGL_IN_USE.compare_and_swap(false, true, Ordering::SeqCst) == false {
            unsafe {
                lvgl_sys::lv_init();
            }
            Ok(Self {
                _not_sync: PhantomData,
            })
        } else {
            Err(LvError::AlreadyInUse)
        }
    }

    pub fn disp_drv_register(&mut self, display: DisplayDriver) {
        // Throw display driver into a box and add to user data (if we need to get the display back)
        // or simply forget the display pointer/object to prevent Drop to be called
        // register it
        unsafe {
            let boxed = Box::new(display.raw);
            lvgl_sys::lv_disp_drv_register(Box::into_raw(boxed));
        }
    }

    pub fn scr_act(&self) -> ObjectX {
        unsafe {
            let screen = lvgl_sys::lv_disp_get_scr_act(ptr::null_mut());
            ObjectX::from_raw(NonNull::new_unchecked(screen))
        }
    }

    pub fn tick_inc(&mut self, tick_period: Duration) {
        unsafe {
            lvgl_sys::lv_tick_inc(tick_period.as_millis() as u32);
        }
    }

    pub fn task_handler(&mut self) {
        unsafe {
            lvgl_sys::lv_task_handler();
        }
    }
}
