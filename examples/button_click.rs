use cstr_core::CString;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use lvgl::input_device::{BufferStatus, InputData, Pointer};
use lvgl::style::Style;
use lvgl::widgets::{Btn, Label};
use lvgl::{self, Align, Color, LvError, Part, State, Widget, UI};
use std::cell::RefCell;
use std::rc::Rc;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() -> Result<(), LvError> {
    let display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(lvgl::HOR_RES_MAX, lvgl::VER_RES_MAX));

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Bar Example", &output_settings);

    let mut ui = UI::init()?;

    // Implement and register your display:
    ui.disp_drv_register(display)?;

    // Initial state of input
    let latest_touch_status: Rc<RefCell<BufferStatus>> = Rc::new(RefCell::new(
        InputData::Touch(Point::new(0, 0)).released().once(),
    ));

    // Register the input mode
    let internal = Rc::clone(&latest_touch_status);
    let mut touch_screen = Pointer::new(move || {
        let input_status = internal.borrow().clone();
        //println!("input_status = {:?}", input_status);
        input_status
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
        println!("Some event! {:?}", event);
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
    let mut latest_touch_point = Point::new(0, 0);
    'running: loop {
        ui.task_handler();
        window.update(ui.get_display_ref().unwrap());

        let mut events = window.events().peekable();

        if events.peek().is_none() {
            latest_touch_status.replace(
                InputData::Touch(latest_touch_point.clone())
                    .released()
                    .once(),
            );
        }

        for event in events {
            match event {
                SimulatorEvent::MouseButtonUp {
                    mouse_btn: _,
                    point,
                } => {
                    println!("Clicked on: {:?}", point);
                    // Send a event to the button directly
                    latest_touch_point = point.clone();
                    latest_touch_status.replace(InputData::Touch(point).pressed().once());
                }
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }

        sleep(Duration::from_millis(50));

        ui.tick_inc(loop_started.elapsed());
        loop_started = Instant::now();
    }

    Ok(())
}
