use crate::{Display, DrawBuffer};

macro_rules! lv_drv_fbdev {
    ($draw_buffer:ident, $hor_res:ident, $ver_res:ident) => {
        unsafe {
            lvgl_sys::fbdev_init();
            Display::register_raw(
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
    };
}
/*
fn fbdev<const N: usize>(buffer: DrawBuffer<N>) {
    //let disp = Display::register_raw(draw_buffer, hor_res, ver_res, flush_cb, rounder_cb, set_px_cb, clear_cb, monitor_cb, wait_cb, clean_dcache_cb, drv_update_cb, render_start_cb)

}
*/
