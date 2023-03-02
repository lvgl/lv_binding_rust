use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use lvgl;
use lvgl::widgets::Label;
use lvgl::{Display, DrawBuffer, DISP_HOR_RES, DISP_VER_RES};
use std::borrow::Borrow;
use std::cell::RefCell;

type ColorSpace = Rgb565;

fn main() {
    let embedded_graphics_display: SimulatorDisplay<ColorSpace> =
        SimulatorDisplay::new(Size::new(DISP_HOR_RES as u32, DISP_VER_RES as u32));

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("App Example", &output_settings);

    let mut shared_native_display = RefCell::new(embedded_graphics_display);

    // LVGL usage
    lvgl::init();

    let buffer = DrawBuffer::<{ (DISP_HOR_RES * DISP_VER_RES) as usize }>::new();

    let display = Display::register(&buffer, |refresh| {
        shared_native_display
            .borrow_mut()
            .draw_iter(refresh.as_pixels());
    })
    .unwrap();

    let label: Label = "Nice!".into();
}
