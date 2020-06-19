use cstr_core::CString;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use lvgl::style::Style;
use lvgl::widgets::{Bar, Label, LabelAlign};
use lvgl::{self, Align, Animation, Color, Event, LvError, Part, State, Widget, UI};
use lvgl_sys;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() -> Result<(), LvError> {
    let display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(
        lvgl_sys::LV_HOR_RES_MAX,
        lvgl_sys::LV_VER_RES_MAX,
    ));

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Bar Example", &output_settings);

    let mut ui = UI::init()?;

    // Implement and register your display:
    ui.disp_drv_register(display).unwrap();

    // Create screen and widgets
    let mut screen = ui.scr_act()?;

    let mut screen_style = Style::default();
    screen_style.set_bg_color(State::DEFAULT, Color::from_rgb((255, 255, 255)));
    screen_style.set_radius(State::DEFAULT, 0);
    screen.add_style(Part::Main, screen_style)?;

    // Create the bar object
    let mut bar = Bar::new(&mut screen)?;
    bar.set_size(175, 20)?;
    bar.set_align(&mut screen, Align::Center, 0, 10)?;
    bar.set_range(0, 100)?;
    bar.on_event(|_b, _e| {
        println!("Completed!");
    })?;

    // // Set the indicator style for the bar object
    let mut ind_style = Style::default();
    ind_style.set_bg_color(State::DEFAULT, Color::from_rgb((100, 245, 100)));
    bar.add_style(Part::All, ind_style)?;

    let mut loading_lbl = Label::new(&mut screen)?;
    loading_lbl.set_text(CString::new("Loading...").unwrap().as_c_str())?;
    loading_lbl.set_align(&mut bar, Align::OutTopMid, 0, -10)?;
    loading_lbl.set_label_align(LabelAlign::Center)?;

    let mut loading_style = Style::default();
    loading_style.set_text_color(State::DEFAULT, Color::from_rgb((0, 0, 0)));
    loading_lbl.add_style(Part::Main, loading_style)?;

    let threaded_ui = Arc::new(Mutex::new(ui));

    let (stop_ch, read_ch) = mpsc::channel();
    let closure_ui = threaded_ui.clone();
    let mut loop_started = Instant::now();
    let tick_thr = std::thread::spawn(move || loop {
        // Needs to be called periodically for LittlevGL internal timing calculations.
        {
            let mut ui = closure_ui.lock().unwrap();
            ui.tick_inc(loop_started.elapsed());
        }

        sleep(Duration::from_millis(5));
        if read_ch.try_recv().is_ok() {
            break;
        }
        loop_started = Instant::now();
    });

    let mut i = 0;
    'running: loop {
        if i > 100 {
            i = 0;
            threaded_ui
                .lock()
                .unwrap()
                .event_send(&mut bar, Event::Clicked)?
        }
        bar.set_value(i, Animation::ON)?;
        i += 1;

        sleep(Duration::from_millis(50));

        threaded_ui.lock().unwrap().task_handler();
        if let Some(disp) = threaded_ui.lock().unwrap().get_display_ref() {
            window.update(disp);
        }

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
