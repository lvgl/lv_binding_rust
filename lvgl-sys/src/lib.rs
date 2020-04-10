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

            let hres = lv_disp_get_hor_res(std::ptr::null_mut());
            assert_eq!(hres, 480);

            let vres = lv_disp_get_ver_res(std::ptr::null_mut());
            assert_eq!(vres, 320);
        }
    }
}
