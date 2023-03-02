use super::generic::{BufferStatus, Data, InputDriver, InputState};
use crate::Box;
use crate::{LvError, LvResult};
use core::mem::MaybeUninit;
use embedded_graphics::geometry::Point;

/// Pointer-specific input data. Contains the point clicked and the key.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum PointerInputData {
    Touch(Point),
    Key(u32),
}

impl PointerInputData {
    pub fn released(self) -> InputState {
        InputState::Released(Data::Pointer(self))
    }

    pub fn pressed(self) -> InputState {
        InputState::Pressed(Data::Pointer(self))
    }
}

/// Represents a pointer-type input driver.
pub struct Pointer {
    pub(crate) driver: Box<lvgl_sys::lv_indev_drv_t>,
    pub(crate) descriptor: Option<*mut lvgl_sys::lv_indev_t>,
}

impl InputDriver<Pointer> for Pointer {
    fn new<F>(handler: F) -> Self
    where
        F: Fn() -> BufferStatus,
    {
        let driver = unsafe {
            let mut indev_drv = MaybeUninit::uninit();
            lvgl_sys::lv_indev_drv_init(indev_drv.as_mut_ptr());
            let mut indev_drv = Box::new(indev_drv.assume_init());
            indev_drv.type_ = lvgl_sys::lv_indev_type_t_LV_INDEV_TYPE_POINTER;
            indev_drv.read_cb = Some(read_input::<F>);
            indev_drv.feedback_cb = Some(feedback);
            indev_drv.user_data = Box::into_raw(Box::new(handler)) as *mut _;
            indev_drv
        };
        Self {
            driver,
            descriptor: None,
        }
    }

    fn get_driver(&mut self) -> &mut lvgl_sys::lv_indev_drv_t {
        self.driver.as_mut()
    }

    unsafe fn set_descriptor(&mut self, descriptor: *mut lvgl_sys::lv_indev_t) -> LvResult<()> {
        if self.descriptor.is_none() {
            self.descriptor = Some(descriptor);
        } else {
            return Err(LvError::AlreadyInUse);
        }
        Ok(())
    }
}

unsafe extern "C" fn read_input<F>(
    indev_drv: *mut lvgl_sys::lv_indev_drv_t,
    data: *mut lvgl_sys::lv_indev_data_t,
) where
    F: Fn() -> BufferStatus,
{
    // convert user data to function
    let user_closure = &mut *((*indev_drv).user_data as *mut F);
    // call user data
    let info: BufferStatus = user_closure();
    unsafe {
        (*data).continue_reading = match info {
            BufferStatus::Once(InputState::Pressed(Data::Pointer(PointerInputData::Touch(
                point,
            )))) => {
                (*data).point.x = point.x as lvgl_sys::lv_coord_t;
                (*data).point.y = point.y as lvgl_sys::lv_coord_t;
                (*data).state =
                    lvgl_sys::lv_indev_state_t_LV_INDEV_STATE_PRESSED as lvgl_sys::lv_indev_state_t;
                false
            }
            BufferStatus::Once(InputState::Released(Data::Pointer(PointerInputData::Touch(
                point,
            )))) => {
                (*data).point.x = point.x as lvgl_sys::lv_coord_t;
                (*data).point.y = point.y as lvgl_sys::lv_coord_t;
                (*data).state = lvgl_sys::lv_indev_state_t_LV_INDEV_STATE_RELEASED
                    as lvgl_sys::lv_indev_state_t;
                false
            }
            BufferStatus::Buffered(InputState::Pressed(Data::Pointer(
                PointerInputData::Touch(point),
            ))) => {
                (*data).point.x = point.x as lvgl_sys::lv_coord_t;
                (*data).point.y = point.y as lvgl_sys::lv_coord_t;
                (*data).state =
                    lvgl_sys::lv_indev_state_t_LV_INDEV_STATE_PRESSED as lvgl_sys::lv_indev_state_t;
                true
            }
            BufferStatus::Buffered(InputState::Released(Data::Pointer(
                PointerInputData::Touch(point),
            ))) => {
                (*data).point.x = point.x as lvgl_sys::lv_coord_t;
                (*data).point.y = point.y as lvgl_sys::lv_coord_t;
                (*data).state = lvgl_sys::lv_indev_state_t_LV_INDEV_STATE_RELEASED
                    as lvgl_sys::lv_indev_state_t;
                true
            }
            BufferStatus::Once(InputState::Released(Data::Pointer(PointerInputData::Key(_)))) => {
                false
            }
            BufferStatus::Once(InputState::Pressed(Data::Pointer(PointerInputData::Key(_)))) => {
                false
            }
            BufferStatus::Buffered(InputState::Released(Data::Pointer(PointerInputData::Key(
                _,
            )))) => true,
            BufferStatus::Buffered(InputState::Pressed(Data::Pointer(PointerInputData::Key(
                _,
            )))) => true,
        }
    }
}

unsafe extern "C" fn feedback(_indev_drv: *mut lvgl_sys::lv_indev_drv_t, _code: u8) {}

#[cfg(test)]
mod test {
    //use super::*;
    use core::marker::PhantomData;
    use embedded_graphics::draw_target::DrawTarget;
    use embedded_graphics::geometry::Size;
    use embedded_graphics::pixelcolor::PixelColor;
    //use embedded_graphics::pixelcolor::Rgb565;
    use embedded_graphics::prelude::OriginDimensions;
    use embedded_graphics::Pixel;

    struct FakeDisplay<C>
    where
        C: PixelColor,
    {
        p: PhantomData<C>,
    }

    impl<C> DrawTarget for FakeDisplay<C>
    where
        C: PixelColor,
    {
        type Color = C;
        type Error = ();

        fn draw_iter<I>(&mut self, _pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Pixel<Self::Color>>,
        {
            Ok(())
        }
    }

    impl<C> OriginDimensions for FakeDisplay<C>
    where
        C: PixelColor,
    {
        fn size(&self) -> Size {
            Size::new(240, 240)
        }
    }

    //#[test]
    // We cannot test right now by having instances of UI global state... :(
    // I need to find a way to test while having global state...
    /*
    fn pointer_input_device() -> LvResult<()> {
        crate::init();

        //FIXME
        let disp: FakeDisplay<Rgb565> = FakeDisplay { p: PhantomData };

        //ui.disp_drv_register(disp)?;

        fn read_touchpad_device() -> BufferStatus {
            PointerInputData::Touch(Point::new(120, 23))
                .pressed()
                .once()
        }

        let mut touch_screen = Pointer::new(|| read_touchpad_device());

        crate::indev_drv_register(&mut touch_screen)?;

        Ok(())
    }
    */
}
