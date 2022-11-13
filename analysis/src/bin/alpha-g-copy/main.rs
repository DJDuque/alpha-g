//! Make local copies of the MIDAS files from specific runs of the ALPHA-g
//! experiment.

use crate::extension::{decompress_lz4, Extension};
use crate::host::Host;
use clap::Parser;
use glob::{glob, Pattern};
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::{fs, path::PathBuf, process::Command};

/// Hosts for ALPHA-g MIDAS files.
mod host;

/// Extensions for ALPHA-g MIDAS files.
mod extension;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "Make local copies of MIDAS files from remote hosts", long_about = None)]
struct Args {
    /// Run numbers for which you want to copy all MIDAS files locally
    #[arg(required = true)]
    run_numbers: Vec<u32>,
    /// User at remote host
    #[arg(short, long)]
    user: String,
    /// Host from which the files will be copied
    #[arg(value_enum, short, long)]
    source: Host,
    /// Path where the MIDAS files will be copied into
    #[arg(short, long, default_value = "./", value_parser(is_directory))]
    output_path: PathBuf,
    /// Extension i.e. compression of remote files
    #[arg(value_enum, short, long)]
    extension: Option<Extension>,
    /// Decompress the copied MIDAS file (requires --extension)
    #[arg(short, long, requires("extension"))]
    decompress: bool,
}

/// Copy and (if applicable) decompress the MIDAS files
fn main() {
    let args = Args::parse();

    let filenames: Vec<Pattern> = args
        .run_numbers
        .into_iter()
        .map(|n| args.source.filename(n, args.extension))
        .collect();

    let remote_filenames = filenames.iter().map(|f| {
        let remote_path = args.source.path_to_data().join(f.to_string());
        args.user.clone() + "@" + &args.source.to_string() + ":" + remote_path.to_str().unwrap()
    });

    let status = Command::new("rsync")
        .args(["--partial", "--progress", "--human-readable", "--compress"])
        .args(remote_filenames)
        .arg(&args.output_path)
        .status()
        .expect("failed to execute rsync");

    if status.success() && args.decompress {
        let spinner = ProgressBar::new_spinner()
            .with_style(ProgressStyle::default_spinner().tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "));
        spinner.enable_steady_tick(std::time::Duration::from_millis(100));

        let local_filenames = filenames
            .iter()
            .map(|f| args.output_path.join(f.to_string()));
        for pattern in local_filenames {
            for entry in glob(pattern.to_str().unwrap()).unwrap() {
                let path = entry.unwrap();
                spinner.set_message(format!("Decompressing {}...", path.display()));
                match args.extension.unwrap() {
                    Extension::Lz4 => decompress_lz4(&path, &path.with_extension("")).unwrap(),
                }
                fs::remove_file(path).unwrap();
            }
        }
        spinner.finish_and_clear();
    }
}

/// Parse `--output-path` flag as valid directory
fn is_directory(s: &str) -> Result<PathBuf, String> {
    let path: PathBuf = s.into();
    if path.is_dir() {
        Ok(path)
    } else {
        Err(String::from("path is not pointing at a directory on disk"))
    }
}

#[cfg(test)]
mod tests;