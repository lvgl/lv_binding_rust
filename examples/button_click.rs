use cstr_core::CString;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use lvgl::style::Style;
use lvgl::widgets::{Btn, Label};
use lvgl::{self, Align, Color, Event, LvError, Part, State, Widget, UI};
use lvgl_sys;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), LvError> {
    let display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(
        lvgl_sys::LV_HOR_RES_MAX,
        lvgl_sys::LV_VER_RES_MAX,
    ));

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Bar Example", &output_settings);

    let mut ui = UI::init()?;

    // Implement and register your display:
    ui.disp_drv_register(display)?;

    // Create screen and widgets
    let mut screen = ui.scr_act()?;

    let mut screen_style = Style::default();
    screen_style.set_bg_color(State::DEFAULT, Color::from_rgb((0, 0, 0)));
    screen.add_style(Part::Main, screen_style)?;

    // Create the button
    let mut button = Btn::new(&mut screen)?;
    button.set_align(&mut screen, Align::InLeftMid, 30, 0)?;
    button.set_size(180, 80)?;
    let mut btn_lbl = Label::new(&mut button)?;
    btn_lbl.set_text(CString::new("Click me!").unwrap().as_c_str())?;

    let mut btn_state = false;
    button.on_event(|mut btn, event| {
        if let lvgl::Event::Clicked = event {
            if btn_state {
                btn_lbl
                    .set_text(CString::new("Click me!").unwrap().as_c_str())
                    .unwrap();
            } else {
                btn_lbl
                    .set_text(CString::new("Clicked!").unwrap().as_c_str())
                    .unwrap();
            }
            btn_state = !btn_state;
            println!("Clicked!");
            btn.toggle().unwrap();
        }
    })?;

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

    'running: loop {
        let mut ui = threaded_ui.lock().unwrap();
        ui.task_handler();
        if let Some(disp) = ui.get_display_ref() {
            window.update(disp);
        }
        for event in window.events() {
            match event {
                SimulatorEvent::MouseButtonUp {
                    mouse_btn: _,
                    point,
                } => {
                    println!("Clicked on: {:?}", point);
                    // Send a event to the button directly
                    ui.event_send(&mut button, Event::Clicked)?;
                }
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }

        sleep(Duration::from_millis(5));
    }

    stop_ch.send(true).unwrap();
    tick_thr.join().unwrap();

    Ok(())
}
