use std::ffi::CStr;
use std::path::Path;
use windows::Win32::Foundation::HINSTANCE;

mod screenshot;
use screenshot::continuous_screen_capture;

#[no_mangle]
pub extern "system" fn ScreenCapture(_: isize, _: HINSTANCE, path: *const u8, _: i32) {
    unsafe {
        if ! path.is_null() {
            let output_path = Path::new(CStr::from_ptr(path as *const i8).to_str().unwrap_or("%TEMP%"));
            continuous_screen_capture(&output_path);
        }
    };
}
