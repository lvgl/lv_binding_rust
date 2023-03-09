use crate::input_device::generic::InputDriver;
use crate::input_device::pointer::Pointer;

macro_rules! lv_drv_input_pointer_evdev {
    () => {
        unsafe {
            lvgl_sys::evdev_init();
            Pointer::new_raw(
                read_cb = lvgl_sys::evdev_read,
                feedback_cb = None,
            )
        }
    }
}

macro_rules! lv_drv_input_pointer_gtk {
    () => {
        unsafe {
            Pointer::new_raw(
                read_cb = lvgl_sys::gtkdrv_mouse_read_cb,
                feedback_cb = None,
            )
        }
    }
}

macro_rules! lv_drv_input_pointer_sdl {
    () => {
        unsafe {
            Pointer::new_raw(
                read_cb = lvgl_sys::sdl_mouse_read,
                feedback_cb = None,
            )
        }
    }
}

macro_rules! lv_drv_input_ad_touch {
    () => {
        unsafe {
            lvgl_sys::ad_touch_init();
            Pointer::new_raw(
                read_cb = lvgl_sys::ad_touch_read,
                feedback_cb = None,
            )
        }
    }
}

macro_rules! lv_drv_input_ft5406ee8 {
    () => {
        unsafe {
            lvgl_sys::ft5406ee8_init();
            Pointer::new_raw(
                read_cb = lvgl_sys::ft5406ee8_read,
                feedback_cb = None,
            )
        }
    }
}

macro_rules! lv_drv_input_xpt2046 {
    () => {
        unsafe {
            lvgl_sys::xpt2046_init();
            Pointer::new_raw(
                read_cb = lvgl_sys::xpt2046_read,
                feedback_cb = None,
            )
        }
    }
}
