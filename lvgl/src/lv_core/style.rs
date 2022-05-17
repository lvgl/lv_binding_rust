use crate::Box;
use crate::{Color, LvResult, State};
use core::mem;
use cstr_core::CStr;

pub enum Themes {
    Pretty,
}

#[derive(Clone)]
pub struct Style {
    pub(crate) raw: Box<lvgl_sys::lv_style_t>,
}

impl Style {
    pub fn set_value_str(&mut self, state: State, value: &CStr) -> LvResult<()> {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_ptr(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_VALUE_STR as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.as_ptr() as *mut cty::c_void,
            );
        }
        Ok(())
    }
}

impl Default for Style {
    fn default() -> Self {
        let raw = unsafe {
            let mut style = mem::MaybeUninit::<lvgl_sys::lv_style_t>::uninit();
            lvgl_sys::lv_style_init(style.as_mut_ptr());
            Box::new(style.assume_init())
        };
        Self { raw }
    }
}

bitflags! {
    pub struct Opacity: u32 {
        const OPA_TRANSP = lvgl_sys::LV_OPA_TRANSP as u32;
        const OPA_0 = lvgl_sys::LV_OPA_0 as u32;
        const OPA_10 = lvgl_sys::LV_OPA_10 as u32;
        const OPA_20 = lvgl_sys::LV_OPA_20 as u32;
        const OPA_30 = lvgl_sys::LV_OPA_30 as u32;
        const OPA_40 = lvgl_sys::LV_OPA_40 as u32;
        const OPA_50 = lvgl_sys::LV_OPA_50 as u32;
        const OPA_60 = lvgl_sys::LV_OPA_60 as u32;
        const OPA_70 = lvgl_sys::LV_OPA_70 as u32;
        const OPA_80 = lvgl_sys::LV_OPA_80 as u32;
        const OPA_90 = lvgl_sys::LV_OPA_90 as u32;
        const OPA_100 = lvgl_sys::LV_OPA_100 as u32;
        const OPA_COVER = lvgl_sys::LV_OPA_COVER as u32;
    }
}

impl Into<u8> for Opacity {
    fn into(self) -> u8 {
        self.bits as u8
    }
}

