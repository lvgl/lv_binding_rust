#[repr(i32)]
enum ColorDepth {
    D1 = 1,
    D8 = 8,
    D16 = 16,
    D32 = 32,
}

#[repr(i32)]
enum ColorMixRoundOFS {
    R0 = 0,
    R64 = 64,
    R128 = 128,
    R192 = 192,
    R254 = 254,
}

pub struct LvConfig {
    // Color
    color_depth: ColorDepth,
    color_16_swap: bool,
    color_screen_transp: bool,
    color_mix_round_ofs: ColorMixRoundOFS,
    color_chroma_hex: u64,
    // Memory
    mem_custom: bool,
    mem_size: u128,
    mem_adr: usize, // TODO: is this fine when cross-compiling to targets w/ different usize?
    mem_custom_include: String,
    mem_custom_alloc: String,
    mem_custom_free: String,
    mem_custom_realloc: String,
    mem_buf_max_num: u32,
    memcpy_memset_std: bool,
    // HAL
    disp_def_refr_period: u32,
    indev_def_read_period: u32,
    tick_custom: bool,
    tick_custom_include: String,
    tick_custom_sys_time_expr: String,
    dpi_def: u32,
    // Drawing
    draw_complex: bool,
    shadow_cache_size: u128,
    circle_cache_size: u128,
    layer_simple_buf_size: u128,
    layer_simple_fallback_buf_size: u128,
    img_cache_def_size: u128,
    gradient_max_stops: u32,
    grad_cache_def_size: u128,
    dither_grdient: bool,
    dither_gradient_error_diffusion: bool,
    disp_rot_max_buf: u128,
    // GPU
    
}

pub struct DrvConfig {

}

pub fn conf_gen() {}