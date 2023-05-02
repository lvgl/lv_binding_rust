//! Styling for LVGL objects and widgets
//!
//! Objects in LVGL can have associated styling information. After a `Style` is
//! created and configured, it can be added to any object or widget:
//! ```
//! use lvgl::{Color, Widget};
//! use lvgl::style::Style;
//!
//! let mut my_style = Style::default();
//! my_style.set_text_color(Color::from_rgb((0, 0, 0)));
//!
//! //my_widget.add_style(Part::Main, &mut my_style).unwrap();
//! // ...
//! ```
//! All methods on the `Style` type directly lower to their C LVGL
//! counterparts.

use crate::{font::Font, Align, Box, Color, TextAlign};
use core::mem;
use cty::c_uint;
use paste::paste;

pub enum Themes {
    Pretty,
}

/// An LVGL `lv_style_t`. Allows for styling objects. Once created, a `Style`
/// should be configured and then added to an object.
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
    /// Represents possible opacities for use on `Style` objects.
    pub struct Opacity: u32 {
        const OPA_TRANSP = lvgl_sys::LV_OPA_TRANSP;
        const OPA_0 = lvgl_sys::LV_OPA_0;
        const OPA_10 = lvgl_sys::LV_OPA_10;
        const OPA_20 = lvgl_sys::LV_OPA_20;
        const OPA_30 = lvgl_sys::LV_OPA_30;
        const OPA_40 = lvgl_sys::LV_OPA_40;
        const OPA_50 = lvgl_sys::LV_OPA_50;
        const OPA_60 = lvgl_sys::LV_OPA_60;
        const OPA_70 = lvgl_sys::LV_OPA_70;
        const OPA_80 = lvgl_sys::LV_OPA_80;
        const OPA_90 = lvgl_sys::LV_OPA_90;
        const OPA_100 = lvgl_sys::LV_OPA_100;
        const OPA_COVER = lvgl_sys::LV_OPA_COVER;
    }
}

impl From<Opacity> for u8 {
    fn from(value: Opacity) -> u8 {
        value.bits() as u8
    }
}

/// Represents a `Layout`, to be used with the `set_layout()` method on `Style`
/// objects.
pub struct Layout {
    inner: u16
}

impl Layout {
    /// Generates an `LV_LAYOUT_FLEX`
    pub fn flex() -> Self {
        Self {
            inner: unsafe {
                lvgl_sys::LV_LAYOUT_FLEX
            }
        }
    }

    /// Generates an `LV_LAYOUT_GRID`
    pub fn grid() -> Self {
        Self {
            inner: unsafe {
                lvgl_sys::LV_LAYOUT_GRID
            }
        }
    }
}

impl From<Layout> for u16 {
    fn from(value: Layout) -> Self {
        value.inner
    }
}

/// A coordinate array, for use with `set_grid_*_dsc_array()` methods on
/// `Style` objects.
#[derive(Clone)]
pub struct CoordDesc {
    inner: Box<[i16; 3]>
}

impl CoordDesc {
    /// Generates a `CoordDesc` from 3 values.
    pub fn from_values(x: i16, y: i16, z: i16) -> Self {
        Self {
            inner: Box::new([x, y, z])
        }
    }

    /// Returns the values contained.
    pub fn values(&self) -> [i16; 3] {
        *self.clone().inner
    }
}

impl From<CoordDesc> for *const i16 {
    fn from(value: CoordDesc) -> Self {
        value.inner.as_ptr()
    }
}

