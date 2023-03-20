use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use lvgl;
use lvgl::widgets::Label;
use lvgl::{Display, DrawBuffer};
use std::cell::RefCell;

type ColorSpace = Rgb565;

#[allow(unused_mut)]
#[allow(unused_variables)]
fn main() {
    const HOR_RES: u32 = 240;
    const VER_RES: u32 = 240;

    let embedded_graphics_display: SimulatorDisplay<ColorSpace> =
        SimulatorDisplay::new(Size::new(HOR_RES, VER_RES));

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("App Example", &output_settings);

    let shared_native_display = RefCell::new(embedded_graphics_display);

    // LVGL usage
    lvgl::init();

    let buffer = DrawBuffer::<{ (HOR_RES * VER_RES) as usize }>::new();

    let display = Display::register(buffer, HOR_RES, VER_RES, |refresh| {
        shared_native_display
            .borrow_mut()
            .draw_iter(refresh.as_pixels())
            .unwrap();
    })
    .unwrap();
}
