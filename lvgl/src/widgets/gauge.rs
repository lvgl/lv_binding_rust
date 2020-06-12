pub enum GaugePart {
    Main,
    Major,
    Needle,
}

impl From<GaugePart> for u8 {
    fn from(part: GaugePart) -> Self {
        match part {
            GaugePart::Main => lvgl_sys::LV_GAUGE_PART_MAIN as u8,
            GaugePart::Major => lvgl_sys::LV_GAUGE_PART_MAJOR as u8,
            GaugePart::Needle => lvgl_sys::LV_GAUGE_PART_NEEDLE as u8,
        }
    }
}
