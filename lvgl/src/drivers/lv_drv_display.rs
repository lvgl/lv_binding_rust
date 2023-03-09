#[macro_export]
macro_rules! lv_drv_disp_fbdev {
    ($draw_buffer:ident, $hor_res:ident, $ver_res:ident) => {
        unsafe {
            lvgl_sys::fbdev_init();
            lvgl::Display::register_raw(
                draw_buffer = $draw_buffer,
                hor_res = $hor_res,
                ver_res = $ver_res,
                flush_cb = lvgl_sys::fbdev_flush,
                rounder_cb = None,
                set_px_cb = None,
                clear_cb = None,
                monitor_cb = None,
                wait_cb = None,
                clean_dcache_cb = None,
                drv_update_cb = None,
                render_start_cb = None,
                drop = lvgl_sys::fbdev_exit,
            )
        }
    }
}

#[macro_export]
macro_rules! lv_drv_disp_drm {
    ($draw_buffer:ident, $hor_res:ident, $ver_res:ident) => {
        unsafe {
            lvgl_sys::drm_init();
            lvgl::Display::register_raw(
                draw_buffer = $draw_buffer,
                hor_res = $hor_res,
                ver_res = $ver_res,
                flush_cb = lvgl_sys::drm_flush,
                rounder_cb = None,
                set_px_cb = None,
                clear_cb = None,
                monitor_cb = None,
                wait_cb = lvgl_sys::drm_wait_vsync,
                clean_dcache_cb = None,
                drv_update_cb = None,
                render_start_cb = None,
                drop = lvgl_sys::drm_exit,
            )
        }
    }
}

#[macro_export]
macro_rules! lv_drv_disp_gtk {
    ($draw_buffer:ident, $hor_res:ident, $ver_res:ident) => {
        unsafe {
            lvgl_sys::gtkdrv_init();
            lvgl::Display::register_raw(
                draw_buffer = $draw_buffer,
                hor_res = $hor_res,
                ver_res = $ver_res,
                flush_cb = lvgl_sys::gtkdrv_flush_cb,
                rounder_cb = None,
                set_px_cb = None,
                clear_cb = None,
                monitor_cb = None,
                wait_cb = None,
                clean_dcache_cb = None,
                drv_update_cb = None,
                render_start_cb = None,
                drop = None,
            )
        }
    }
}

#[macro_export]
macro_rules! lv_drv_disp_sdl {
    ($draw_buffer:ident, $hor_res:ident, $ver_res:ident) => {
        unsafe {
            lvgl_sys::sdl_init();
            lvgl::Display::register_raw(
                draw_buffer = $draw_buffer,
                hor_res = $hor_res,
                ver_res = $ver_res,
                flush_cb = lvgl_sys::sdl_display_flush,
                rounder_cb = None,
                set_px_cb = None,
                clear_cb = None,
                monitor_cb = None,
                wait_cb = None,
                clean_dcache_cb = None,
                drv_update_cb = None,
                render_start_cb = None,
                drop = None,
            )
        }
    }
}

#[macro_export]
macro_rules! lv_drv_disp_gc9a01 {
    ($draw_buffer:ident, $hor_res:ident, $ver_res:ident) => {
        unsafe {
            match lvgl_sys::GC9A01_init() {
                0 => (),
                c = panic!("GC9A01_init() returned error code {c}")
            };
            lvgl::Display::register_raw(
                draw_buffer = $draw_buffer,
                hor_res = $hor_res,
                ver_res = $ver_res,
                flush_cb = lvgl_sys::GC9A01_flush,
                rounder_cb = None,
                set_px_cb = None,
                clear_cb = None,
                monitor_cb = None,
                wait_cb = None,
                clean_dcache_cb = None,
                drv_update_cb = None,
                render_start_cb = None,
                drop = None,
            )
        }
    }
}

