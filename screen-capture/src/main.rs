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
}

fn main() {
    let args = Args::parse();
    unsafe {
        if let Some(name) = args.file {
            capture_screen(&name);
        } else if let Some(name) = args.dir {
            continuous_screen_capture(Path::new(&name));
        }
    }
}
