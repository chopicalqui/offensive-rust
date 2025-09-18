use std::fs;
use std::path::Path;
use clap::{Arg, Command};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use rand::Rng;

#[cfg(windows)]
use winapi::um::fileapi::{SetFileTime, CreateFileW, OPEN_EXISTING};
#[cfg(windows)]
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
#[cfg(windows)]
use winapi::um::winnt::{FILE_ATTRIBUTE_NORMAL, GENERIC_WRITE, FILE_SHARE_READ, FILE_SHARE_WRITE};
#[cfg(windows)]
use winapi::um::errhandlingapi::GetLastError;
#[cfg(windows)]
use winapi::shared::minwindef::FILETIME;
#[cfg(windows)]
use std::ffi::OsStr;
#[cfg(windows)]
use std::iter::once;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;

// Windows constant for FILE_FLAG_BACKUP_SEMANTICS
#[cfg(windows)]
const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x02000000;

fn main() {
    let matches = Command::new("File Timestamp Tool")
        .version("1.0")
        .author("Your Name")
        .about("Sets creation, access, and modification timestamps for files and directories")
        .arg(
            Arg::new("date")
                .long("date")
                .value_name("DATE")
                .help("Date in YYYY-MM-DD format")
                .required(true),
        )
        .arg(
            Arg::new("time")
                .long("time")
                .value_name("TIME")
                .help("Time in HH:MM:SS format")
                .required(true),
        )
        .arg(
            Arg::new("jitter")
                .long("jitter")
                .value_name("SECONDS")
                .help("Maximum jitter in seconds for directory entries (optional)")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            Arg::new("path")
                .value_name("FILE_OR_DIR")
                .help("File or directory path")
                .required(true)
                .index(1),
        )
        .get_matches();

    let date_str = matches.get_one::<String>("date").unwrap();
    let time_str = matches.get_one::<String>("time").unwrap();
    let jitter_seconds = matches.get_one::<u32>("jitter").copied().unwrap_or(0);
    let path_str = matches.get_one::<String>("path").unwrap();

    // Parse date and time
    let base_datetime = match parse_datetime(date_str, time_str) {
        Ok(dt) => dt,
        Err(e) => {
            eprintln!("Error parsing date/time: {}", e);
            std::process::exit(1);
        }
    };

    let path = Path::new(path_str);
    
    if !path.exists() {
        eprintln!("Error: Path '{}' does not exist", path_str);
        std::process::exit(1);
    }

    if path.is_file() {
        match set_file_timestamps(path, base_datetime) {
            Ok(()) => println!("Successfully updated timestamps for file: {}", path_str),
            Err(e) => {
                eprintln!("ERROR: Failed to set timestamps for file '{}': {}", path_str, e);
                std::process::exit(1);
            }
        }
    } else if path.is_dir() {
        let (success_count, error_count) = process_directory(path, base_datetime, jitter_seconds);
        
        if error_count > 0 {
            println!("Completed with {} successful updates and {} errors in directory: {}", 
                success_count, error_count, path_str);
            if success_count == 0 {
                std::process::exit(1);
            }
        } else {
            println!("Successfully updated timestamps for {} files in directory: {}", success_count, path_str);
        }
    }
}

fn parse_datetime(date_str: &str, time_str: &str) -> Result<NaiveDateTime, String> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|_| format!("Invalid date format '{}'. Expected YYYY-MM-DD", date_str))?;
    
    let time = NaiveTime::parse_from_str(time_str, "%H:%M:%S")
        .map_err(|_| format!("Invalid time format '{}'. Expected HH:MM:SS", time_str))?;
    
    Ok(date.and_time(time))
}

fn process_directory(dir: &Path, base_datetime: NaiveDateTime, jitter_seconds: u32) -> (usize, usize) {
    let mut count = 0;
    let mut error_count = 0;
    let mut rng = rand::thread_rng();

    println!("Processing directory: {}", dir.display());

    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("ERROR: Failed to read directory {}: {}", dir.display(), e);
            return (0, 1);
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("ERROR: Failed to read directory entry in {}: {}", dir.display(), e);
                error_count += 1;
                continue;
            }
        };
        
        let path = entry.path();
        
        println!("Found: {} ({})", path.display(), if path.is_dir() { "directory" } else { "file" });
        
        // Calculate jittered datetime
        let jitter = if jitter_seconds > 0 {
            rng.gen_range(0..=jitter_seconds as i64)
        } else {
            0
        };
        
        let jittered_datetime = base_datetime + chrono::Duration::seconds(jitter);
        
        if path.is_file() {
            println!("Setting timestamp for file: {}", path.display());
            match set_file_timestamps(&path, jittered_datetime) {
                Ok(()) => count += 1,
                Err(e) => {
                    eprintln!("ERROR: Failed to set timestamp for file {}: {}", path.display(), e);
                    error_count += 1;
                }
            }
        } else if path.is_dir() {
            // Recursively process subdirectories
            println!("Recursively processing subdirectory: {}", path.display());
            let (subdir_count, subdir_errors) = process_directory(&path, base_datetime, jitter_seconds);
            count += subdir_count;
            error_count += subdir_errors;
            
            // Set timestamps for the directory itself
            println!("Setting timestamp for directory: {}", path.display());
            match set_file_timestamps(&path, jittered_datetime) {
                Ok(()) => count += 1,
                Err(e) => {
                    eprintln!("ERROR: Failed to set timestamp for directory {}: {}", path.display(), e);
                    error_count += 1;
                }
            }
        }
    }
    
    (count, error_count)
}

