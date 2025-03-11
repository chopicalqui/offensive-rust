/*!
 * Screenshot Utility
 *
 * This Rust crate captures and saves screenshots of the current screen.
 *
 * License: GNU General Public License v3.0 (GPLv3)
 * Author: Lukas Reiter
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use chrono::offset::Local;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::WindowsAndMessaging::*;


/// Retrieves the current screen resolution using the Windows API.
///
/// # Returns
/// A tuple containing `(width, height)` of the screen.
pub unsafe fn get_screen_resolution() -> (i32, i32) {
    let mut dev_mode = DEVMODEW {
        dmSize: std::mem::size_of::<DEVMODEW>() as u16,
        ..Default::default()
    };

    if EnumDisplaySettingsW(None, ENUM_CURRENT_SETTINGS, &mut dev_mode).as_bool() {
        return (dev_mode.dmPelsWidth as i32, dev_mode.dmPelsHeight as i32);
    }

    // Fallback values in case of failure
    println!("Warning: EnumDisplaySettingsW failed, using default values.");
    (2048, 1080) // Manually set resolution if needed
}


/// Saves a screenshot bitmap to a file in BMP format.
///
/// # Arguments
/// * `file_name` - The path where the BMP file will be saved.
/// * `h_bitmap` - The handle to the bitmap containing screenshot data.
/// * `h_dc` - The handle to the device context from which bitmap data is extracted.
/// * `width` - The width of the screenshot in pixels.
/// * `height` - The height of the screenshot in pixels.
pub unsafe fn save_bitmap(file_name: &str, h_bitmap: HBITMAP, h_dc: HDC, width: i32, height: i32) {
    let mut file = File::create(file_name).expect("Failed to create file");

    let mut bitmap_info = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width,
            biHeight: -height,
            biPlanes: 1,
            biBitCount: 24,
            biCompression: BI_RGB.0,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: [RGBQUAD { rgbBlue: 0, rgbGreen: 0, rgbRed: 0, rgbReserved: 0 }],
    };

    let mut bitmap_data = vec![0u8; (width * height * 3) as usize];
    GetDIBits(h_dc, h_bitmap, 0, height as u32, Some(bitmap_data.as_mut_ptr().cast()), &mut bitmap_info, DIB_RGB_COLORS);

    // Write BMP header
    let file_size = 14 + std::mem::size_of::<BITMAPINFOHEADER>() + bitmap_data.len();
    file.write_all(&[0x42, 0x4D]).unwrap(); // 'BM'
    file.write_all(&(file_size as u32).to_le_bytes()).unwrap();
    file.write_all(&[0, 0, 0, 0]).unwrap(); // Reserved
    file.write_all(&(14 + std::mem::size_of::<BITMAPINFOHEADER>() as u32).to_le_bytes()).unwrap(); // Offset
    
    // Write DIB header
    file.write_all(&bitmap_info.bmiHeader.biSize.to_le_bytes()).unwrap();
    file.write_all(&bitmap_info.bmiHeader.biWidth.to_le_bytes()).unwrap();
    file.write_all(&bitmap_info.bmiHeader.biHeight.to_le_bytes()).unwrap();
    file.write_all(&bitmap_info.bmiHeader.biPlanes.to_le_bytes()).unwrap();
    file.write_all(&bitmap_info.bmiHeader.biBitCount.to_le_bytes()).unwrap();
    file.write_all(&bitmap_info.bmiHeader.biCompression.to_le_bytes()).unwrap();
    file.write_all(&bitmap_info.bmiHeader.biSizeImage.to_le_bytes()).unwrap();
    file.write_all(&bitmap_info.bmiHeader.biXPelsPerMeter.to_le_bytes()).unwrap();
    file.write_all(&bitmap_info.bmiHeader.biYPelsPerMeter.to_le_bytes()).unwrap();
    file.write_all(&bitmap_info.bmiHeader.biClrUsed.to_le_bytes()).unwrap();
    file.write_all(&bitmap_info.bmiHeader.biClrImportant.to_le_bytes()).unwrap();
    
    // Write pixel data
    file.write_all(&bitmap_data).unwrap();
}


/// Captures the current screen and saves it to a file.
///
/// # Arguments
/// * `file_name` - The path where the screenshot will be saved.
pub unsafe fn capture_screen(file_name: &str) {
    // Get the desktop device context
    let h_dc = GetDC(None);
    let h_dc_mem = CreateCompatibleDC(h_dc);

    // Get screen dimensions including multi-monitor setups
    let left = GetSystemMetrics(SM_XVIRTUALSCREEN);
    let top = GetSystemMetrics(SM_YVIRTUALSCREEN);
    // let width = GetSystemMetrics(SM_CXVIRTUALSCREEN);
    // let height = GetSystemMetrics(SM_CYVIRTUALSCREEN);
    let (width, height) = get_screen_resolution();

    // Create a compatible bitmap
    let h_bitmap = CreateCompatibleBitmap(h_dc, width, height);
    SelectObject(h_dc_mem, h_bitmap);

    // Copy screen content to memory device context
    let _ = BitBlt(h_dc_mem, 0, 0, width, height, h_dc, left, top, SRCCOPY);

    // Save bitmap to file
    save_bitmap(&file_name, h_bitmap, h_dc, width, height);

    // Cleanup
    let _ = DeleteObject(h_bitmap);
    let _ = DeleteDC(h_dc_mem);
    let _ = ReleaseDC(None, h_dc);
}


/// Continuously captures screenshots at a 1-second interval.
///
/// # Arguments
/// * `output_dir` - The directory where screenshots will be saved.
pub unsafe fn continuous_screen_capture(output_dir: &Path) {
    loop {
        // Generate timestamped filename
        let timestamp = Local::now().format("%Y%m%d-%H%M%S").to_string();
        let filename = output_dir.join(format!("{}.bmp", timestamp)).display().to_string();

        // Capture the screenshot
        unsafe {
            capture_screen(&filename);
        }

        println!("Screenshot saved to {}", filename);

        // Wait 1 second before taking the next screenshot
        sleep(Duration::from_secs(1));
    }
}