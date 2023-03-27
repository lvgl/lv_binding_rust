// Still WIP
//#![allow(unused_labels)]
//#![allow(unused_variables)]
//#![allow(unreachable_code)]

use lvgl::LvResult;
use lvgl::lv_drv_disp_sdl;
use lvgl::lv_drv_input_pointer_sdl;
use lvgl::input_device::generic::InputDriver;
use lvgl::style::Style;
use lvgl::widgets::{Btn, Label};
use lvgl::{Align, Color, DrawBuffer, Part, Widget};
use std::time::Duration;
use std::time::Instant;
use std::thread::sleep;
use cstr_core::CString;

fn main() -> LvResult<()> {
    const HOR_RES: u32 = 240;
    const VER_RES: u32 = 240;

    lvgl::init();
    let buffer = DrawBuffer::<{ (HOR_RES * VER_RES) as usize }>::new();
    let display = lv_drv_disp_sdl!(buffer, HOR_RES, VER_RES)?;
    let mut input = lv_drv_input_pointer_sdl!();
    lvgl::indev_drv_register(&mut input)?;

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
            //btn.toggle().unwrap();
        }
    })?;

    loop {
        let start = Instant::now();
        lvgl::task_handler();
        //println!("Loop");
        sleep(Duration::from_millis(15));
        lvgl::tick_inc(Instant::now().duration_since(start));
    }
}