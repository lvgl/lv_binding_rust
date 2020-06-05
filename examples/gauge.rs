use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use lvgl::style::{Opacity, Style};
use lvgl::widgets::{Gauge, GaugePart};
use lvgl::{self, Align, Color, DisplayDriver, Part, State, Widget, UI};
use lvgl_sys;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), String> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(
        lvgl_sys::LV_HOR_RES_MAX,
        lvgl_sys::LV_VER_RES_MAX,
    ));

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Gauge Example", &output_settings);

    let mut ui = UI::init().unwrap();

    // Implement and register your display:
    let display_driver = DisplayDriver::new(&mut display);
    ui.disp_drv_register(display_driver);

    // Create screen and widgets
    let mut screen = ui.scr_act();

    let mut screen_style = Style::default();
    screen_style.set_bg_color(State::DEFAULT, Color::from_rgb((0, 0, 0)));
    screen.add_style(Part::Main, screen_style);

    // Create the gauge
    let mut gauge_style = Style::default();
    // Set a background color and a radius
    gauge_style.set_radius(State::DEFAULT, 5);
    gauge_style.set_bg_opa(State::DEFAULT, Opacity::OPA_COVER);
    gauge_style.set_bg_color(State::DEFAULT, Color::from_rgb((192, 192, 192)));
    // Set some paddings
    gauge_style.set_pad_inner(State::DEFAULT, 20);
    gauge_style.set_pad_top(State::DEFAULT, 20);
    gauge_style.set_pad_left(State::DEFAULT, 5);
    gauge_style.set_pad_right(State::DEFAULT, 5);

    gauge_style.set_scale_end_color(State::DEFAULT, Color::from_rgb((255, 0, 0)));
    gauge_style.set_line_color(State::DEFAULT, Color::from_rgb((255, 255, 255)));
    gauge_style.set_scale_grad_color(State::DEFAULT, Color::from_rgb((0, 0, 255)));
    gauge_style.set_line_width(State::DEFAULT, 2);
    gauge_style.set_scale_end_line_width(State::DEFAULT, 4);
    gauge_style.set_scale_end_border_width(State::DEFAULT, 4);

    let mut gauge = Gauge::new(&mut screen);
    gauge.add_style(GaugePart::Main, gauge_style);
    gauge.set_align(&mut screen, Align::Center, 0, 0);
    gauge.set_value(0, 50);

    let threaded_ui = Arc::new(Mutex::new(ui));

    let (stop_ch, read_ch) = mpsc::channel();
    let closure_ui = threaded_ui.clone();
    let tick_thr = std::thread::spawn(move || loop {
        let period = Duration::from_millis(5);

        // Needs to be called periodically for LittlevGL internal timing calculations.
        closure_ui.lock().unwrap().tick_inc(period);

        sleep(period);
        if read_ch.try_recv().is_ok() {
            break;
        }
    });

    let mut i = 0;
    'running: loop {
        threaded_ui.lock().unwrap().task_handler();

        window.update(&display);
        for event in window.events() {
            match event {
                SimulatorEvent::MouseButtonUp {
                    mouse_btn: _,
                    point,
                } => {
                    println!("Clicked on: {:?}", point);
                }
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }

        sleep(Duration::from_millis(25));
        gauge.set_value(0, i);

        if i > 99 {
            i = 0;
        } else {
            i = i + 1;
        }
    }

    stop_ch.send(true).unwrap();
    tick_thr.join().unwrap();

    Ok(())
}
