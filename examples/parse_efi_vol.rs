use uefi_fv::uefi_pi::UefiFirmwareVolumeLoader;
use clap::Parser;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    vol_file: PathBuf,
}


fn main() {

    let fpath = Args::parse().vol_file;
    let fdata = fs::read(fpath).unwrap();

    let vol_loader = UefiFirmwareVolumeLoader::new(fdata).unwrap();

    println!("{vol_loader:#X?}");
}