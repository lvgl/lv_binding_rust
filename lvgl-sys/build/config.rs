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
    color_depth: ColorDepth,
    color_16_swap: bool,
    color_screen_transp: bool,
    color_mix_round_ofs: ColorMixRoundOFS,
    color_chroma_hex: u64,
    mem_custom: bool,
    mem_size: u128,
    mem_adr: usize, // TODO: is this fine when cross-compiling to targets w/ different usize?
    mem_custom_include: String,
    mem_custom_alloc: String,
    mem_custom_free: String,
    mem_custom_realloc: String,
}

pub struct DrvConfig {

}

pub fn conf_gen() {}