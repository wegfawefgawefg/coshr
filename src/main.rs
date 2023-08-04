use anyhow::{Context, Result};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    filepaths: Vec<std::path::PathBuf>,
}

fn process_files(file_paths: Vec<std::path::PathBuf>) -> Result<String> {
    let mut composite_text = String::new();

    for file_path in file_paths {
        if !file_path.exists() {
            println!("File {:?} does not exist. Skipping.", file_path);
            continue;
        }

        composite_text += &format!("\n\n# File: {:?}\n\n", file_path);

        let file = File::open(&file_path)
            .with_context(|| format!("could not read file {:?}", file_path))?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let mut line = line?;
            if line.len() > 80 {
                line = format!("{}...{}", &line[..40], &line[(line.len() - 37)..]);
            }
            composite_text += &(line + "\n");
        }
    }

    Ok(composite_text)
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let composite_text = process_files(args.filepaths)?;
    println!("{}", composite_text);
    Ok(())
}
