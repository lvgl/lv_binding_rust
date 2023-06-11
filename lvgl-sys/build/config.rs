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

enum LogLevel {
    Trace,
    Info,
    Warn,
    Error,
    User,
    None,
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
    use_gpu_arm2d: bool,
    use_gpu_stm32_dma2d: bool,
    gpu_dma2d_cmsis_include: String,
    use_gpu_swm341_dma2d: bool,
    gpu_dma2d_swm341_include: String,
    use_gpu_nxp_pxp: bool,
    use_gpu_nxp_pxp_auto_init: bool,
    use_gpu_nxp_vg_lite: bool,
    use_gpu_sdl: bool,
    gpu_sdl_include_path: String,
    gpu_sdl_lru_size: u128,
    gpu_sdl_custom_blend_mode: String,
    // Logging
    use_log: bool,
    log_level: LogLevel,
    log_printf: bool,
    log_trace_mem: bool,
    log_trace_timer: bool,
    log_trace_indev: bool,
    log_trace_disp_refr: bool,
    log_trace_event: bool,
    log_trace_obj_create: bool,
    log_trace_layout: bool,
    log_trace_anim: bool,
    // Asserts
    use_assert_null: bool,
    use_assert_malloc: bool,
    use_assert_style: bool,
    use_assert_mem_integrity: bool,
    use_assert_obj: bool,
    assert_handler_include: String,
    assert_handler: String,
    // Others
    use_perf_monitor: bool,
    use_perf_monitor_pos: u8, // FIXME
    use_mem_monitor: bool,
    use_mem_monitor_pos: u8, // FIXME
    use_refr_debugger: bool,
    sprintf_custom: bool,
    sprintf_custom_include: String,
    snprintf: String,
    vsnprintf: String,
    sprintf_use_float: bool,
    use_user_data: bool,
    enable_gc: bool,
    gc_include: String,
    big_endian_system: bool,
    attribute_tick_inc: Option<String>,
    attribute_timer_handler: Option<String>,
    attribute_flush_ready: Option<String>,
    attribute_mem_align_size: u32,
    attribute_mem_align: Option<String>,
    attribute_large_const: Option<String>,
    attribute_large_ram_array: Option<String>,
    attribute_fast_mem: Option<String>,
    attribute_dma: Option<String>,
    export_const_int: u8, //FIXME
    use_large_coord: bool,
}

pub struct DrvConfig {

}

pub fn conf_gen() {}