#[cfg(windows)]
fn set_file_timestamps(path: &Path, datetime: NaiveDateTime) -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = datetime.and_utc().timestamp() as u64;
    let filetime = unix_to_filetime(timestamp);
    
    // Convert path to wide string for Windows API
    let wide_path: Vec<u16> = OsStr::new(path)
        .encode_wide()
        .chain(once(0))
        .collect();
    
    // Debug: Print what we're trying to do
    println!("Attempting to open {} with timestamp {}", 
        if path.is_dir() { "directory" } else { "file" }, 
        datetime.format("%Y-%m-%d %H:%M:%S"));
    
    // Use appropriate flags for files vs directories
    let flags = if path.is_dir() {
        println!("Using FILE_FLAG_BACKUP_SEMANTICS for directory");
        FILE_FLAG_BACKUP_SEMANTICS
    } else {
        println!("Using FILE_ATTRIBUTE_NORMAL for file");
        FILE_ATTRIBUTE_NORMAL
    };
    
    unsafe {
        let handle = CreateFileW(
            wide_path.as_ptr(),
            GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            std::ptr::null_mut(),
            OPEN_EXISTING,
            flags,
            std::ptr::null_mut(),
        );
        
        if handle == INVALID_HANDLE_VALUE {
            let error_code = GetLastError();
            let error_msg = match error_code {
                2 => "The system cannot find the file specified",
                3 => "The system cannot find the path specified", 
                5 => "Access is denied (try running as administrator)",
                32 => "The process cannot access the file because it is being used by another process",
                87 => "The parameter is incorrect",
                183 => "Cannot create a file when that file already exists",
                _ => "Unknown error"
            };
            return Err(format!("Failed to open {}: {} (Error code: {})\nPath: {}", 
                if path.is_dir() { "directory" } else { "file" },
                error_msg, 
                error_code,
                path.display()
            ).into());
        }
        
        println!("Successfully opened handle, setting timestamps...");
        let result = SetFileTime(handle, &filetime, &filetime, &filetime);
        
        if result == 0 {
            let error_code = GetLastError();
            CloseHandle(handle);
            return Err(format!("Failed to set file times for: {} (Error code: {})", path.display(), error_code).into());
        }
        
        CloseHandle(handle);
        println!("Successfully set timestamps for: {}", path.display());
    }
    
    Ok(())
}

#[cfg(not(windows))]
fn set_file_timestamps(path: &Path, datetime: NaiveDateTime) -> Result<(), Box<dyn std::error::Error>> {
    use libc::{utimensat, timespec, AT_FDCWD};
    use std::ffi::CString;
    
    let timestamp = datetime.and_utc().timestamp();
    let path_cstring = CString::new(path.to_str().ok_or("Invalid path")?)?;
    
    let times = [
        timespec {
            tv_sec: timestamp,
            tv_nsec: 0,
        },
        timespec {
            tv_sec: timestamp,
            tv_nsec: 0,
        },
    ];
    
    unsafe {
        if utimensat(AT_FDCWD, path_cstring.as_ptr(), times.as_ptr(), 0) != 0 {
            return Err(format!("Failed to set timestamps for: {}", path.display()).into());
        }
    }
    
    Ok(())
}

#[cfg(windows)]
fn unix_to_filetime(unix_time: u64) -> FILETIME {
    // Convert Unix timestamp to Windows FILETIME
    // Windows FILETIME is in 100-nanosecond intervals since January 1, 1601
    // Unix timestamp is in seconds since January 1, 1970
    
    const UNIX_EPOCH_FILETIME: u64 = 116444736000000000; // January 1, 1970 in FILETIME
    let filetime_64 = (unix_time * 10000000) + UNIX_EPOCH_FILETIME;
    
    FILETIME {
        dwLowDateTime: (filetime_64 & 0xFFFFFFFF) as u32,
        dwHighDateTime: (filetime_64 >> 32) as u32,
    }
}
