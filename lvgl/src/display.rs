use crate::Color;
use alloc::boxed::Box;
use core::mem::MaybeUninit;
use embedded_graphics::prelude::*;
use embedded_graphics::{drawable, DrawTarget};

pub struct DisplayDriver {
    pub(crate) raw: lvgl_sys::lv_disp_drv_t,
}

impl DisplayDriver {
    // we should accept a Rc<RefCell<T>> and throw it in a box and add to the user_data of the callback handler function
    pub fn new<T, C>(device: &mut T) -> Self
    where
        T: DrawTarget<C>,
        C: PixelColor + From<Color>,
    {
        let disp_drv = unsafe {
            // Create a display buffer for LittlevGL
            let mut display_buffer = MaybeUninit::<lvgl_sys::lv_disp_buf_t>::uninit();

            // Declare a buffer for the refresh rate
            // TODO: Make this an external configuration
            const REFRESH_BUFFER_LEN: usize = 2;
            let refresh_buffer1 = Box::new(
                MaybeUninit::<
                    [MaybeUninit<lvgl_sys::lv_color_t>;
                        lvgl_sys::LV_HOR_RES_MAX as usize * REFRESH_BUFFER_LEN],
                >::uninit()
                .assume_init(),
            );
            let refresh_buffer2 = Box::new(
                MaybeUninit::<
                    [MaybeUninit<lvgl_sys::lv_color_t>;
                        lvgl_sys::LV_HOR_RES_MAX as usize * REFRESH_BUFFER_LEN],
                >::uninit()
                .assume_init(),
            );

            // Initialize the display buffer
            lvgl_sys::lv_disp_buf_init(
                display_buffer.as_mut_ptr(),
                Box::into_raw(refresh_buffer1) as *mut cty::c_void,
                Box::into_raw(refresh_buffer2) as *mut cty::c_void,
                lvgl_sys::LV_HOR_RES_MAX * REFRESH_BUFFER_LEN as u32,
            );
            let display_buffer = Box::new(display_buffer.assume_init());

            // Basic initialization of the display driver
            let mut disp_drv = MaybeUninit::<lvgl_sys::lv_disp_drv_t>::uninit();
            lvgl_sys::lv_disp_drv_init(disp_drv.as_mut_ptr());
            let mut disp_drv = disp_drv.assume_init();
            // Assign the buffer to the display
            disp_drv.buffer = Box::into_raw(display_buffer);
            // Set your driver function
            disp_drv.flush_cb = Some(display_callback_wrapper::<T, C>);
            // TODO: DrawHandler type here
            disp_drv.user_data = device as *mut _ as *mut cty::c_void;
            disp_drv
        };
        Self { raw: disp_drv }
    }
}

// We need to keep a reference to the DisplayDriver in UI if we implement Drop
// impl Drop for DisplayDriver {
//     fn drop(&mut self) {
//         // grab the user data and deref the DrawHandler to free the instance for dealloc in the Rust universe.
//         unimplemented!()
//     }
// }

// a reference is kept to the external drawing target (T)
// the reference is kept in the callback function of the drawing handler
// we need a reference counter for the drawing target and free the ref counter when the display is
// destroyed.
//type DrawHandler = Rc<RefCell<u8>>;
//
// impl Drop for DrawHandler {
//     fn drop(&mut self) {
//         unimplemented!()
//     }
// }

unsafe extern "C" fn display_callback_wrapper<T, C>(
    disp_drv: *mut lvgl_sys::lv_disp_drv_t,
    area: *const lvgl_sys::lv_area_t,
    color_p: *mut lvgl_sys::lv_color_t,
) where
    T: DrawTarget<C>,
    C: PixelColor + From<Color>,
{
    // In the `std` world we would make sure to capture panics here and make them not escape across
    // the FFI boundary. Since this library is focused on embedded platforms, we don't
    // have an standard unwinding mechanism to rely upon.
    let display_driver = *disp_drv;
    // Rust code closure reference
    if !display_driver.user_data.is_null() {
        let device = &mut *(display_driver.user_data as *mut T);
        let x1 = (*area).x1;
        let x2 = (*area).x2;
        let y1 = (*area).y1;
        let y2 = (*area).y2;
        // TODO: Can we do anything when there is a error while flushing?
        let _ = display_flush(device, (x1, x2), (y1, y2), color_p);
    }
    // Indicate to LittlevGL that we are ready with the flushing
    lvgl_sys::lv_disp_flush_ready(disp_drv);
}

// We separate this display flush function to reduce the amount of unsafe code we need to write.
// This also provides a good separation of concerns, what is necessary from LittlevGL to work and
// what is the lvgl-rs wrapper responsibility.
fn display_flush<T, C>(
    display: &mut T,
    (x1, x2): (i16, i16),
    (y1, y2): (i16, i16),
    color_p: *mut lvgl_sys::lv_color_t,
) -> Result<(), T::Error>
where
    T: DrawTarget<C>,
    C: PixelColor + From<Color>,
{
    let ys = y1..=y2;
    let xs = (x1..=x2).enumerate();
    let x_len = (x2 - x1 + 1) as usize;

    // We use iterators here to ensure that the Rust compiler can apply all possible
    // optimizations at compile time.
    let pixels = ys
        .enumerate()
        .map(|(iy, y)| {
            xs.clone().map(move |(ix, x)| {
                let color_len = x_len * iy + ix;
                let lv_color = unsafe { *color_p.add(color_len) };
                let raw_color = Color::from_raw(lv_color);
                drawable::Pixel(Point::new(x as i32, y as i32), raw_color.into())
            })
        })
        .flatten();

    Ok(display.draw_iter(pixels)?)
}
