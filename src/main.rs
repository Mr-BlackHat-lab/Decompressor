use clap::Parser;
use std::fs;
use std::io;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf, // path of zip file
}

fn main() {
    std::process::exit(logic());
}

fn logic() -> i32 {
    let args = Cli::parse();
    println!("The zip file is : {:?}", args.path);
    let file = fs::File::open(&args.path).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    0
}
