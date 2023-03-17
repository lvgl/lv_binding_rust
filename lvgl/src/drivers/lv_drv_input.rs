#[macro_export]
macro_rules! lv_drv_input_pointer_evdev {
    () => {
        unsafe {
            lvgl_sys::evdev_init();
            $crate::input_device::pointer::PointerPointer::new_raw(
                Some(lvgl_sys::evdev_read),
                None,
            )
        }
    }
}

#[macro_export]
macro_rules! lv_drv_input_pointer_gtk {
    () => {
        unsafe {
            $crate::input_device::pointer::Pointer::new_raw(
                Some(lvgl_sys::gtkdrv_mouse_read_cb),
                None,
            )
        }
    }
}

#[macro_export]
macro_rules! lv_drv_input_pointer_sdl {
    () => {
        unsafe {
            $crate::input_device::pointer::Pointer::new_raw(
                Some(lvgl_sys::sdl_mouse_read),
                None,
            )
        }
    }
}

#[macro_export]
macro_rules! lv_drv_input_ad_touch {
    () => {
        unsafe {
            lvgl_sys::ad_touch_init();
            $crate::input_device::pointer::Pointer::new_raw(
                Some(lvgl_sys::ad_touch_read),
                None,
            )
        }
    }
}

#[macro_export]
macro_rules! lv_drv_input_ft5406ee8 {
    () => {
        unsafe {
            lvgl_sys::ft5406ee8_init();
            $crate::input_device::pointer::Pointer::new_raw(
                Some(lvgl_sys::ft5406ee8_read),
                None,
            )
        }
    }
}

#[macro_export]
macro_rules! lv_drv_input_xpt2046 {
    () => {
        unsafe {
            lvgl_sys::xpt2046_init();
            $crate::input_device::pointer::Pointer::new_raw(
                Some(lvgl_sys::xpt2046_read),
                None,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    #[macro_use]
    use super::*;
    use crate::*;
    use crate::tests;
    use crate::DrawBuffer;
    use crate::input_device::generic::InputDriver;

    #[test]
    fn gtk_test() {
        const HOR_RES: u32 = 240;
        const VER_RES: u32 = 240;
        tests::initialize_test();
        let buffer = DrawBuffer::<{ (HOR_RES * VER_RES) as usize }>::new();
        let _disp = lv_drv_disp_sdl!(buffer, HOR_RES, VER_RES).unwrap();
        let _input = lv_drv_input_pointer_sdl!();
    }
}