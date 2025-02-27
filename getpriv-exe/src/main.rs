use windows::Win32::Security::TOKEN_PRIVILEGES_ATTRIBUTES;
use windows::{
    core::PWSTR,
    Win32::Foundation::{CloseHandle, GetLastError, HANDLE},
    Win32::Security::{
        GetTokenInformation, LookupPrivilegeNameW, SE_PRIVILEGE_ENABLED,
        TOKEN_PRIVILEGES, TOKEN_QUERY,
    },
    Win32::System::Threading::{GetCurrentProcess, OpenProcessToken},
};

fn print_user_privileges() {
    unsafe {
        let mut token_handle: HANDLE = HANDLE::default();

        // Open the current process token
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle).is_err() {
            println!("Failed to open process token. Error: {:?}", GetLastError());
            return;
        }

        // Get required size for buffer
        let mut token_info_size = 0;
        let _ = GetTokenInformation(
            token_handle,
            windows::Win32::Security::TokenPrivileges,
            None,
            0,
            &mut token_info_size,
        );

        let mut buffer = vec![0u8; token_info_size as usize];

        if GetTokenInformation(
            token_handle,
            windows::Win32::Security::TokenPrivileges,
            Some(buffer.as_mut_ptr() as *mut _),
            token_info_size,
            &mut token_info_size,
        )
        .is_err()
        {
            println!("Failed to retrieve token information. Error: {:?}", GetLastError());
            let _ = CloseHandle(token_handle);
            return;
        }

        let token_privileges = buffer.as_ptr() as *const TOKEN_PRIVILEGES;
        let privilege_count = (*token_privileges).PrivilegeCount;
        let privileges_ptr = (*token_privileges).Privileges.as_ptr(); // Pointer to the first element

        println!("User privileges ({:?}):", privilege_count);
        for i in 0..privilege_count {
            let luid_and_attr = *privileges_ptr.add(i as usize);

            // Retrieve privilege name
            let mut name_buffer = vec![0u16; 256];
            let mut name_len = name_buffer.len() as u32;
            if LookupPrivilegeNameW(
                None,
                &luid_and_attr.Luid,
                PWSTR(name_buffer.as_mut_ptr()),
                &mut name_len,
            )
            .is_err()
            {
                println!("Failed to lookup privilege name. Error: {:?}", GetLastError());
                continue;
            }

            let privilege_name = String::from_utf16_lossy(&name_buffer[..name_len as usize]);

            // Determine if privilege is enabled
            let enabled = if (luid_and_attr.Attributes & SE_PRIVILEGE_ENABLED) != TOKEN_PRIVILEGES_ATTRIBUTES(0) {
                "Enabled"
            } else {
                "Disabled"
            };
            println!("{:<width1$}: {}", privilege_name, enabled, width1=40);
        }

        let _ = CloseHandle(token_handle);
    }
}


fn main() {
    print_user_privileges();
    let _ = std::io::stdin().read_line(&mut String::new());
}
