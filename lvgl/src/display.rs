use crate::objx::ObjectX;
use core::ptr;
use embedded_graphics;
use embedded_graphics::prelude::*;
use embedded_graphics::{drawable, DrawTarget};
use core::mem::MaybeUninit;
use embedded_graphics::pixelcolor::{PixelColor, raw::RawData, raw::RawU32};
use core::marker::PhantomData;


struct DisplayDriver<F, C>
    where
        F: FnMut(drawable::Pixel<C>),
        C: PixelColor
{
    pub raw: lvgl_sys::lv_disp_drv_t,
    callback: F,
    display_buffer: MaybeUninit<lvgl_sys::lv_disp_buf_t>,
    refresh_buffer: [MaybeUninit<lvgl_sys::lv_color_t>; lvgl_sys::LV_HOR_RES_MAX as usize * 10],
    phantom: PhantomData<C>,
}

impl<F, C> DisplayDriver<F, C> where
    F: FnMut(drawable::Pixel<C>),
    C: PixelColor {

    pub fn new<T>(mut device: T) -> Self
        where
        T: DrawTarget<C>
    {
        let mut callback = move |pixel: drawable::Pixel<C>| {
            let _ = device.draw_pixel(pixel);
        };

        // Create a display buffer for LittlevGL
        let mut display_buffer = MaybeUninit::<lvgl_sys::lv_disp_buf_t>::uninit();
        // Declare a buffer for 10 lines
        let mut refresh_buffer: [MaybeUninit<lvgl_sys::lv_color_t>;
            lvgl_sys::LV_HOR_RES_MAX as usize * 10] =
            unsafe { MaybeUninit::uninit().assume_init() };
        // Initialize the display buffer
        unsafe {
            lvgl_sys::lv_disp_buf_init(
                display_buffer.as_mut_ptr(),
                refresh_buffer.as_mut_ptr() as *mut cty::c_void,
                core::ptr::null_mut(),
                (lvgl_sys::LV_HOR_RES_MAX * 10) as u32,
            );
        }
        let mut disp_drv = unsafe {
            // Descriptor of a display driver
            let mut disp_drv = MaybeUninit::<lvgl_sys::lv_disp_drv_t>::uninit().assume_init();
            // Basic initialization
            lvgl_sys::lv_disp_drv_init(&mut disp_drv);
            // Set your driver function
            disp_drv.flush_cb = Some(display_callback_wrapper::<F, C>);
            disp_drv.user_data = &mut callback as *mut _ as *mut cty::c_void;
            disp_drv
        };
        // Assign the buffer to the display
        disp_drv.buffer = display_buffer.as_mut_ptr();
        // Finally register the driver
        unsafe {
            lvgl_sys::lv_disp_drv_register(&mut disp_drv);
        }
        Self {
            raw: disp_drv,
            callback,
            display_buffer,
            refresh_buffer,
            phantom: PhantomData,
        }
    }

    fn get_active_screen(&mut self) -> ObjectX<'static> {
        get_active_screen()
    }
}

unsafe extern "C" fn display_callback_wrapper<F, C>(
    disp_drv: *mut lvgl_sys::lv_disp_drv_t,
    area: *const lvgl_sys::lv_area_t,
    color_p: *mut lvgl_sys::lv_color_t,
) where
    F: FnMut(drawable::Pixel<C>),
    C: PixelColor
{
    // We need to make sure panics can't escape across the FFI boundary.
    //let _ = panic::catch_unwind(|| {
        let mut i = 0;
        let disp = *disp_drv;

        // Rust code closure reference
        let closure = &mut *(disp.user_data as *mut F);

        for y in (*area).y1..=(*area).y2 {
            for x in (*area).x1..=(*area).x2 {
                // Convert C color representation to high-level Rust
                let raw_color = *color_p.add(i);
                // let color = Rgb888::new(raw_color.ch.red,
                //                         raw_color.ch.green,
                //                         raw_color.ch.blue);
                let color = C::Raw::from_u32(raw_color.full);
                i = i + 1;
                // Callback the Rust closure to flush the new points to the screen
                closure(drawable::Pixel(Point::new(x as i32, y as i32), color));
            }
        }
        // Indicate to LittlevGL that you are ready with the flushing
        lvgl_sys::lv_disp_flush_ready(disp_drv);
    //}); // end of panic::catch_unwind
}

pub fn get_active_screen() -> ObjectX<'static> {
    let raw =
        unsafe { ptr::NonNull::new_unchecked(lvgl_sys::lv_disp_get_scr_act(ptr::null_mut())) };
    ObjectX::new(raw)
}
