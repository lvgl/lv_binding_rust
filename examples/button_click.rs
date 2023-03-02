use cstr_core::CString;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use lvgl;
use lvgl::input_device::{
    generic::InputDriver,
    pointer::{Pointer, PointerInputData},
};
use lvgl::style::Style;
use lvgl::widgets::{Btn, Label};
use lvgl::{
    Align, Color, Display, DrawBuffer, LvError, Part, State, Widget, HOR_RES_MAX, VER_RES_MAX,
};
use std::cell::RefCell;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() -> Result<(), LvError> {
    lvgl::init();
    let sim_display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(HOR_RES_MAX, VER_RES_MAX));

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Button Example", &output_settings);

    let shared_native_display = RefCell::new(sim_display);

    let buffer = DrawBuffer::<{ (HOR_RES_MAX * VER_RES_MAX) as usize }>::new();

    let display = Display::register(&buffer, |refresh| {
        shared_native_display
            .borrow_mut()
            .draw_iter(refresh.as_pixels())
            .unwrap();
    })?;

    // Define the initial state of your input
    let mut latest_touch_status = PointerInputData::Touch(Point::new(0, 0)).released().once();

    // Register a new input device that's capable of reading the current state of the input
    let mut touch_screen = Pointer::new(|| latest_touch_status);
    lvgl::indev_drv_register(&mut touch_screen)?;

    // Create screen and widgets
    let mut screen = display.get_scr_act()?;

    let mut screen_style = Style::default();
    screen_style.set_bg_color(State::DEFAULT, Color::from_rgb((0, 0, 0)));
    screen.add_style(Part::Main, &mut screen_style)?;
    // Create the button
    let mut button = Btn::create(&mut screen, None)?;
    button.set_align(&mut screen, Align::InLeftMid, 30, 0)?;
    button.set_size(180, 80)?;
    let mut btn_lbl = Label::create(&mut button, None)?;
    btn_lbl.set_text(CString::new("Click me!").unwrap().as_c_str())?;

    let mut btn_state = false;
    button.on_event(|mut btn, event| {
        println!("Button received event: {:?}", event);
        if let lvgl::Event::Clicked = event {
            if btn_state {
                let nt = CString::new("Click me!").unwrap();
                btn_lbl.set_text(nt.as_c_str()).unwrap();
            } else {
                let nt = CString::new("Clicked!").unwrap();
                btn_lbl.set_text(nt.as_c_str()).unwrap();
            }
            btn_state = !btn_state;
            btn.toggle().unwrap();
        }
    })?;

    let mut latest_touch_point = Point::new(0, 0);
    'running: loop {
        lvgl::task_handler();
        window.update(&shared_native_display.borrow());

        let mut events = window.events().peekable();

        if events.peek().is_none() {
            latest_touch_status = PointerInputData::Touch(latest_touch_point.clone())
                .released()
                .once();
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
                    latest_touch_status = PointerInputData::Touch(point).pressed().once();
                }
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }

        lvgl::tick_inc(Duration::from_millis(15));
    }

    Ok(())
}
