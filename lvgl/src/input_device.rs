use crate::LvResult;
use core::mem::MaybeUninit;
use embedded_graphics::geometry::Point;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum InputData {
    Touch(Point),
    Key(u32),
}

impl InputData {
    pub fn released(self) -> InputState {
        InputState::Released(self)
    }

    pub fn pressed(self) -> InputState {
        InputState::Pressed(self)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum InputState {
    Released(InputData),
    Pressed(InputData),
}

impl InputState {
    pub fn once(self) -> BufferStatus {
        BufferStatus::Empty(self)
    }

    pub fn and_continued(self) -> BufferStatus {
        BufferStatus::Buffered(self)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum BufferStatus {
    Empty(InputState),
    Buffered(InputState),
}

pub struct Pointer<F>
where
    F: Fn() -> Option<BufferStatus>,
{
    pub(crate) driver: lvgl_sys::lv_indev_drv_t,
    pub(crate) descriptor: Option<lvgl_sys::lv_indev_t>,
    handler: F,
}

impl<F> Pointer<F>
where
    F: Fn() -> Option<BufferStatus>,
{
    pub fn new(mut handler: F) -> Self {
        let driver = unsafe {
            let mut indev_drv = MaybeUninit::uninit();
            lvgl_sys::lv_indev_drv_init(indev_drv.as_mut_ptr());
            let mut indev_drv = indev_drv.assume_init();
            indev_drv.type_ = lvgl_sys::LV_INDEV_TYPE_POINTER as lvgl_sys::lv_indev_type_t;
            indev_drv.read_cb = Some(read_input::<F>);
            indev_drv.user_data = &mut handler as *mut _ as lvgl_sys::lv_indev_drv_user_data_t;
            indev_drv
        };
        Self {
            handler,
            driver,
            descriptor: None,
        }
    }

    pub(crate) unsafe fn set_descriptor(
        &mut self,
        descriptor: *mut lvgl_sys::lv_indev_t,
    ) -> LvResult<()> {
        // TODO: check if not null && check if `self.descriptor` is not already set!
        self.descriptor = Some(*descriptor);
        Ok(())
    }
}

unsafe extern "C" fn read_input<F>(
    indev_drv: *mut lvgl_sys::lv_indev_drv_t,
    data: *mut lvgl_sys::lv_indev_data_t,
) -> bool
where
    F: Fn() -> Option<BufferStatus>,
{
    let mut data = *data;
    // convert user data to function
    let user_closure = &mut *((*indev_drv).user_data as *mut F);
    // call user data
    let result: Option<BufferStatus> = user_closure();
    return if let Some(info) = result {
        match info {
            BufferStatus::Empty(InputState::Pressed(InputData::Touch(point))) => {
                data.point.x = point.x as lvgl_sys::lv_coord_t;
                data.point.y = point.y as lvgl_sys::lv_coord_t;
                data.state = lvgl_sys::LV_INDEV_STATE_PR as lvgl_sys::lv_indev_state_t;
                false
            }
            BufferStatus::Empty(InputState::Released(InputData::Touch(point))) => {
                data.point.x = point.x as lvgl_sys::lv_coord_t;
                data.point.y = point.y as lvgl_sys::lv_coord_t;
                data.state = lvgl_sys::LV_INDEV_STATE_REL as lvgl_sys::lv_indev_state_t;
                false
            }
            BufferStatus::Buffered(InputState::Pressed(InputData::Touch(point))) => {
                data.point.x = point.x as lvgl_sys::lv_coord_t;
                data.point.y = point.y as lvgl_sys::lv_coord_t;
                data.state = lvgl_sys::LV_INDEV_STATE_PR as lvgl_sys::lv_indev_state_t;
                true
            }
            BufferStatus::Buffered(InputState::Released(InputData::Touch(point))) => {
                data.point.x = point.x as lvgl_sys::lv_coord_t;
                data.point.y = point.y as lvgl_sys::lv_coord_t;
                data.state = lvgl_sys::LV_INDEV_STATE_REL as lvgl_sys::lv_indev_state_t;
                true
            }
            BufferStatus::Empty(InputState::Released(InputData::Key(_))) => false,
            BufferStatus::Empty(InputState::Pressed(InputData::Key(_))) => false,
            BufferStatus::Buffered(InputState::Released(InputData::Key(_))) => true,
            BufferStatus::Buffered(InputState::Pressed(InputData::Key(_))) => true,
        }
    } else {
        false
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Color, UI};
    use core::marker::PhantomData;
    use embedded_graphics::drawable::Pixel;
    use embedded_graphics::geometry::Size;
    use embedded_graphics::pixelcolor::PixelColor;
    use embedded_graphics::pixelcolor::Rgb565;
    use embedded_graphics::DrawTarget;

    struct FakeDisplay<C>
    where
        C: PixelColor + From<Color>,
    {
        p: PhantomData<C>,
    }

    impl<C> DrawTarget<C> for FakeDisplay<C>
    where
        C: PixelColor + From<Color>,
    {
        type Error = ();

        fn draw_pixel(&mut self, item: Pixel<C>) -> Result<(), Self::Error> {
            Ok(())
        }

        fn size(&self) -> Size {
            Size::new(crate::VER_RES_MAX, crate::HOR_RES_MAX)
        }
    }

    //#[test]
    // We cannot test right now by having instances of UI global state... :(
    // I need to find a way to test while having global state...
    fn pointer_input_device() -> LvResult<()> {
        let mut ui = UI::init()?;

        let disp: FakeDisplay<Rgb565> = FakeDisplay { p: PhantomData };

        ui.disp_drv_register(disp);

        fn read_touchpad_device() -> Point {
            Point::new(120, 23)
        }

        let mut touch_screen =
            Pointer::new(|| InputData::Touch(read_touchpad_device()).pressed().once());

        ui.indev_drv_register(&mut touch_screen)?;

        Ok(())
    }
}
