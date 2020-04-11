use std::ffi::CString;
use std::os::raw::c_void;
use std::time::Duration;
use std::mem::MaybeUninit;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormat, PixelFormatEnum};

use lvgl_sys;
use lvgl_sys::{lv_btn_create, lv_color_t, lv_disp_buf_init, lv_disp_buf_t, lv_disp_drv_init, lv_disp_drv_register, lv_disp_drv_t, lv_disp_get_hor_res, lv_disp_get_scr_act, lv_disp_get_ver_res, lv_label_create, lv_label_set_text, lv_obj_set_pos, lv_obj_set_size, lv_obj_set_x, lv_obj_set_y, lv_obj_t, lv_task_handler, lv_area_t, lv_disp_flush_ready, LV_HOR_RES_MAX, lv_style_t, lv_style_copy, lv_style_btn_rel, LV_BTN_STYLE_PR, LV_BTN_STYLE_REL, lv_slider_set_style};

fn main() -> Result<(), String> {
    unsafe {
        lvgl_sys::lv_init();
    }

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let hr: u32 = unsafe { lv_disp_get_hor_res(std::ptr::null_mut()) as u32 };
    let vr: u32 = unsafe { lv_disp_get_ver_res(std::ptr::null_mut()) as u32 };

    let window = video_subsystem.window("TFT Display: Demo", hr, vr)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    // Create a display buffer for LittlevGL
    let mut disp_buf = MaybeUninit::<lv_disp_buf_t>::uninit();
    let mut buf: [MaybeUninit<lv_color_t>;  LV_HOR_RES_MAX as usize * 10] = unsafe { MaybeUninit::uninit().assume_init()  };          /*Declare a buffer for 10 lines*/
    unsafe {
        lv_disp_buf_init(disp_buf.as_mut_ptr(), buf.as_mut_ptr() as *mut c_void, std::ptr::null_mut(), (hr * 10) as u32);    /*Initialize the display buffer*/
    }

    // Implement and register a function which can copy a pixel array to an area of your display:
    unsafe {
        unsafe extern "C" fn my_disp_flush(disp_drv: *mut lv_disp_drv_t, area: *const lv_area_t, color_p: *mut lv_color_t) {
            let mut i = 1;
            for y in (*area).y1..(*area).y2 {
                for x in (*area).x1..(*area).x2 {
                    // Put a pixel to the display.
                    let raw_color = *color_p.add(i);
                    let color = Color::from((raw_color.ch.red, raw_color.ch.green, raw_color.ch.blue, raw_color.ch.alpha));
                    if raw_color.ch.blue != 255 {
                        println!("{}x{} - {:?}", x, y, color.rgb());
                    }
                    i = i + 1;
                }
            }

            lv_disp_flush_ready(disp_drv);         /* Indicate you are ready with the flushing*/
        }

        let mut disp_drv = MaybeUninit::<lv_disp_drv_t>::uninit().assume_init();                  /*Descriptor of a display driver*/
        lv_disp_drv_init(&mut disp_drv);     /*Basic initialization*/
        disp_drv.flush_cb = Some(my_disp_flush);                /*Set your driver function*/
        disp_drv.buffer = disp_buf.as_mut_ptr();          /*Assign the buffer to the display*/
        lv_disp_drv_register(&mut disp_drv); /*Finally register the driver*/
    }

    // Create screen and widgets
    let mut screen = unsafe { lv_disp_get_scr_act(std::ptr::null_mut()) };
    let mut btn = unsafe { lv_btn_create(screen, std::ptr::null_mut()) };
    unsafe {
        lv_obj_set_pos(btn, 10, 10);
        lv_obj_set_size(btn, 200, 50)
    }
    let mut label = unsafe { lv_label_create(btn, std::ptr::null_mut()) };
    unsafe {
        lv_label_set_text(label, CString::new("Click me!").unwrap().as_ptr());
    }

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::from_millis(300));
        // The rest of the game loop goes here...

        unsafe {
            lv_task_handler();
        }
    }

    Ok(())
}
