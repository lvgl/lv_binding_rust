use crate::{LvError, NativeObject, Obj};

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