#[macro_export]
macro_rules! lv_drv_disp_ili9341 {
    ($draw_buffer:ident, $hor_res:ident, $ver_res:ident) => {
        unsafe {
            lvgl_sys::ili9341_init();
            lvgl::Display::register_raw(
                draw_buffer = $draw_buffer,
                hor_res = $hor_res,
                ver_res = $ver_res,
                flush_cb = lvgl_sys::ili9341_flush,
                rounder_cb = None,
                set_px_cb = None,
                clear_cb = None,
                monitor_cb = None,
                wait_cb = None,
                clean_dcache_cb = None,
                drv_update_cb = None,
                render_start_cb = None,
                drop = None,
            )
        }
    }
}

#[macro_export]
macro_rules! lv_drv_disp_r61581 {
    ($draw_buffer:ident, $hor_res:ident, $ver_res:ident) => {
        unsafe {
            lvgl_sys::r61581_init();
            lvgl::Display::register_raw(
                draw_buffer = $draw_buffer,
                hor_res = $hor_res,
                ver_res = $ver_res,
                flush_cb = lvgl_sys::r61581_flush,
                rounder_cb = None,
                set_px_cb = None,
                clear_cb = None,
                monitor_cb = None,
                wait_cb = None,
                clean_dcache_cb = None,
                drv_update_cb = None,
                render_start_cb = None,
                drop = None,
            )
        }
    }
}

#[macro_export]
macro_rules! lv_drv_disp_sharp_mip {
    ($draw_buffer:ident, $hor_res:ident, $ver_res:ident) => {
        unsafe {
            lvgl_sys::sharp_mip_init();
            lvgl::Display::register_raw(
                draw_buffer = $draw_buffer,
                hor_res = $hor_res,
                ver_res = $ver_res,
                flush_cb = lvgl_sys::sharp_mip_flush,
                rounder_cb = lvgl_sys::sharp_mip_rounder,
                set_px_cb = lvgl_sys::sharp_mip_set_px,
                clear_cb = None,
                monitor_cb = None,
                wait_cb = None,
                clean_dcache_cb = None,
                drv_update_cb = None,
                render_start_cb = None,
                drop = None,
            )
        }
    }
}

#[macro_export]
macro_rules! lv_drv_disp_ssd1963 {
    ($draw_buffer:ident, $hor_res:ident, $ver_res:ident) => {
        unsafe {
            lvgl_sys::ssd1963_init();
            lvgl::Display::register_raw(
                draw_buffer = $draw_buffer,
                hor_res = $hor_res,
                ver_res = $ver_res,
                flush_cb = lvgl_sys::ssd1963_flush,
                rounder_cb = None,
                set_px_cb = None,
                clear_cb = None,
                monitor_cb = None,
                wait_cb = None,
                clean_dcache_cb = None,
                drv_update_cb = None,
                render_start_cb = None,
                drop = None,
            )
        }
    }
}

#[macro_export]
macro_rules! lv_drv_disp_st7565 {
    ($draw_buffer:ident, $hor_res:ident, $ver_res:ident) => {
        unsafe {
            lvgl_sys::st7565_init();
            lvgl::Display::register_raw(
                draw_buffer = $draw_buffer,
                hor_res = $hor_res,
                ver_res = $ver_res,
                flush_cb = lvgl_sys::st7565_flush,
                rounder_cb = None,
                set_px_cb = None,
                clear_cb = None,
                monitor_cb = None,
                wait_cb = None,
                clean_dcache_cb = None,
                drv_update_cb = None,
                render_start_cb = None,
                drop = None,
            )
        }
    }
}

#[macro_export]
macro_rules! lv_drv_disp_uc1610 {
    ($draw_buffer:ident, $hor_res:ident, $ver_res:ident) => {
        unsafe {
            lvgl_sys::uc1610_init();
            lvgl::Display::register_raw(
                draw_buffer = $draw_buffer,
                hor_res = $hor_res,
                ver_res = $ver_res,
                flush_cb = lvgl_sys::uc1610_flush_cb,
                rounder_cb = lvgl_sys::uc1610_rounder_cb,
                set_px_cb = lvgl_sys::uc1610_set_px_cb,
                clear_cb = None,
                monitor_cb = None,
                wait_cb = None,
                clean_dcache_cb = None,
                drv_update_cb = None,
                render_start_cb = None,
                drop = None,
            )
        }
    }
}
