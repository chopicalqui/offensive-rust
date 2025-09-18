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

use std::path::Path;
use clap::{Parser, ArgGroup};

mod screenshot;
use screenshot::{capture_screen, continuous_screen_capture};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(ArgGroup::new("output").required(true).args(["file", "dir"])))]
struct Args {
    /// Screenshot output file
    #[arg(short, long, group = "output")]
    file: Option<String>,

    /// Screenshot output directory
    #[arg(short, long, group = "output")]
    dir: Option<String>,

    /// Screenshot frequency in milliseconds
    #[arg(long, default_value_t = 1000)]
    frequency: u64,

    /// Use timestamp for filenames (default: true). If false, incrementing filenames will be used.
    #[arg(long, default_value_t = false)]
    timestamp: bool,
}

fn main() {
    let args = Args::parse();
    unsafe {
        if let Some(name) = args.file {
            capture_screen(&name);
        } else if let Some(name) = args.dir {
            continuous_screen_capture(Path::new(&name), args.frequency, args.timestamp);
        }
    }
}
