use crate::{Part, LvError, NativeObject, Obj, Widget};

/// An LVGL screen.
pub struct Screen {
    raw: Obj
}

impl Default for Screen {
    fn default() -> Self {
        Self { raw: Obj::default() }
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

    fn from_raw(raw: core::ptr::NonNull<lvgl_sys::lv_obj_t>) -> Option<Self> {
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
