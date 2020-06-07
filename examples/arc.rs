use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use lvgl::style::Style;
use lvgl::widgets::{Arc, ArcPart, Label, LabelAlign};
use lvgl::{self, Align, Color, DisplayDriver, Part, State, UI};
use lvgl::{LvError, Widget};
use lvgl_sys;
use std::sync::{mpsc, Arc as StdArc, Mutex};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), LvError> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(
        lvgl_sys::LV_HOR_RES_MAX,
        lvgl_sys::LV_VER_RES_MAX,
    ));

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Arc Example", &output_settings);

    let mut ui = UI::init()?;

    // Implement and register your display:
    let display_driver = DisplayDriver::new(&mut display);
    ui.disp_drv_register(display_driver);

    // Create screen and widgets
    let mut screen = ui.scr_act()?;

    let mut screen_style = Style::default();
    screen_style.set_bg_color(State::DEFAULT, Color::from_rgb((255, 255, 255)));
    screen_style.set_radius(State::DEFAULT, 0);
    screen.add_style(Part::Main, screen_style)?;

    // Create the arc object
    let mut arc = Arc::new(&mut screen)?;
    arc.set_size(150, 150)?;
    arc.set_align(&mut screen, Align::Center, 0, 10)?;
    arc.set_start_angle(135, ArcPart::Indicator)?;
    arc.set_end_angle(135, ArcPart::Indicator)?;

    let mut loading_lbl = Label::new(&mut screen)?;
    loading_lbl.set_text("Loading...")?;
    loading_lbl.set_align(&mut arc, Align::OutTopMid, 0, -10)?;
    loading_lbl.set_label_align(LabelAlign::Center)?;

    let mut loading_style = Style::default();
    loading_style.set_text_color(State::DEFAULT, Color::from_rgb((0, 0, 0)));
    loading_lbl.add_style(Part::Main, loading_style)?;

    let threaded_ui = StdArc::new(Mutex::new(ui));

    let (stop_ch, read_ch) = mpsc::channel();
    let closure_ui = threaded_ui.clone();
    let tick_thr = std::thread::spawn(move || loop {
        let period = Duration::from_millis(10);
        closure_ui.lock().unwrap().tick_inc(period);

        sleep(period);
        if read_ch.try_recv().is_ok() {
            break;
        }
    });

    let mut angle = 0;
    let mut forward = true;

    let mut i = 0;
    'running: loop {
        if i > 270 {
            forward = if forward { false } else { true };
            i = 1;
        }
        angle = if forward { angle + 1 } else { angle - 1 };
        arc.set_end_angle(angle + 135, ArcPart::Indicator)?;
        i += 1;

        sleep(Duration::from_millis(10));

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
