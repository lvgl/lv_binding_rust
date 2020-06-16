use crate::mem::Box;
use crate::{Color, Event, LvError, LvResult, Obj, Widget};
use core::marker::PhantomData;
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ptr;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, Ordering};
use core::time::Duration;
use embedded_graphics::pixelcolor::PixelColor;
use embedded_graphics::prelude::*;
use embedded_graphics::{drawable, DrawTarget};

// There can only be a single reference to LittlevGL library.
static LVGL_IN_USE: AtomicBool = AtomicBool::new(false);

// TODO: Make this an external configuration
const REFRESH_BUFFER_LEN: usize = 2;
// Declare a buffer for the refresh rate
const BUF_SIZE: usize = lvgl_sys::LV_HOR_RES_MAX as usize * REFRESH_BUFFER_LEN;

pub struct UI<T, C>
where
    T: DrawTarget<C>,
    C: PixelColor + From<Color>,
{
    // LittlevGL is not thread-safe by default.
    _not_sync: PhantomData<*mut ()>,
    // Later we can add possibility to have multiple displays by using `heapless::Vec`
    display_data: Option<DisplayUserData<T, C>>,
}

// LittlevGL does not use thread locals.
unsafe impl<T, C> Send for UI<T, C>
where
    T: DrawTarget<C>,
    C: PixelColor + From<Color>,
{
}

impl<T, C> UI<T, C>
where
    T: DrawTarget<C>,
    C: PixelColor + From<Color>,
{
    pub fn init() -> LvResult<Self> {
        if !LVGL_IN_USE.compare_and_swap(false, true, Ordering::SeqCst) {
            unsafe {
                lvgl_sys::lv_init();
            }
            Ok(Self {
                _not_sync: PhantomData,
                display_data: None,
            })
        } else {
            Err(LvError::AlreadyInUse)
        }
    }

    pub fn disp_drv_register(&mut self, display: T) -> LvResult<()> {
        self.display_data = Some(DisplayUserData {
            display,
            phantom: PhantomData,
        });

        unsafe {
            // Create a display buffer for LittlevGL
            // Initialize the display buffer
            let refresh_buffer1 = [Color::from_rgb((0, 0, 0)).raw; BUF_SIZE];
            let refresh_buffer2 = [Color::from_rgb((0, 0, 0)).raw; BUF_SIZE];

            let mut disp_buf = MaybeUninit::<lvgl_sys::lv_disp_buf_t>::uninit();

            lvgl_sys::lv_disp_buf_init(
                disp_buf.as_mut_ptr(),
                Box::into_raw(Box::new(refresh_buffer1)?) as *mut cty::c_void,
                Box::into_raw(Box::new(refresh_buffer2)?) as *mut cty::c_void,
                lvgl_sys::LV_HOR_RES_MAX * REFRESH_BUFFER_LEN as u32,
            );
            let disp_buf = Box::new(disp_buf.assume_init())?;

            // Basic initialization of the display driver
            let mut disp_drv = MaybeUninit::<lvgl_sys::lv_disp_drv_t>::uninit();
            lvgl_sys::lv_disp_drv_init(disp_drv.as_mut_ptr());
            // Since this is C managed memory, we don't want to drop it using Rust, thus `ManuallyDrop` wrapping.
            let mut disp_drv = ManuallyDrop::new(disp_drv.assume_init());
            // Assign the buffer to the display
            disp_drv.buffer = Box::into_raw(disp_buf);
            // Set your driver function
            disp_drv.flush_cb = Some(display_callback_wrapper::<T, C>);
            // TODO: DrawHandler type here
            // Safety: `user_data` is set to NULL in C code.
            disp_drv.user_data = &mut self.display_data as *mut _ as *mut cty::c_void;
            lvgl_sys::lv_disp_drv_register(
                &mut ManuallyDrop::take(&mut disp_drv) as *mut lvgl_sys::lv_disp_drv_t
            );
        };

        Ok(())
    }

    pub fn get_display_ref(&self) -> Option<&T> {
        match self.display_data.as_ref() {
            None => None,
            Some(v) => Some(&v.display),
        }
    }

    pub fn scr_act(&self) -> LvResult<Obj> {
        unsafe {
            let screen = lvgl_sys::lv_disp_get_scr_act(ptr::null_mut());
            if let Some(v) = NonNull::new(screen) {
                Ok(Obj::from_raw(v))
            } else {
                Err(LvError::InvalidReference)
            }
        }
    }

    pub fn event_send<W>(&mut self, obj: &mut W, event: Event<W::SpecialEvent>) -> LvResult<()>
    where
        W: Widget,
    {
        unsafe {
            lvgl_sys::lv_event_send(obj.raw()?.as_mut(), event.into(), ptr::null_mut());
        }
        Ok(())
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

pub(crate) struct DisplayUserData<T, C>
where
    T: DrawTarget<C>,
    C: PixelColor + From<Color>,
{
    display: T,
    phantom: PhantomData<C>,
}

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
        let user_data = &mut *(display_driver.user_data as *mut DisplayUserData<T, C>);
        let x1 = (*area).x1;
        let x2 = (*area).x2;
        let y1 = (*area).y1;
        let y2 = (*area).y2;
        // TODO: Can we do anything when there is a error while flushing?
        let _ = display_flush(&mut user_data.display, (x1, x2), (y1, y2), color_p);
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
