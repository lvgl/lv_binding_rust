use lvgl_sys::lv_coord_t;

pub const LV_SIZE_CONTENT: lv_coord_t = 2001 | lvgl_sys::_LV_COORD_TYPE_SPEC;

pub fn pct(pct: lv_coord_t) -> lv_coord_t {
    if pct > 0 {
        pct | lvgl_sys::_LV_COORD_TYPE_SPEC
    } else {
        (1000 - pct) | lvgl_sys::_LV_COORD_TYPE_SPEC
    }
}

pub fn coord_is_pct(pct: lv_coord_t) -> bool {
    (pct & lvgl_sys::_LV_COORD_TYPE_MASK == lvgl_sys::_LV_COORD_TYPE_SPEC) &&
    (pct & !lvgl_sys::lvgl_sys::_LV_COORD_TYPE_MASK <= 2000)
}

pub fn coord_get_pct(pct: lv_coord_t) -> lv_coord_t {
    (pct & !lvgl_sys::lvgl_sys::_LV_COORD_TYPE_MASK) % 1000
}
