use cstr_core::CString;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use lvgl::input_device::{InputData, Pointer};
use lvgl::style::Style;
use lvgl::widgets::{Btn, Label};
use lvgl::{self, Align, Color, LvError, Part, State, Widget, UI};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

fn main() -> Result<(), LvError> {
    let display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(lvgl::HOR_RES_MAX, lvgl::VER_RES_MAX));

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Bar Example", &output_settings);

    let mut ui = UI::init()?;

    // Implement and register your display:
    ui.disp_drv_register(display)?;

    // Register the input mode
    let latest_touch_point: Rc<RefCell<Option<Point>>> = Rc::new(RefCell::new(None));
    let internal = Rc::clone(&latest_touch_point);
    let mut touch_screen = Pointer::new(move || {
        let info = internal.borrow().clone();
        if info.is_some() {
            let point = info.unwrap();
            println!("Changed to {:?}", point);
            Some(InputData::Touch(point).pressed().once())
        } else {
            None
        }
    });

    ui.indev_drv_register(&mut touch_screen)?;

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
                let nt = CString::new("Click me!").unwrap();
                btn_lbl.set_text(nt.as_c_str()).unwrap();
            } else {
                let nt = CString::new("Clicked!").unwrap();
                btn_lbl.set_text(nt.as_c_str()).unwrap();
            }
            btn_state = !btn_state;
            println!("Clicked! Inner..");
            btn.toggle().unwrap();
        }
    })?;

    let mut loop_started = Instant::now();
    'running: loop {
        ui.task_handler();
        window.update(ui.get_display_ref().unwrap());

        for event in window.events() {
            match event {
                SimulatorEvent::MouseButtonUp {
                    mouse_btn: _,
                    point,
                } => {
                    println!("Clicked on: {:?}", point);
                    // Send a event to the button directly
                    latest_touch_point.borrow_mut().replace(point);
                }
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }

        ui.tick_inc(loop_started.elapsed());
        loop_started = Instant::now();
    }

    Ok(())
}
