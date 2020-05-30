#![no_std]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_sanity_check() {
        unsafe {
            lv_init();

            let horizontal_resolution = lv_disp_get_hor_res(std::ptr::null_mut());
            assert_eq!(horizontal_resolution, 480);

            let vertical_resolution = lv_disp_get_ver_res(std::ptr::null_mut());
            assert_eq!(vertical_resolution, 320);
        }
    }
}