bitflags! {
    pub struct StyleProp: u32 {
        const RADIUS = lvgl_sys::LV_STYLE_RADIUS as u32;
        const CLIP_CORNER = lvgl_sys::LV_STYLE_CLIP_CORNER as u32;
        const SIZE = lvgl_sys::LV_STYLE_SIZE as u32;
        const TRANSFORM_WIDTH = lvgl_sys::LV_STYLE_TRANSFORM_WIDTH as u32;
        const TRANSFORM_HEIGHT = lvgl_sys::LV_STYLE_TRANSFORM_HEIGHT as u32;
        const TRANSFORM_ANGLE = lvgl_sys::LV_STYLE_TRANSFORM_ANGLE as u32;
        const TRANSFORM_ZOOM = lvgl_sys::LV_STYLE_TRANSFORM_ZOOM as u32;
        const OPA_SCALE = lvgl_sys::LV_STYLE_OPA_SCALE as u32;
        const PAD_TOP = lvgl_sys::LV_STYLE_PAD_TOP as u32;
        const PAD_BOTTOM = lvgl_sys::LV_STYLE_PAD_BOTTOM as u32;
        const PAD_LEFT = lvgl_sys::LV_STYLE_PAD_LEFT as u32;
        const PAD_RIGHT = lvgl_sys::LV_STYLE_PAD_RIGHT as u32;
        const PAD_INNER = lvgl_sys::LV_STYLE_PAD_INNER as u32;
        const MARGIN_TOP = lvgl_sys::LV_STYLE_MARGIN_TOP as u32;
        const MARGIN_BOTTOM = lvgl_sys::LV_STYLE_MARGIN_BOTTOM as u32;
        const MARGIN_LEFT = lvgl_sys::LV_STYLE_MARGIN_LEFT as u32;
        const MARGIN_RIGHT = lvgl_sys::LV_STYLE_MARGIN_RIGHT as u32;
        const BG_BLEND_MODE = lvgl_sys::LV_STYLE_BG_BLEND_MODE as u32;
        const BG_MAIN_STOP = lvgl_sys::LV_STYLE_BG_MAIN_STOP as u32;
        const BG_GRAD_STOP = lvgl_sys::LV_STYLE_BG_GRAD_STOP as u32;
        const BG_GRAD_DIR = lvgl_sys::LV_STYLE_BG_GRAD_DIR as u32;
        const BG_COLOR = lvgl_sys::LV_STYLE_BG_COLOR as u32;
        const BG_GRAD_COLOR = lvgl_sys::LV_STYLE_BG_GRAD_COLOR as u32;
        const BG_OPA = lvgl_sys::LV_STYLE_BG_OPA as u32;
        const BORDER_WIDTH = lvgl_sys::LV_STYLE_BORDER_WIDTH as u32;
        const BORDER_SIDE = lvgl_sys::LV_STYLE_BORDER_SIDE as u32;
        const BORDER_BLEND_MODE = lvgl_sys::LV_STYLE_BORDER_BLEND_MODE as u32;
        const BORDER_POST = lvgl_sys::LV_STYLE_BORDER_POST as u32;
        const BORDER_COLOR = lvgl_sys::LV_STYLE_BORDER_COLOR as u32;
        const BORDER_OPA = lvgl_sys::LV_STYLE_BORDER_OPA as u32;
        const OUTLINE_WIDTH = lvgl_sys::LV_STYLE_OUTLINE_WIDTH as u32;
        const OUTLINE_PAD = lvgl_sys::LV_STYLE_OUTLINE_PAD as u32;
        const OUTLINE_BLEND_MODE = lvgl_sys::LV_STYLE_OUTLINE_BLEND_MODE as u32;
        const OUTLINE_COLOR = lvgl_sys::LV_STYLE_OUTLINE_COLOR as u32;
        const OUTLINE_OPA = lvgl_sys::LV_STYLE_OUTLINE_OPA as u32;
        const SHADOW_WIDTH = lvgl_sys::LV_STYLE_SHADOW_WIDTH as u32;
        const SHADOW_OFS_X = lvgl_sys::LV_STYLE_SHADOW_OFS_X as u32;
        const SHADOW_OFS_Y = lvgl_sys::LV_STYLE_SHADOW_OFS_Y as u32;
        const SHADOW_SPREAD = lvgl_sys::LV_STYLE_SHADOW_SPREAD as u32;
        const SHADOW_BLEND_MODE = lvgl_sys::LV_STYLE_SHADOW_BLEND_MODE as u32;
        const SHADOW_COLOR = lvgl_sys::LV_STYLE_SHADOW_COLOR as u32;
        const SHADOW_OPA = lvgl_sys::LV_STYLE_SHADOW_OPA as u32;
        const PATTERN_BLEND_MODE = lvgl_sys::LV_STYLE_PATTERN_BLEND_MODE as u32;
        const PATTERN_REPEAT = lvgl_sys::LV_STYLE_PATTERN_REPEAT as u32;
        const PATTERN_RECOLOR = lvgl_sys::LV_STYLE_PATTERN_RECOLOR as u32;
        const PATTERN_OPA = lvgl_sys::LV_STYLE_PATTERN_OPA as u32;
        const PATTERN_RECOLOR_OPA = lvgl_sys::LV_STYLE_PATTERN_RECOLOR_OPA as u32;
        const PATTERN_IMAGE = lvgl_sys::LV_STYLE_PATTERN_IMAGE as u32;
        const VALUE_LETTER_SPACE = lvgl_sys::LV_STYLE_VALUE_LETTER_SPACE as u32;
        const VALUE_LINE_SPACE = lvgl_sys::LV_STYLE_VALUE_LINE_SPACE as u32;
        const VALUE_BLEND_MODE = lvgl_sys::LV_STYLE_VALUE_BLEND_MODE as u32;
        const VALUE_OFS_X = lvgl_sys::LV_STYLE_VALUE_OFS_X as u32;
        const VALUE_OFS_Y = lvgl_sys::LV_STYLE_VALUE_OFS_Y as u32;
        const VALUE_ALIGN = lvgl_sys::LV_STYLE_VALUE_ALIGN as u32;
        const VALUE_COLOR = lvgl_sys::LV_STYLE_VALUE_COLOR as u32;
        const VALUE_OPA = lvgl_sys::LV_STYLE_VALUE_OPA as u32;
        const VALUE_FONT = lvgl_sys::LV_STYLE_VALUE_FONT as u32;
        const VALUE_STR = lvgl_sys::LV_STYLE_VALUE_STR as u32;
        const TEXT_LETTER_SPACE = lvgl_sys::LV_STYLE_TEXT_LETTER_SPACE as u32;
        const TEXT_LINE_SPACE = lvgl_sys::LV_STYLE_TEXT_LINE_SPACE as u32;
        const TEXT_DECOR = lvgl_sys::LV_STYLE_TEXT_DECOR as u32;
        const TEXT_BLEND_MODE = lvgl_sys::LV_STYLE_TEXT_BLEND_MODE as u32;
        const TEXT_COLOR = lvgl_sys::LV_STYLE_TEXT_COLOR as u32;
        const TEXT_SEL_COLOR = lvgl_sys::LV_STYLE_TEXT_SEL_COLOR as u32;
        const TEXT_OPA = lvgl_sys::LV_STYLE_TEXT_OPA as u32;
        const TEXT_FONT = lvgl_sys::LV_STYLE_TEXT_FONT as u32;
        const LINE_WIDTH = lvgl_sys::LV_STYLE_LINE_WIDTH as u32;
        const LINE_BLEND_MODE = lvgl_sys::LV_STYLE_LINE_BLEND_MODE as u32;
        const LINE_DASH_WIDTH = lvgl_sys::LV_STYLE_LINE_DASH_WIDTH as u32;
        const LINE_DASH_GAP = lvgl_sys::LV_STYLE_LINE_DASH_GAP as u32;
        const LINE_ROUNDED = lvgl_sys::LV_STYLE_LINE_ROUNDED as u32;
        const LINE_COLOR = lvgl_sys::LV_STYLE_LINE_COLOR as u32;
        const LINE_OPA = lvgl_sys::LV_STYLE_LINE_OPA as u32;
        const IMAGE_BLEND_MODE = lvgl_sys::LV_STYLE_IMAGE_BLEND_MODE as u32;
        const IMAGE_RECOLOR = lvgl_sys::LV_STYLE_IMAGE_RECOLOR as u32;
        const IMAGE_OPA = lvgl_sys::LV_STYLE_IMAGE_OPA as u32;
        const IMAGE_RECOLOR_OPA = lvgl_sys::LV_STYLE_IMAGE_RECOLOR_OPA as u32;
        const TRANSITION_TIME = lvgl_sys::LV_STYLE_TRANSITION_TIME as u32;
        const TRANSITION_DELAY = lvgl_sys::LV_STYLE_TRANSITION_DELAY as u32;
        const TRANSITION_PROP_1 = lvgl_sys::LV_STYLE_TRANSITION_PROP_1 as u32;
        const TRANSITION_PROP_2 = lvgl_sys::LV_STYLE_TRANSITION_PROP_2 as u32;
        const TRANSITION_PROP_3 = lvgl_sys::LV_STYLE_TRANSITION_PROP_3 as u32;
        const TRANSITION_PROP_4 = lvgl_sys::LV_STYLE_TRANSITION_PROP_4 as u32;
        const TRANSITION_PROP_5 = lvgl_sys::LV_STYLE_TRANSITION_PROP_5 as u32;
        const TRANSITION_PROP_6 = lvgl_sys::LV_STYLE_TRANSITION_PROP_6 as u32;
        const TRANSITION_PATH = lvgl_sys::LV_STYLE_TRANSITION_PATH as u32;
        const SCALE_WIDTH = lvgl_sys::LV_STYLE_SCALE_WIDTH as u32;
        const SCALE_BORDER_WIDTH = lvgl_sys::LV_STYLE_SCALE_BORDER_WIDTH as u32;
        const SCALE_END_BORDER_WIDTH = lvgl_sys::LV_STYLE_SCALE_END_BORDER_WIDTH as u32;
        const SCALE_END_LINE_WIDTH = lvgl_sys::LV_STYLE_SCALE_END_LINE_WIDTH as u32;
        const SCALE_GRAD_COLOR = lvgl_sys::LV_STYLE_SCALE_GRAD_COLOR as u32;
        const SCALE_END_COLOR = lvgl_sys::LV_STYLE_SCALE_END_COLOR as u32;
    }
}

