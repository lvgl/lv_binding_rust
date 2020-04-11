use lazy_static;
use lvgl_sys;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas, CanvasBuilder, Canvas};
use sdl2::video::Window;
use std::ffi::CString;
use std::mem::{MaybeUninit, ManuallyDrop};
use std::os::raw::c_void;
use std::time::Duration;
use std::ptr::NonNull;
use std::panic;
use std::sync::mpsc::channel;


fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let hr: u32 = unsafe { lvgl_sys::lv_disp_get_hor_res(std::ptr::null_mut()) as u32 };
    let vr: u32 = unsafe { lvgl_sys::lv_disp_get_ver_res(std::ptr::null_mut()) as u32 };

    let window = video_subsystem
        .window("TFT Display: Demo", hr, vr)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    unsafe {
        lvgl_sys::lv_init();
    }
    // Create a display buffer for LittlevGL
    let mut disp_buf = MaybeUninit::<lvgl_sys::lv_disp_buf_t>::uninit();
    let mut buf: [MaybeUninit<lvgl_sys::lv_color_t>; lvgl_sys::LV_HOR_RES_MAX as usize * 10] =
        unsafe { MaybeUninit::uninit().assume_init() }; /*Declare a buffer for 10 lines*/
    unsafe {
        // Initialize the display buffer
        lvgl_sys::lv_disp_buf_init(
            disp_buf.as_mut_ptr(),
            buf.as_mut_ptr() as *mut c_void,
            std::ptr::null_mut(),
            (hr * 10) as u32,
        );
    }

    // Implement and register a function which can copy a pixel array to an area of your display:
    let mut display_driver = DisplayDriver::new(move |points, colors| {
        for (i, point) in points.into_iter().enumerate() {
            canvas.set_draw_color(colors[i]);
            canvas.draw_point(point);
        }
        canvas.present();
    });

    display_driver.raw.buffer = disp_buf.as_mut_ptr(); // Assign the buffer to the display
    unsafe {
        lvgl_sys::lv_disp_drv_register(&mut display_driver.raw); // Finally register the driver
    }

    // Create screen and widgets
    let mut screen = unsafe { lvgl_sys::lv_disp_get_scr_act(std::ptr::null_mut()) };
    let mut btn = unsafe { lvgl_sys::lv_btn_create(screen, std::ptr::null_mut()) };
    unsafe {
        lvgl_sys::lv_obj_set_pos(btn, 10, 10);
        lvgl_sys::lv_obj_set_size(btn, 200, 50)
    }
    let mut label = unsafe { lvgl_sys::lv_label_create(btn, std::ptr::null_mut()) };
    let text = CString::new("Ewa, eu te amo!").unwrap();
    unsafe {
        lvgl_sys::lv_label_set_text(label, text.as_ptr());
    }

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::from_millis(300));
        // The rest of the game loop goes here...

        unsafe {
            lvgl_sys::lv_task_handler();
        }
    }

    Ok(())
}

struct DisplayDriver<F>
    where F: FnMut(Vec<Point>, Vec<Color>)
{
    pub raw: lvgl_sys::lv_disp_drv_t,
    callback: F,
}

impl<F> DisplayDriver<F>
    where F: FnMut(Vec<Point>, Vec<Color>)
{
    fn new(mut callback: F) -> Self {
        let disp_drv = unsafe {
            let mut disp_drv = MaybeUninit::<lvgl_sys::lv_disp_drv_t>::uninit().assume_init(); /*Descriptor of a display driver*/
            lvgl_sys::lv_disp_drv_init(&mut disp_drv); // Basic initialization
            disp_drv.flush_cb = Some(display_callback_wrapper::<F>); // Set your driver function
            disp_drv.user_data = &mut callback as *mut _ as *mut c_void;
            disp_drv
        };
        Self{ raw: disp_drv, callback }
    }
}

unsafe extern "C" fn display_callback_wrapper<F>(
    disp_drv: *mut lvgl_sys::lv_disp_drv_t,
    area: *const lvgl_sys::lv_area_t,
    color_p: *mut lvgl_sys::lv_color_t
)
    where F: FnMut(Vec<Point>, Vec<Color>)
{
    // we need to make sure panics can't escape across the FFI boundary.
    let _ = panic::catch_unwind(|| {
        let mut i = 0;
        let mut disp = *disp_drv;
        let closure = &mut *(disp.user_data as *mut F);
        let mut points = vec![];
        let mut colors = vec![];

        for y in (*area).y1..=(*area).y2 {
            for x in (*area).x1..=(*area).x2 {
                points.push(Point::new(x as i32, y as i32));
                let raw_color = *color_p.add(i);
                let color = Color::from((
                    raw_color.ch.red,
                    raw_color.ch.green,
                    raw_color.ch.blue,
                    raw_color.ch.alpha,
                ));
                colors.push(color);
                i = i + 1;
            }
        }
        closure(points, colors);
        lvgl_sys::lv_disp_flush_ready(disp_drv); // Indicate you are ready with the flushing
    });
}
