#[macro_export]
macro_rules! lv_drv_input_pointer_evdev {
    () => {
        unsafe {
            lvgl_sys::evdev_init();
            lvgl::input_device::pointer::PointerPointer::new_raw(
                read_cb = lvgl_sys::evdev_read,
                feedback_cb = None,
            )
        }
    }
}

#[macro_export]
macro_rules! lv_drv_input_pointer_gtk {
    () => {
        unsafe {
            lvgl::input_device::pointer::Pointer::new_raw(
                read_cb = lvgl_sys::gtkdrv_mouse_read_cb,
                feedback_cb = None,
            )
        }
    }
}

#[macro_export]
macro_rules! lv_drv_input_pointer_sdl {
    () => {
        unsafe {
            lvgl::input_device::pointer::Pointer::new_raw(
                read_cb = lvgl_sys::sdl_mouse_read,
                feedback_cb = None,
            )
        }
    }
}

#[macro_export]
macro_rules! lv_drv_input_ad_touch {
    () => {
        unsafe {
            lvgl_sys::ad_touch_init();
            lvgl::input_device::pointer::Pointer::new_raw(
                read_cb = lvgl_sys::ad_touch_read,
                feedback_cb = None,
            )
        }
    }
}

#[macro_export]
macro_rules! lv_drv_input_ft5406ee8 {
    () => {
        unsafe {
            lvgl_sys::ft5406ee8_init();
            lvgl::input_device::pointer::Pointer::new_raw(
                read_cb = lvgl_sys::ft5406ee8_read,
                feedback_cb = None,
            )
        }
    }
}

#[macro_export]
macro_rules! lv_drv_input_xpt2046 {
    () => {
        unsafe {
            lvgl_sys::xpt2046_init();
            lvgl::input_device::pointer::Pointer::new_raw(
                read_cb = lvgl_sys::xpt2046_read,
                feedback_cb = None,
            )
        }
    }
}
