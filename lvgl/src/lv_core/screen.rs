use crate::{LvError, NativeObject, Obj, Part, Widget};

/// An LVGL screen.
#[derive(Debug)]
pub struct Screen {
    raw: Obj,
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            raw: Obj::default(),
        }
    }
}

impl NativeObject for Screen {
    fn raw(&self) -> crate::LvResult<core::ptr::NonNull<lvgl_sys::lv_obj_t>> {
        self.raw.raw()
    }
}

impl Widget for Screen {
    type SpecialEvent = u32;
    type Part = Part;

    unsafe fn from_raw(raw: core::ptr::NonNull<lvgl_sys::lv_obj_t>) -> Option<Self> {
        match Self::try_from(Obj::from_raw(raw)?) {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }
}

impl TryFrom<Obj> for Screen {
    type Error = LvError;

    fn try_from(value: Obj) -> Result<Self, Self::Error> {
        match unsafe { (*value.raw()?.as_mut()).parent } as usize {
            0 => Ok(Self { raw: value }),
            _ => Err(LvError::InvalidReference),
        }
    }
}

impl Into<Obj> for Screen {
    fn into(self) -> Obj {
        self.raw
    }
}

impl AsRef<Obj> for Screen {
    fn as_ref(&self) -> &Obj {
        &self.raw
    }
}

impl AsMut<Obj> for Screen {
    fn as_mut(&mut self) -> &mut Obj {
        &mut self.raw
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Display, DrawBuffer};

    #[test]
    fn screen_test() {
        const HOR_RES: u32 = 240;
        const VER_RES: u32 = 240;
        crate::tests::initialize_test(false);
        let buffer = DrawBuffer::<{ (HOR_RES * VER_RES) as usize }>::default();
        let mut display = Display::register(buffer, HOR_RES, VER_RES, |_| {}).unwrap();
        let mut screen_old = display.get_scr_act().unwrap();
        let mut screen_new = Screen::default();
        display.set_scr_act(&mut screen_new).unwrap();
        display.set_scr_act(&mut screen_old).unwrap();
    }
}
