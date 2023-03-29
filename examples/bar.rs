use cstr_core::CString;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use lvgl;
use lvgl::style::Style;
use lvgl::widgets::{Bar, Label};
use lvgl::{Align, Animation, Color, Display, DrawBuffer, Event, LvError, Part, Widget};
use std::cell::RefCell;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

fn main() -> Result<(), LvError> {
    const HOR_RES: u32 = 240;
    const VER_RES: u32 = 240;

    let sim_display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(HOR_RES, VER_RES));

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Bar Example", &output_settings);

    let shared_native_display = RefCell::new(sim_display);

    let buffer = DrawBuffer::<{ (HOR_RES * VER_RES) as usize }>::new();

    let display = Display::register(buffer, HOR_RES, VER_RES, |refresh| {
        shared_native_display
            .borrow_mut()
            .draw_iter(refresh.as_pixels())
            .unwrap();
    })?;

    let mut screen = display.get_scr_act()?;

    let mut screen_style = Style::default();
    screen_style.set_bg_color(Color::from_rgb((255, 255, 255)));
    screen_style.set_radius(0);
    screen.add_style(Part::Main, &mut screen_style)?;

    // Create the bar object
    let mut bar = Bar::create(&mut screen)?;
    bar.set_size(175, 20)?;
    bar.set_align(Align::Center, 0, 10)?;
    bar.set_range(0, 100)?;
    bar.on_event(|_b, _e| {
        println!("Completed!");
    })?;

    // Set the indicator style for the bar object
    let mut ind_style = Style::default();
    ind_style.set_bg_color(Color::from_rgb((100, 245, 100)));
    bar.add_style(Part::Any, &mut ind_style)?;

    let mut loading_lbl = Label::create(&mut screen)?;
    loading_lbl.set_text(CString::new("Loading...").unwrap().as_c_str())?;
    loading_lbl.set_align(Align::OutTopMid, 0, 0)?;

    let mut loading_style = Style::default();
    loading_style.set_text_color(Color::from_rgb((0, 0, 0)));
    loading_lbl.add_style(Part::Main, &mut loading_style)?;

    let mut i = 0;
    'running: loop {
        let start = Instant::now();
        if i > 100 {
            i = 0;
            lvgl::event_send(&mut bar, Event::Clicked)?;
        }
        bar.set_value(i, Animation::ON)?;
        i += 1;

        lvgl::task_handler();
        window.update(&shared_native_display.borrow());

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }
        sleep(Duration::from_millis(15));
        lvgl::tick_inc(Instant::now().duration_since(start));
    }

    Ok(())
}
