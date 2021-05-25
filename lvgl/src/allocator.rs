use core::alloc::{GlobalAlloc, Layout};

// Register the global allocator
#[global_allocator]
static ALLOCATOR: LvglAlloc = LvglAlloc;

pub struct LvglAlloc;

unsafe impl GlobalAlloc for LvglAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Make sure LVGL is initialized!
        let _ = *crate::LVGL_INITIALIZED;
        lvgl_sys::lv_mem_alloc(layout.size() as lvgl_sys::size_t) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let _ = *crate::LVGL_INITIALIZED;
        lvgl_sys::lv_mem_free(ptr as *const cty::c_void)
    }
}
