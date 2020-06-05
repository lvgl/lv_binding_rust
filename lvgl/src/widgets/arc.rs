use crate::NativeObject;

define_object!(Arc, lv_arc_create, part = ArcPart);

impl Arc {
    /// Set the start angle, for the given arc part.
    /// 0 degrees for the right, 90 degrees for the bottom, etc.
    pub fn set_start_angle(&mut self, angle: u16, part: ArcPart) {
        match part {
            ArcPart::Background => unsafe {
                lvgl_sys::lv_arc_set_bg_start_angle(self.core.raw().as_mut(), angle)
            },
            ArcPart::Indicator => unsafe {
                lvgl_sys::lv_arc_set_start_angle(self.core.raw().as_mut(), angle)
            },
        }
    }

    /// Set the end angle, for the given arc part.
    /// 0 degrees for the right, 90 degrees for the bottom, etc.
    pub fn set_end_angle(&mut self, angle: u16, part: ArcPart) {
        match part {
            ArcPart::Background => unsafe {
                lvgl_sys::lv_arc_set_bg_start_angle(self.core.raw().as_mut(), angle)
            },
            ArcPart::Indicator => unsafe {
                lvgl_sys::lv_arc_set_end_angle(self.core.raw().as_mut(), angle)
            },
        }
    }

    /// Rotate the arc, `angle` degrees clockwise.
    pub fn set_rotation(&mut self, angle: u16) {
        unsafe {
            lvgl_sys::lv_arc_set_rotation(self.core.raw().as_mut(), angle);
        }
    }
}

/// The different parts, of an arc object.
pub enum ArcPart {
    /// The background of the arc.
    Background,
    /// The indicator of the arc.
    /// This is what moves/changes, depending on the arc's value.
    Indicator,
}

impl From<ArcPart> for u8 {
    fn from(component: ArcPart) -> Self {
        match component {
            ArcPart::Background => lvgl_sys::LV_ARC_PART_BG as u8,
            ArcPart::Indicator => lvgl_sys::LV_ARC_PART_INDIC as u8,
        }
    }
}
