use lvgl;
use lvgl::Object;
use lvgl_sys;
use std::time::Duration;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::mock_display::MockDisplay;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window, BinaryColorTheme, SimulatorEvent};

fn main() -> Result<(), String> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(lvgl_sys::LV_HOR_RES_MAX,lvgl_sys::LV_VER_RES_MAX));

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Hello World", &output_settings);

    unsafe {
        lvgl_sys::lv_init();
    }

    // Implement and register your display:
    let mut display_driver = lvgl::DisplayDriver::new(&mut display);

    // Create screen and widgets
    let mut screen = display_driver.get_active_screen();

    let font_roboto_28 = unsafe { &lvgl_sys::lv_font_roboto_28 };
    let font_noto_sans_numeric_28 = unsafe { &noto_sans_numeric_80 };

    let mut screen_style = lvgl::Style::new();
    screen_style.set_body_main_color(lvgl::Color::from_rgb((0, 0, 0)));
    screen_style.set_body_grad_color(lvgl::Color::from_rgb((0, 0, 0)));
    screen.set_style(&mut screen_style);

    let mut time = lvgl::Label::new(&mut screen);
    let mut style_time = lvgl::Style::new();
    style_time.set_text_font(font_noto_sans_numeric_28);
    style_time.set_text_color(lvgl::Color::from_rgb((255, 255, 255)));
    time.set_style(&mut style_time);
    time.set_align(&mut screen, lvgl::Align::InLeftMid, 20, 0);
    time.set_text("20:46\0");
    time.set_width(240);
    time.set_height(240);

    let mut bt = lvgl::Label::new(&mut screen);
    let mut style_bt = lvgl::Style::new();
    style_bt.set_text_font(font_roboto_28);
    let mut style_power = style_bt.clone();
    bt.set_style(&mut style_bt);
    bt.set_width(50);
    bt.set_height(80);
    bt.set_recolor(true);
    bt.set_text("#5794f2 \u{F293}#\0");
    bt.set_label_align(lvgl::LabelAlign::Left);
    bt.set_align(&mut screen, lvgl::Align::InTopLeft, 0, 0);

    let mut power = lvgl::Label::new(&mut screen);
    power.set_style(&mut style_power);
    power.set_recolor(true);
    power.set_width(80);
    power.set_height(20);
    power.set_text("#fade2a 20%#\0");
    power.set_label_align(lvgl::LabelAlign::Right);
    power.set_align(&mut screen, lvgl::Align::InTopRight, 0, 0);


    let mut i = 0;
    'running: loop {
        if i > 59 {
            i = 0;
        }
        time.set_text(format!("21:{:02}\0", i).as_str());
        i = 1 + i;

        ::std::thread::sleep(Duration::from_millis(10));

        unsafe {
            lvgl_sys::lv_task_handler();
            lvgl_sys::lv_tick_inc(10);
        }
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }
    }

    Ok(())
}

// Reference to native font for LittlevGL, defined in the file: "fonts_noto_sans_numeric_80.c"
extern "C" {
    pub static mut noto_sans_numeric_80: lvgl_sys::lv_font_t;
}
