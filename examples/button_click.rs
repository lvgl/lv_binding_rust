use cstr_core::CString;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use lvgl;
use lvgl::input_device::{
    pointer::{Pointer, PointerInputData},
    InputDriver,
};
use lvgl::style::Style;
use lvgl::widgets::{Btn, Label};
use lvgl::{Align, Color, Display, DrawBuffer, LvError, Part, Widget};
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

#[allow(unused_assignments)]
fn main() -> Result<(), LvError> {
    const HOR_RES: u32 = 240;
    const VER_RES: u32 = 240;

    let mut sim_display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(HOR_RES, VER_RES));

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Button Example", &output_settings);

    let buffer = DrawBuffer::<{ (HOR_RES * VER_RES) as usize }>::default();

    let display = Display::register(buffer, HOR_RES, VER_RES, |refresh| {
        sim_display.draw_iter(refresh.as_pixels()).unwrap();
    })?;

    // Define the initial state of your input
    let mut latest_touch_status = PointerInputData::Touch(Point::new(0, 0)).released().once();

    // Register a new input device that's capable of reading the current state of the input
    let _touch_screen = Pointer::register(|| latest_touch_status, &display)?;

    // Create screen and widgets
    let mut screen = display.get_scr_act()?;

    let mut screen_style = Style::default();
    screen_style.set_bg_color(Color::from_rgb((0, 0, 0)));
    screen.add_style(Part::Main, &mut screen_style)?;
    // Create the button
    let mut button = Btn::create(&mut screen)?;
    button.set_align(Align::LeftMid, 30, 0)?;
    button.set_size(180, 80)?;
    let mut btn_lbl = Label::create(&mut button)?;
    btn_lbl.set_text(CString::new("Click me!").unwrap().as_c_str())?;

    let mut btn_state = false;
    button.on_event(|_btn, event| {
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
        }
    })?;

    let mut latest_touch_point = Point::new(0, 0);
    'running: loop {
        let start = Instant::now();
        lvgl::task_handler();
        window.update(&sim_display);

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
                    latest_touch_point = point.clone();
                    latest_touch_status = PointerInputData::Touch(point).pressed().once();
                }
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }
        sleep(Duration::from_millis(15));
        lvgl::tick_inc(Instant::now().duration_since(start));
    }

    Ok(())
}
