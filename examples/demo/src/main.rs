use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use lvgl;
use lvgl::{Object, UI};
use lvgl_sys;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), String> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(
        lvgl_sys::LV_HOR_RES_MAX,
        lvgl_sys::LV_VER_RES_MAX,
    ));

    let output_settings = OutputSettingsBuilder::new().scale(4).build();
    let mut window = Window::new("PineTime", &output_settings);

    let mut ui = UI::init().unwrap();

    // Implement and register your display:
    let display_driver = lvgl::DisplayDriver::new(&mut display);
    ui.disp_drv_register(display_driver);

    // Create screen and widgets
    let mut screen = ui.scr_act();

    let font_roboto_28 = unsafe { &lvgl_sys::lv_font_roboto_28 };
    let font_noto_sans_numeric_28 = unsafe { &noto_sans_numeric_80 };

    let mut screen_style = lvgl::Style::new();
    screen_style.set_body_main_color(lvgl::Color::from_rgb((0, 0, 0)));
    screen_style.set_body_grad_color(lvgl::Color::from_rgb((0, 0, 0)));
    screen.set_style(screen_style);

    let mut time = lvgl::Label::new(&mut screen);
    let mut style_time = lvgl::Style::new();
    style_time.set_text_font(font_noto_sans_numeric_28);
    style_time.set_text_color(lvgl::Color::from_rgb((255, 255, 255)));
    time.set_style(style_time);
    time.set_align(&mut screen, lvgl::Align::InLeftMid, 20, 0);
    time.set_text("20:46");
    time.set_width(240);
    time.set_height(240);

    let mut bt = lvgl::Label::new(&mut screen);
    let mut style_bt = lvgl::Style::new();
    style_bt.set_text_font(font_roboto_28);
    let style_power = style_bt.clone();
    bt.set_style(style_bt);
    bt.set_width(50);
    bt.set_height(80);
    bt.set_recolor(true);
    bt.set_text("#5794f2 \u{F293}#");
    bt.set_label_align(lvgl::LabelAlign::Left);
    bt.set_align(&mut screen, lvgl::Align::InTopLeft, 0, 0);

    let mut power = lvgl::Label::new(&mut screen);
    power.set_style(style_power);
    power.set_recolor(true);
    power.set_width(80);
    power.set_height(20);
    power.set_text("#fade2a 20%#");
    power.set_label_align(lvgl::LabelAlign::Right);
    power.set_align(&mut screen, lvgl::Align::InTopRight, 0, 0);

    let threaded_ui = Arc::new(Mutex::new(ui));

    let (stop_ch, read_ch) = mpsc::channel();
    let closure_ui = threaded_ui.clone();
    let tick_thr = std::thread::spawn(move || loop {
        let period = Duration::from_millis(5);
        closure_ui.lock().unwrap().tick_inc(period);

        sleep(period);
        if read_ch.try_recv().is_ok() {
            break;
        }
    });

    let mut i = 0;
    'running: loop {
        if i > 59 {
            i = 0;
        }
        time.set_text(format!("21:{:02}", i).as_str());
        i = 1 + i;

        sleep(Duration::from_millis(
            lvgl_sys::LV_DISP_DEF_REFR_PERIOD as u64,
        ));

        threaded_ui.lock().unwrap().task_handler();

        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }
    }

    stop_ch.send(true).unwrap();
    tick_thr.join().unwrap();

    Ok(())
}

// Reference to native font for LittlevGL, defined in the file: "fonts_noto_sans_numeric_80.c"
// TODO: Create a macro for defining a safe wrapper for fonts.
// Maybe sometihng like:
//
// font_declare! {
//     NotoSansNumeric80 = noto_sans_numeric_80;
// };
//
extern "C" {
    pub static mut noto_sans_numeric_80: lvgl_sys::lv_font_t;
}
