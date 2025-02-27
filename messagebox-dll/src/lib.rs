use std::ptr::null_mut;
use windows::core::PCSTR;
use windows::Win32::Foundation::{HWND, HINSTANCE};
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK};
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

#[no_mangle]
pub extern "system" fn DllMain(_: HINSTANCE, fdw_reason: u32, _lpv_reserved: *const ()) -> u32 {
    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                MessageBoxA(
                    HWND(null_mut()),
                    PCSTR(b"Test DLL\0".as_ptr()),
                    PCSTR(b"DLL successfully executed.\0".as_ptr()),
                    MB_OK,
                );
            }
        },
        DLL_PROCESS_DETACH => {
            // Cleanup if necessary
        },
        _ => {}
    }
    0
}
