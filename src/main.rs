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

fn logic() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    println!("The zip file is : {:?}", args.path);
    let file = fs::File::open(&args.path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    println!(
        "Successfully opened! The zip contains {} files.",
        archive.len()
    );
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File:{} comment:{}", i, comment);
            }
        }
        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath)?;
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                fs::create_dir_all(p)?;
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}
