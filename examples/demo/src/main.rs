use lvgl_sys;
use lvgl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::os::raw::c_void;
use std::panic;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "TFT Display: Demo",
            lvgl_sys::LV_HOR_RES_MAX,
            lvgl_sys::LV_VER_RES_MAX,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

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
            (lvgl_sys::LV_HOR_RES_MAX * 10) as u32,
        );
    }

    // Implement and register a function which can copy a pixel array to an area of your display:
    let mut display_driver = DisplayDriver::new(move |points, colors| {
        for (i, point) in points.into_iter().enumerate() {
            canvas.set_draw_color(colors[i]);
            canvas.draw_point(point).unwrap();
        }
        canvas.present();
    });

    display_driver.raw.buffer = disp_buf.as_mut_ptr(); // Assign the buffer to the display
    unsafe {
        lvgl_sys::lv_disp_drv_register(&mut display_driver.raw); // Finally register the driver
    }

    // Create screen and widgets
    let mut screen = lvgl::display::get_active_screen();

    let mut button = lvgl::Button::new(&mut screen);
    button.set_pos(100, 10);

    let mut label = lvgl::Label::new(&mut button);
    label.set_text("Hello Beauty!");

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
where
    F: FnMut(Vec<Point>, Vec<Color>),
{
    pub raw: lvgl_sys::lv_disp_drv_t,
    callback: F,
}

impl<F> DisplayDriver<F>
where
    F: FnMut(Vec<Point>, Vec<Color>),
{
    fn new(mut callback: F) -> Self {
        let disp_drv = unsafe {
            let mut disp_drv = MaybeUninit::<lvgl_sys::lv_disp_drv_t>::uninit().assume_init(); /*Descriptor of a display driver*/
            lvgl_sys::lv_disp_drv_init(&mut disp_drv); // Basic initialization
            disp_drv.flush_cb = Some(display_callback_wrapper::<F>); // Set your driver function
            disp_drv.user_data = &mut callback as *mut _ as *mut c_void;
            disp_drv
        };
        Self {
            raw: disp_drv,
            callback,
        }
    }
}

unsafe extern "C" fn display_callback_wrapper<F>(
    disp_drv: *mut lvgl_sys::lv_disp_drv_t,
    area: *const lvgl_sys::lv_area_t,
    color_p: *mut lvgl_sys::lv_color_t,
) where
    F: FnMut(Vec<Point>, Vec<Color>),
{
    // we need to make sure panics can't escape across the FFI boundary.
    let _ = panic::catch_unwind(|| {
        let mut i = 0;
        let disp = *disp_drv;
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