// Auto-gen code, please look into lvgl-codegen for any changes.
impl Style {
    pub fn set_radius(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_RADIUS as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_clip_corner(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_CLIP_CORNER as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_size(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SIZE as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_transform_width(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSFORM_WIDTH as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_transform_height(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSFORM_HEIGHT as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_transform_angle(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSFORM_ANGLE as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_transform_zoom(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSFORM_ZOOM as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_opa_scale(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_OPA_SCALE as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_pad_top(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_PAD_TOP as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_pad_bottom(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_PAD_BOTTOM as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_pad_left(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_PAD_LEFT as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_pad_right(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_PAD_RIGHT as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_pad_inner(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_PAD_INNER as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_margin_top(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_MARGIN_TOP as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_margin_bottom(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_MARGIN_BOTTOM as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_margin_left(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_MARGIN_LEFT as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_margin_right(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_MARGIN_RIGHT as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_bg_blend_mode(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BG_BLEND_MODE as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_bg_main_stop(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BG_MAIN_STOP as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_bg_grad_stop(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BG_GRAD_STOP as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_bg_grad_dir(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BG_GRAD_DIR as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_bg_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BG_COLOR as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_bg_grad_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BG_GRAD_COLOR as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_bg_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BG_OPA as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_border_width(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BORDER_WIDTH as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_border_side(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BORDER_SIDE as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_border_blend_mode(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BORDER_BLEND_MODE as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_border_post(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BORDER_POST as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_border_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BORDER_COLOR as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_border_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BORDER_OPA as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_outline_width(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_OUTLINE_WIDTH as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_outline_pad(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_OUTLINE_PAD as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_outline_blend_mode(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_OUTLINE_BLEND_MODE as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_outline_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_OUTLINE_COLOR as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_outline_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_OUTLINE_OPA as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_shadow_width(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SHADOW_WIDTH as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_shadow_ofs_x(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SHADOW_OFS_X as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_shadow_ofs_y(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SHADOW_OFS_Y as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_shadow_spread(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SHADOW_SPREAD as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_shadow_blend_mode(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SHADOW_BLEND_MODE as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_shadow_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SHADOW_COLOR as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_shadow_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SHADOW_OPA as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_pattern_repeat(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_PATTERN_REPEAT as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_pattern_blend_mode(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_PATTERN_BLEND_MODE as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_pattern_recolor(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_PATTERN_RECOLOR as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_pattern_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_PATTERN_OPA as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_pattern_recolor_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_PATTERN_RECOLOR_OPA as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_value_letter_space(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_VALUE_LETTER_SPACE as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_value_line_space(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_VALUE_LINE_SPACE as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_value_blend_mode(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_VALUE_BLEND_MODE as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_value_ofs_x(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_VALUE_OFS_X as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_value_ofs_y(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_VALUE_OFS_Y as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_value_align(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_VALUE_ALIGN as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_value_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_VALUE_COLOR as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_value_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_VALUE_OPA as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_text_letter_space(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TEXT_LETTER_SPACE as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_text_line_space(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TEXT_LINE_SPACE as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_text_decor(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TEXT_DECOR as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_text_blend_mode(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TEXT_BLEND_MODE as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_text_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TEXT_COLOR as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_text_sel_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TEXT_SEL_COLOR as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_text_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TEXT_OPA as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_line_width(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_LINE_WIDTH as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_line_blend_mode(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_LINE_BLEND_MODE as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_line_dash_width(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_LINE_DASH_WIDTH as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_line_dash_gap(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_LINE_DASH_GAP as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_line_rounded(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_LINE_ROUNDED as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_line_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_LINE_COLOR as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_line_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_LINE_OPA as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_image_blend_mode(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_IMAGE_BLEND_MODE as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_image_recolor(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_IMAGE_RECOLOR as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_image_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_IMAGE_OPA as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_image_recolor_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_IMAGE_RECOLOR_OPA as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_transition_time(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSITION_TIME as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_transition_delay(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSITION_DELAY as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_transition_prop_1(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSITION_PROP_1 as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_transition_prop_2(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSITION_PROP_2 as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_transition_prop_3(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSITION_PROP_3 as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_transition_prop_4(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSITION_PROP_4 as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_transition_prop_5(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSITION_PROP_5 as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_transition_prop_6(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSITION_PROP_6 as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_scale_width(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SCALE_WIDTH as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_scale_border_width(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SCALE_BORDER_WIDTH as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_scale_end_border_width(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SCALE_END_BORDER_WIDTH as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_scale_end_line_width(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SCALE_END_LINE_WIDTH as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_scale_grad_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SCALE_GRAD_COLOR as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_scale_end_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SCALE_END_COLOR as u32
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS)) as u16,
                value.raw,
            );
        }
    }
}
