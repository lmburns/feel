#![forbid(unsafe_code, future_incompatible)]
#![deny(
    missing_debug_implementations,
    nonstandard_style,
    missing_copy_implementations,
    unused_qualifications
)]

use clap::{crate_authors, crate_version, AppSettings, Clap};
use colored::*;
use filetime::FileTime;

use std::{
    fs::{DirBuilder, OpenOptions},
    path::PathBuf,
    time::SystemTime,
};

#[derive(Clap, Default, Debug)]
#[clap(
    version = crate_version!(),
    author = crate_authors!(),
    about = "it goes beyond touch",
    global_setting = AppSettings::ColoredHelp,
    global_setting = AppSettings::ColorAlways,
    name = "feel",
)]
struct FeelOpts {
    #[clap(parse(from_os_str), multiple_values = true)]
    path:  Vec<PathBuf>,
    #[clap(long, short)]
    quiet: bool,
}

fn main() -> Result<(), String> {
    let opts = FeelOpts::parse();

    for (idx, path) in opts.path.iter().enumerate() {
        let dir = path.parent().expect("unable to find the path base");

        DirBuilder::new()
            .recursive(true)
            .create(dir)
            .map_err(|_| format!("could not create {}", dir.to_string_lossy()))?;

        OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&path)
            .map_err(|_| format!("could not open {}", path.to_string_lossy()))?;

        let file_time = FileTime::from_system_time(SystemTime::now());

        filetime::set_file_times(path, file_time, file_time)
            .map_err(|_| String::from("could not update file times"))?;

        if !opts.quiet {
            if idx == 0 as usize {
                println!("{}", "Created".bold().green());
            }
            println!("  └  {}", path.display().to_string().bold().magenta());
        }
    }

    Ok(())
}