bitflags! {
    /// Various constants relevant for `Style` parameters 
    pub struct StyleProp: u32 {
        const PROP_INV = lvgl_sys::lv_style_prop_t_LV_STYLE_PROP_INV;

        /*Group 0*/
        const WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_WIDTH;
        const MIN_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_MIN_WIDTH;
        const MAX_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_MAX_WIDTH;
        const HEIGHT = lvgl_sys::lv_style_prop_t_LV_STYLE_HEIGHT;
        const MIN_HEIGHT = lvgl_sys::lv_style_prop_t_LV_STYLE_MIN_HEIGHT;
        const MAX_HEIGHT = lvgl_sys::lv_style_prop_t_LV_STYLE_MAX_HEIGHT;
        const X = lvgl_sys::lv_style_prop_t_LV_STYLE_X;
        const Y = lvgl_sys::lv_style_prop_t_LV_STYLE_Y;
        const ALIGN = lvgl_sys::lv_style_prop_t_LV_STYLE_ALIGN;
        const TRANSFORM_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_TRANSFORM_WIDTH;
        const TRANSFORM_HEIGHT = lvgl_sys::lv_style_prop_t_LV_STYLE_TRANSFORM_HEIGHT;
        const TRANSLATE_X = lvgl_sys::lv_style_prop_t_LV_STYLE_TRANSLATE_X;
        const TRANSLATE_Y = lvgl_sys::lv_style_prop_t_LV_STYLE_TRANSLATE_Y;
        const TRANSFORM_ZOOM = lvgl_sys::lv_style_prop_t_LV_STYLE_TRANSFORM_ZOOM;
        const TRANSFORM_ANGLE = lvgl_sys::lv_style_prop_t_LV_STYLE_TRANSFORM_ANGLE;

        /*Group 1*/
        const PAD_TOP = lvgl_sys::lv_style_prop_t_LV_STYLE_PAD_TOP;
        const PAD_BOTTOM = lvgl_sys::lv_style_prop_t_LV_STYLE_PAD_BOTTOM;
        const PAD_LEFT = lvgl_sys::lv_style_prop_t_LV_STYLE_PAD_LEFT;
        const PAD_RIGHT = lvgl_sys::lv_style_prop_t_LV_STYLE_PAD_RIGHT;
        const PAD_ROW = lvgl_sys::lv_style_prop_t_LV_STYLE_PAD_ROW;
        const PAD_COLUMN = lvgl_sys::lv_style_prop_t_LV_STYLE_PAD_COLUMN;

        /*Group 2*/
        const BG_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_COLOR;
        //const BG_COLOR_FILTERED = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_COLOR_FILTERED as u32;
        const BG_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_OPA;
        const BG_GRAD_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_GRAD_COLOR;
        //const BG_GRAD_COLOR_FILTERED = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_GRAD_COLOR_FILTERED as u32;
        const BG_GRAD_DIR = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_GRAD_DIR;
        const BG_MAIN_STOP = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_MAIN_STOP;
        const BG_GRAD_STOP = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_GRAD_STOP;

        const BG_IMG_SRC = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_IMG_SRC;
        const BG_IMG_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_IMG_OPA;
        const BG_IMG_RECOLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_IMG_RECOLOR;
        //const BG_IMG_RECOLOR_FILTERED = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_IMG_RECOLOR_FILTERED as u32;
        const BG_IMG_RECOLOR_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_IMG_RECOLOR_OPA;
        const BG_IMG_TILED = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_IMG_TILED;

        /*Group 3*/
        const BORDER_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_BORDER_COLOR;
        //const BORDER_COLOR_FILTERED = lvgl_sys::lv_style_prop_t_LV_STYLE_BORDER_COLOR_FILTERED as u32;
        const BORDER_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_BORDER_OPA;
        const BORDER_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_BORDER_WIDTH;
        const BORDER_SIDE = lvgl_sys::lv_style_prop_t_LV_STYLE_BORDER_SIDE;
        const BORDER_POST = lvgl_sys::lv_style_prop_t_LV_STYLE_BORDER_POST;

        const OUTLINE_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_OUTLINE_WIDTH;
        const OUTLINE_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_OUTLINE_COLOR;
        //const OUTLINE_COLOR_FILTERED = lvgl_sys::lv_style_prop_t_LV_STYLE_OUTLINE_COLOR_FILTERED as u32;
        const OUTLINE_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_OUTLINE_OPA;
        const OUTLINE_PAD = lvgl_sys::lv_style_prop_t_LV_STYLE_OUTLINE_PAD;

        /*Group 4*/
        const SHADOW_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_WIDTH;
        const SHADOW_OFS_X = lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_OFS_X;
        const SHADOW_OFS_Y = lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_OFS_Y;
        const SHADOW_SPREAD = lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_SPREAD;
        const SHADOW_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_COLOR;
        //const SHADOW_COLOR_FILTERED = lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_COLOR_FILTERED as u32;
        const SHADOW_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_OPA;

        const IMG_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_IMG_OPA;
        const IMG_RECOLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_IMG_RECOLOR;
        //const IMG_RECOLOR_FILTERED = lvgl_sys::lv_style_prop_t_LV_STYLE_IMG_RECOLOR_FILTERED as u32;
        const IMG_RECOLOR_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_IMG_RECOLOR_OPA;

        const LINE_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_LINE_WIDTH;
        const LINE_DASH_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_LINE_DASH_WIDTH;
        const LINE_DASH_GAP = lvgl_sys::lv_style_prop_t_LV_STYLE_LINE_DASH_GAP;
        const LINE_ROUNDED = lvgl_sys::lv_style_prop_t_LV_STYLE_LINE_ROUNDED;
        const LINE_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_LINE_COLOR;
        //const LINE_COLOR_FILTERED = lvgl_sys::lv_style_prop_t_LV_STYLE_LINE_COLOR_FILTERED as u32;
        const LINE_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_LINE_OPA;

        /*Group 5*/
        const ARC_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_ARC_WIDTH;
        const ARC_ROUNDED = lvgl_sys::lv_style_prop_t_LV_STYLE_ARC_ROUNDED;
        const ARC_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_ARC_COLOR;
        //const ARC_COLOR_FILTERED = lvgl_sys::lv_style_prop_t_LV_STYLE_ARC_COLOR_FILTERED as u32;
        const ARC_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_ARC_OPA;
        const ARC_IMG_SRC = lvgl_sys::lv_style_prop_t_LV_STYLE_ARC_IMG_SRC;

        const TEXT_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_COLOR;
        //const TEXT_COLOR_FILTERED = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_COLOR_FILTERED as u32;
        const TEXT_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_OPA;
        const TEXT_FONT = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_FONT;
        const TEXT_LETTER_SPACE = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_LETTER_SPACE;
        const TEXT_LINE_SPACE = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_LINE_SPACE;
        const TEXT_DECOR = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_DECOR;
        const TEXT_ALIGN = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_ALIGN;

        /*Group 6*/
        const RADIUS = lvgl_sys::lv_style_prop_t_LV_STYLE_RADIUS;
        const CLIP_CORNER = lvgl_sys::lv_style_prop_t_LV_STYLE_CLIP_CORNER;
        const OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_OPA;
        const COLOR_FILTER_DSC = lvgl_sys::lv_style_prop_t_LV_STYLE_COLOR_FILTER_DSC;
        const COLOR_FILTER_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_COLOR_FILTER_OPA;
        const ANIM_TIME = lvgl_sys::lv_style_prop_t_LV_STYLE_ANIM_TIME;
        const ANIM_SPEED = lvgl_sys::lv_style_prop_t_LV_STYLE_ANIM_SPEED;
        const TRANSITION = lvgl_sys::lv_style_prop_t_LV_STYLE_TRANSITION;
        const BLEND_MODE = lvgl_sys::lv_style_prop_t_LV_STYLE_BLEND_MODE;
        const LAYOUT = lvgl_sys::lv_style_prop_t_LV_STYLE_LAYOUT;
        const BASE_DIR = lvgl_sys::lv_style_prop_t_LV_STYLE_BASE_DIR;

        const PROP_ANY = lvgl_sys::lv_style_prop_t_LV_STYLE_PROP_ANY;
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

impl Style {
    gen_lv_style!(set_align, Align);
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
    gen_lv_style!(set_grid_column_dsc_array, CoordDesc);
    gen_lv_style!(set_grid_row_align, c_uint);
    gen_lv_style!(set_grid_row_dsc_array, CoordDesc);
    gen_lv_style!(set_height, i16);
    gen_lv_style!(set_img_opa, Opacity);
    gen_lv_style!(set_img_recolor, Color);
    gen_lv_style!(set_img_recolor_opa, Opacity);
    gen_lv_style!(set_layout, Layout);
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
    gen_lv_style!(set_text_font, Font);
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
