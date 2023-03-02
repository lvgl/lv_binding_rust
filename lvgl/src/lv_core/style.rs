use crate::{Box, Color, TextAlign};
use core::mem;
use cty::c_uint;
use paste::paste;

pub enum Themes {
    Pretty,
}

#[derive(Clone)]
pub struct Style {
    pub(crate) raw: Box<lvgl_sys::lv_style_t>,
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

impl From<Opacity> for u8 {
    fn from(self_: Opacity) -> u8 {
        self_.bits as u8
    }
}

bitflags! {
    pub struct StyleProp: u32 {
        const PROP_INV = lvgl_sys::lv_style_prop_t_LV_STYLE_PROP_INV as u32;

        /*Group 0*/
        const WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_WIDTH as u32;
        const MIN_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_MIN_WIDTH as u32;
        const MAX_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_MAX_WIDTH as u32;
        const HEIGHT = lvgl_sys::lv_style_prop_t_LV_STYLE_HEIGHT as u32;
        const MIN_HEIGHT = lvgl_sys::lv_style_prop_t_LV_STYLE_MIN_HEIGHT as u32;
        const MAX_HEIGHT = lvgl_sys::lv_style_prop_t_LV_STYLE_MAX_HEIGHT as u32;
        const X = lvgl_sys::lv_style_prop_t_LV_STYLE_X as u32;
        const Y = lvgl_sys::lv_style_prop_t_LV_STYLE_Y as u32;
        const ALIGN = lvgl_sys::lv_style_prop_t_LV_STYLE_ALIGN as u32;
        const TRANSFORM_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_TRANSFORM_WIDTH as u32;
        const TRANSFORM_HEIGHT = lvgl_sys::lv_style_prop_t_LV_STYLE_TRANSFORM_HEIGHT as u32;
        const TRANSLATE_X = lvgl_sys::lv_style_prop_t_LV_STYLE_TRANSLATE_X as u32;
        const TRANSLATE_Y = lvgl_sys::lv_style_prop_t_LV_STYLE_TRANSLATE_Y as u32;
        const TRANSFORM_ZOOM = lvgl_sys::lv_style_prop_t_LV_STYLE_TRANSFORM_ZOOM as u32;
        const TRANSFORM_ANGLE = lvgl_sys::lv_style_prop_t_LV_STYLE_TRANSFORM_ANGLE as u32;

        /*Group 1*/
        const PAD_TOP = lvgl_sys::lv_style_prop_t_LV_STYLE_PAD_TOP as u32;
        const PAD_BOTTOM = lvgl_sys::lv_style_prop_t_LV_STYLE_PAD_BOTTOM as u32;
        const PAD_LEFT = lvgl_sys::lv_style_prop_t_LV_STYLE_PAD_LEFT as u32;
        const PAD_RIGHT = lvgl_sys::lv_style_prop_t_LV_STYLE_PAD_RIGHT as u32;
        const PAD_ROW = lvgl_sys::lv_style_prop_t_LV_STYLE_PAD_ROW as u32;
        const PAD_COLUMN = lvgl_sys::lv_style_prop_t_LV_STYLE_PAD_COLUMN as u32;

        /*Group 2*/
        const BG_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_COLOR as u32;
        //const BG_COLOR_FILTERED = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_COLOR_FILTERED as u32;
        const BG_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_OPA as u32;
        const BG_GRAD_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_GRAD_COLOR as u32;
        //const BG_GRAD_COLOR_FILTERED = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_GRAD_COLOR_FILTERED as u32;
        const BG_GRAD_DIR = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_GRAD_DIR as u32;
        const BG_MAIN_STOP = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_MAIN_STOP as u32;
        const BG_GRAD_STOP = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_GRAD_STOP as u32;

        const BG_IMG_SRC = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_IMG_SRC as u32;
        const BG_IMG_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_IMG_OPA as u32;
        const BG_IMG_RECOLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_IMG_RECOLOR as u32;
        //const BG_IMG_RECOLOR_FILTERED = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_IMG_RECOLOR_FILTERED as u32;
        const BG_IMG_RECOLOR_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_IMG_RECOLOR_OPA as u32;
        const BG_IMG_TILED = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_IMG_TILED as u32;

        /*Group 3*/
        const BORDER_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_BORDER_COLOR as u32;
        //const BORDER_COLOR_FILTERED = lvgl_sys::lv_style_prop_t_LV_STYLE_BORDER_COLOR_FILTERED as u32;
        const BORDER_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_BORDER_OPA as u32;
        const BORDER_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_BORDER_WIDTH as u32;
        const BORDER_SIDE = lvgl_sys::lv_style_prop_t_LV_STYLE_BORDER_SIDE as u32;
        const BORDER_POST = lvgl_sys::lv_style_prop_t_LV_STYLE_BORDER_POST as u32;

        const OUTLINE_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_OUTLINE_WIDTH as u32;
        const OUTLINE_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_OUTLINE_COLOR as u32;
        //const OUTLINE_COLOR_FILTERED = lvgl_sys::lv_style_prop_t_LV_STYLE_OUTLINE_COLOR_FILTERED as u32;
        const OUTLINE_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_OUTLINE_OPA as u32;
        const OUTLINE_PAD = lvgl_sys::lv_style_prop_t_LV_STYLE_OUTLINE_PAD as u32;

        /*Group 4*/
        const SHADOW_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_WIDTH as u32;
        const SHADOW_OFS_X = lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_OFS_X as u32;
        const SHADOW_OFS_Y = lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_OFS_Y as u32;
        const SHADOW_SPREAD = lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_SPREAD as u32;
        const SHADOW_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_COLOR as u32;
        //const SHADOW_COLOR_FILTERED = lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_COLOR_FILTERED as u32;
        const SHADOW_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_OPA as u32;

        const IMG_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_IMG_OPA as u32;
        const IMG_RECOLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_IMG_RECOLOR as u32;
        //const IMG_RECOLOR_FILTERED = lvgl_sys::lv_style_prop_t_LV_STYLE_IMG_RECOLOR_FILTERED as u32;
        const IMG_RECOLOR_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_IMG_RECOLOR_OPA as u32;

        const LINE_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_LINE_WIDTH as u32;
        const LINE_DASH_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_LINE_DASH_WIDTH as u32;
        const LINE_DASH_GAP = lvgl_sys::lv_style_prop_t_LV_STYLE_LINE_DASH_GAP as u32;
        const LINE_ROUNDED = lvgl_sys::lv_style_prop_t_LV_STYLE_LINE_ROUNDED as u32;
        const LINE_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_LINE_COLOR as u32;
        //const LINE_COLOR_FILTERED = lvgl_sys::lv_style_prop_t_LV_STYLE_LINE_COLOR_FILTERED as u32;
        const LINE_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_LINE_OPA as u32;

        /*Group 5*/
        const ARC_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_ARC_WIDTH as u32;
        const ARC_ROUNDED = lvgl_sys::lv_style_prop_t_LV_STYLE_ARC_ROUNDED as u32;
        const ARC_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_ARC_COLOR as u32;
        //const ARC_COLOR_FILTERED = lvgl_sys::lv_style_prop_t_LV_STYLE_ARC_COLOR_FILTERED as u32;
        const ARC_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_ARC_OPA as u32;
        const ARC_IMG_SRC = lvgl_sys::lv_style_prop_t_LV_STYLE_ARC_IMG_SRC as u32;

        const TEXT_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_COLOR as u32;
        //const TEXT_COLOR_FILTERED = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_COLOR_FILTERED as u32;
        const TEXT_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_OPA as u32;
        const TEXT_FONT = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_FONT as u32;
        const TEXT_LETTER_SPACE = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_LETTER_SPACE as u32;
        const TEXT_LINE_SPACE = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_LINE_SPACE as u32;
        const TEXT_DECOR = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_DECOR as u32;
        const TEXT_ALIGN = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_ALIGN as u32;

        /*Group 6*/
        const RADIUS = lvgl_sys::lv_style_prop_t_LV_STYLE_RADIUS as u32;
        const CLIP_CORNER = lvgl_sys::lv_style_prop_t_LV_STYLE_CLIP_CORNER as u32;
        const OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_OPA as u32;
        const COLOR_FILTER_DSC = lvgl_sys::lv_style_prop_t_LV_STYLE_COLOR_FILTER_DSC as u32;
        const COLOR_FILTER_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_COLOR_FILTER_OPA as u32;
        const ANIM_TIME = lvgl_sys::lv_style_prop_t_LV_STYLE_ANIM_TIME as u32;
        const ANIM_SPEED = lvgl_sys::lv_style_prop_t_LV_STYLE_ANIM_SPEED as u32;
        const TRANSITION = lvgl_sys::lv_style_prop_t_LV_STYLE_TRANSITION as u32;
        const BLEND_MODE = lvgl_sys::lv_style_prop_t_LV_STYLE_BLEND_MODE as u32;
        const LAYOUT = lvgl_sys::lv_style_prop_t_LV_STYLE_LAYOUT as u32;
        const BASE_DIR = lvgl_sys::lv_style_prop_t_LV_STYLE_BASE_DIR as u32;

        const PROP_ANY = lvgl_sys::lv_style_prop_t_LV_STYLE_PROP_ANY as u32;
    }
}

macro_rules! gen_lv_style {
    ($func_name:ident,$vty:ty) => {
        paste! {
            #[inline]
            pub fn $func_name(&mut self, value: $vty) {
                unsafe {
                    lvgl_sys::[<lv_style_ $func_name>](
                        self.raw.as_mut(),
                        value.into(),
                    );
                }
            }
        }
    };
}

// TODO: Move this into lvgl-codegen
impl Style {
    gen_lv_style!(set_align, u8);
    //gen_lv_style!(set_anim, );
    //gen_lv_style!(set_anim_speed, );
    //gen_lv_style!(set_anim_time, );
    gen_lv_style!(set_arc_color, Color);
    //gen_lv_style!(set_arc_img_src, );
    gen_lv_style!(set_arc_opa, Opacity);
    gen_lv_style!(set_arc_rounded, bool);
    gen_lv_style!(set_arc_width, i16);
    //gen_lv_style!(set_base_dir, );
    gen_lv_style!(set_bg_color, Color);
    gen_lv_style!(set_bg_dither_mode, u8);
    //gen_lv_style!(set_bg_grad, );
    gen_lv_style!(set_bg_grad_color, Color);
    //gen_lv_style!(set_bg_grad_dir, );
    gen_lv_style!(set_bg_grad_stop, i16);
    gen_lv_style!(set_bg_img_opa, Opacity);
    gen_lv_style!(set_bg_img_recolor, Color);
    gen_lv_style!(set_bg_img_recolor_opa, Opacity);
    //gen_lv_style!(set_bg_img_src, );
    gen_lv_style!(set_bg_img_tiled, bool);
    gen_lv_style!(set_bg_main_stop, i16);
    gen_lv_style!(set_bg_opa, Opacity);
    gen_lv_style!(set_blend_mode, u8);
    gen_lv_style!(set_border_color, Color);
    gen_lv_style!(set_border_opa, Opacity);
    gen_lv_style!(set_border_post, bool);
    gen_lv_style!(set_border_side, u8);
    gen_lv_style!(set_border_width, i16);
    gen_lv_style!(set_clip_corner, bool);
    //gen_lv_style!(set_color_filter_dsc, );
    gen_lv_style!(set_color_filter_opa, Opacity);
    gen_lv_style!(set_flex_cross_place, c_uint);
    gen_lv_style!(set_flex_flow, c_uint);
    gen_lv_style!(set_flex_grow, u8);
    gen_lv_style!(set_flex_main_place, c_uint);
    gen_lv_style!(set_flex_track_place, c_uint);
    gen_lv_style!(set_grid_cell_column_pos, i16);
    gen_lv_style!(set_grid_cell_column_span, i16);
    gen_lv_style!(set_grid_cell_row_pos, i16);
    gen_lv_style!(set_grid_cell_row_span, i16);
    gen_lv_style!(set_grid_cell_x_align, i16);
    gen_lv_style!(set_grid_cell_y_align, i16);
    gen_lv_style!(set_grid_column_align, c_uint);
    //gen_lv_style!(set_grid_column_dsc_array, );
    gen_lv_style!(set_grid_row_align, c_uint);
    //gen_lv_style!(set_grid_row_dsc_array, );
    gen_lv_style!(set_height, i16);
    gen_lv_style!(set_img_opa, Opacity);
    gen_lv_style!(set_img_recolor, Color);
    gen_lv_style!(set_img_recolor_opa, Opacity);
    gen_lv_style!(set_layout, u16);
    gen_lv_style!(set_line_color, Color);
    gen_lv_style!(set_line_dash_gap, i16);
    gen_lv_style!(set_line_dash_width, i16);
    gen_lv_style!(set_line_opa, Opacity);
    gen_lv_style!(set_line_rounded, bool);
    gen_lv_style!(set_line_width, i16);
    gen_lv_style!(set_max_height, i16);
    gen_lv_style!(set_max_width, i16);
    gen_lv_style!(set_min_height, i16);
    gen_lv_style!(set_min_width, i16);
    gen_lv_style!(set_opa, Opacity);
    gen_lv_style!(set_outline_color, Color);
    gen_lv_style!(set_outline_opa, Opacity);
    gen_lv_style!(set_outline_pad, i16);
    gen_lv_style!(set_outline_width, i16);
    gen_lv_style!(set_pad_bottom, i16);
    gen_lv_style!(set_pad_column, i16);
    gen_lv_style!(set_pad_left, i16);
    gen_lv_style!(set_pad_right, i16);
    gen_lv_style!(set_pad_row, i16);
    gen_lv_style!(set_pad_top, i16);
    //gen_lv_style!(set_prop, );
    //gen_lv_style!(set_prop_meta, );
    gen_lv_style!(set_radius, i16);
    gen_lv_style!(set_shadow_color, Color);
    gen_lv_style!(set_shadow_ofs_x, i16);
    gen_lv_style!(set_shadow_ofs_y, i16);
    gen_lv_style!(set_shadow_opa, Opacity);
    gen_lv_style!(set_shadow_spread, i16);
    gen_lv_style!(set_shadow_width, i16);
    gen_lv_style!(set_text_align, TextAlign);
    gen_lv_style!(set_text_color, Color);
    gen_lv_style!(set_text_decor, u8);
    //gen_lv_style!(set_text_font, );
    gen_lv_style!(set_text_letter_space, i16);
    gen_lv_style!(set_text_line_space, i16);
    gen_lv_style!(set_text_opa, Opacity);
    gen_lv_style!(set_transform_angle, i16);
    gen_lv_style!(set_transform_height, i16);
    gen_lv_style!(set_transform_pivot_x, i16);
    gen_lv_style!(set_transform_pivot_y, i16);
    gen_lv_style!(set_transform_width, i16);
    gen_lv_style!(set_transform_zoom, i16);
    //gen_lv_style!(set_transition, );
    gen_lv_style!(set_translate_x, i16);
    gen_lv_style!(set_translate_y, i16);
    gen_lv_style!(set_width, i16);
    gen_lv_style!(set_x, i16);
    gen_lv_style!(set_y, i16);
}
