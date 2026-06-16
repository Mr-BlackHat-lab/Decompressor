use clap::Parser;
use std::fs;
use std::io;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf, // path of zip file
}

fn main() {
    if let Err(error) = logic() {
        eprintln!("Oops! Something went wrong: {}", error);
        std::process::exit(1);
    }
}

fn logic() -> Result<(), io::Error> {
    let args = Cli::parse();
    println!("The zip file is : {:?}", args.path);
    let file = fs::File::open(&args.path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    println!(
        "Successfully opened! The zip contains {} files.",
        archive.len()
    );
    Ok(())
}
