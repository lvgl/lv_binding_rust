use core::marker::PhantomData;
use core::time::Duration;
use core::sync::atomic::{AtomicBool, Ordering};

// There can only be a single reference to LittlevGL library.
static LVGL_IN_USE: AtomicBool = AtomicBool::new(false);

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum LvError {
    AlreadyInUse,
}

pub struct UI {
    // LittlevGL is not thread-safe by default.
    _not_send: PhantomData<*const ()>
}

impl UI {
    pub fn init() -> Result<Self, LvError> {
        if LVGL_IN_USE.compare_and_swap(false, true, Ordering::SeqCst) == false {
            // lvgl_sys::lv_init();
            Ok(Self {
                _not_send: PhantomData,
            })
        } else {
            Err(LvError::AlreadyInUse)
        }
    }

    pub fn disp_drv_register(&mut self, _display: &mut DisplayDriver) {
        // register it
        // lvgl_sys::lv_disp_drv_register(&mut disp_drv);
    }

    pub fn tick_inc(&mut self, _tick_period: Duration) {
        // lvgl_sys::lv_tick_inc(tick_period);
    }

    pub fn task_handler(&mut self) {
        // lvgl_sys::lv_task_handler();
    }
}

pub struct DisplayDriver {
    refresh_lines: u32,
    current_screen: Screen,
}

impl DisplayDriver {
    pub fn new(refresh_lines: u32) -> Self {
        Self{ refresh_lines, current_screen: Screen::new() }
    }

    // Ensure cannot delete the current loaded screen
    pub fn load_scr(&mut self, screen: Screen) {
        self.current_screen = screen;
    }

    pub fn scr_act(&mut self) -> &mut Screen {
        &mut self.current_screen
    }
}

pub trait LvObject {
    fn label_create(&self) -> Label where Self: Sized {
        Label{ parent: self }
    }

    fn btn_create<F>(&self) -> Button<F> where Self: Sized, F: FnMut() {
        Button{ parent: self, event_callback: None }
    }
}

pub struct Screen {
    _not_send: PhantomData<*const ()>
}

impl Screen {
    pub fn new() -> Self {
        Self { _not_send: PhantomData }
    }
}

impl LvObject for Screen {}

pub struct Label<'a> {
    parent: &'a dyn LvObject,
}

impl<'a> Label<'a> {
    pub fn set_text(&mut self, _text: &str) {
        // set text call to unsafe...
    }
}

impl<'a> LvObject for Label<'a> {}

pub struct Button<'a, F> where F: FnMut() {
    parent: &'a dyn LvObject,
    event_callback: Option<F>,
}

impl<'a, F> Button<'a, F> where F: FnMut() {
    pub fn on_event(&mut self, callback: F) {
        // add callback
        self.event_callback = Some(callback);
    }
}

impl<'a, F> LvObject for Button<'a, F> where F: FnMut() {}


#[cfg(test)]
mod test {
    use crate::api::{UI, DisplayDriver, LvObject};
    use core::time::Duration;
    use core::cell::RefCell;

    #[test]
    fn basic_usage() {
        let mut ui = UI::init().unwrap();

        let refresh_lines = 10;
        let mut display = DisplayDriver::new(refresh_lines);
        ui.disp_drv_register(&mut display);
        let display = RefCell::new(display);

        {
            let mut d = display.borrow_mut();
            let screen = d.scr_act();

            let mut button = screen.btn_create();

            button.on_event(|| {
                // something
                let mut disp = display.borrow_mut();
                let screen = disp.scr_act();
                let mut label = screen.label_create();
                label.set_text("Clicked");
            });

            let mut label = button.label_create();
            label.set_text("Click me!");
        };

        {
            let mut d = display.borrow_mut();
            let screen = d.scr_act();
            let mut button2 = screen.btn_create();
            button2.on_event(|| {
                // else
            });
            let mut label2 = button2.label_create();
            label2.set_text("Else");
        };

        ui.tick_inc(Duration::from_millis(5));
        ui.task_handler();
    }

    #[test]
    fn test_usage() {
        // // can use Arc<Mutex<lvgl::App>> to share between threads
        // let mut app = lvgl::App::init().unwrap();
        //
        // let disp = DisplayDriver::new();
        // app.register_display_driver(&disp); // takes (&mut self, ...)
        //
        // let screen: &mut lvgl::Screen = disp.new_screen(); // takes (&mut self)
        // disp.load_screen(&screen); // takes (&self) because it just need the ref to
        // // the screen to load, as all the screens are already internal.
        //
        // let button = screen.new_button(); // takes (&self)
        // button.on_event(|&mut app, &mut btn, ev| {
        //     let mut label: &mut Label = app.cur_screen().new_label();
        //     if let lvgl::Event::Clicked = ev {
        //         btn.set_text("clicked!");
        //     }
        //     app.load_screen(&screen);
        // });
        //
        // let mut lbl_click = button.new_label();
        // lbl_click.set_text("Click me!");
        //
        //
        // // lvgl::Timer returns reference to same internal object
        // app.tick(Duration::from_millis(1)); // takes (&mut self)
        //
        // app.task_handler(); // takes (&mut self)
        //
        // // Multiple Displays can be instantiated
        // // Multiple Screen's in each Display
        // // Screen can be instantiated independently
        // // Any specific object impl Screenable(trait) (can be used anywhere Screen is used)
        // // To get active screen from a Display use `lv_disp_get_scr_act(disp)`
        // let screen = disp.get_active_screen();
        // // To set an active screen in a Display use `lv_disp_load_scr(disp, scr)`
        // disp.load_screen(screen);
        //
        // let mut screen = Screen::new();
        // let mut button = screen.add_button();
        // let mut btn_lbl = button.create_label();
    }
}
