
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use lvgl;
use lvgl::style::{Opacity, Style};
use lvgl::widgets::Meter;
use lvgl::{
    Align, Color, Display, DrawBuffer, LvError, Part, Widget,
};
use std::time::Duration;
use std::time::Instant;

fn main() -> Result<(), LvError> {
    const HOR_RES: u32 = 240;
    const VER_RES: u32 = 240;

    let mut sim_display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(HOR_RES, VER_RES));

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Meter Example", &output_settings);

    let buffer = DrawBuffer::<{ (HOR_RES * VER_RES) as usize }>::default();

    let display = Display::register(buffer, HOR_RES, VER_RES, |refresh| {
        sim_display
            .draw_iter(refresh.as_pixels())
            .unwrap();
    })?;

    let mut screen = display.get_scr_act()?;

    let mut screen_style = Style::default();
    screen_style.set_bg_color(Color::from_rgb((0, 0, 0)));
    screen.add_style(Part::Main, &mut screen_style)?;

    // Create the meter
    let mut meter_style = Style::default();
    // Set a background color and a radius
    meter_style.set_radius(5);
    meter_style.set_bg_opa(Opacity::OPA_COVER);
    meter_style.set_bg_color(Color::from_rgb((192, 192, 192)));
    // Set some padding's
    //meter_style.set_pad_inner(20);
    meter_style.set_pad_top(20);
    meter_style.set_pad_left(5);
    meter_style.set_pad_right(5);

    //meter_style.set_scale_end_color(Color::from_rgb((255, 0, 0)));
    meter_style.set_line_color(Color::from_rgb((255, 255, 255)));
    //meter_style.set_scale_grad_color(Color::from_rgb((0, 0, 255)));
    meter_style.set_line_width(2);
    //meter_style.set_scale_end_line_width(4);
    //meter_style.set_scale_end_border_width(4);

    let mut meter = Meter::create(&mut screen)?;
    meter.add_style(Part::Main, &mut meter_style)?;
    meter.set_align(Align::Center, 0, 0)?;
    meter.set_value(0, 50)?;

    let mut i = 0;
    let mut loop_started = Instant::now();
    'running: loop {
        meter.set_value(0, i)?;

        lvgl::task_handler();
        window.update(&sim_display);

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

        if i > 99 {
            i = 0;
        } else {
            i = i + 1;
        }

        lvgl::tick_inc(Duration::from_millis(16));
    }

    Ok(())
}


//fn main() {
//    println!("Currently broken :c")
//}
