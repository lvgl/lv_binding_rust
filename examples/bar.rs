use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use lvgl::{self, widgets::Bar, Align, Color, DisplayDriver, Label, Object, Style, UI};
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
    let mut window = Window::new("Bar Example", &output_settings);

    let mut ui = UI::init().unwrap();

    // Implement and register your display:
    let display_driver = DisplayDriver::new(&mut display);
    ui.disp_drv_register(display_driver);

    // Create screen and widgets
    let mut screen = ui.scr_act();

    let mut screen_style = Style::new();
    screen_style.set_body_main_color(Color::from_rgb((0, 0, 0)));
    screen_style.set_body_grad_color(Color::from_rgb((0, 0, 0)));
    screen_style.set_body_radius(0);
    screen.set_style(screen_style);

    // Create the bar object
    let mut bar = Bar::new(&mut screen);
    bar.set_size(175, 50);
    bar.set_align(&mut screen, Align::Center, 0, 0);
    bar.set_range(0, 100);
    bar.set_value(0);

    let mut loading_lbl = Label::new(&mut screen);
    loading_lbl.set_text("Loading...");
    loading_lbl.set_align(&mut bar, Align::OutTopMid, 0, -10);

    let mut loading_style = Style::new();
    loading_style.set_text_color(Color::from_rgb((255, 255, 255)));
    loading_lbl.set_style(loading_style);

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
        if i > 100 {
            i = 0;
        }
        bar.set_value(i);
        i += 1;

        sleep(Duration::from_millis(25));

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
