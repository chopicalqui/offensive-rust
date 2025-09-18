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
            continuous_screen_capture(&output_path, 1000, true);
        }
    };
}
