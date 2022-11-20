use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

use clap::Parser;

use lithe::parser::parse;

#[derive(Parser, Debug)]
#[command(name = "Lithe CLI")]
#[command(version)]
#[command(about = "CLI for a slim template engine", long_about = None)]
struct Args {
    #[arg(index = 1, num_args = 1, value_name = "FILE")]
    file: Option<PathBuf>,
}

#[derive(Debug)]
struct Config {
    path: PathBuf,
}

fn get_config() -> Result<Config, Box<dyn Error>> {
    let args = Args::parse();
    let path = match args.file {
        Some(buf) => buf,
        None => PathBuf::from("-"),
    };
    Ok(Config { path })
}

fn open_file(path: &PathBuf) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match path.to_str() {
        Some("-") => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(path)?))),
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match open_file(&config.path) {
        Err(err) => {
            let msg =
                format!("failed to open {:?}: {}", config.path.display(), err);
            Err(From::from(msg))
        }
        Ok(mut file) => {
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;

            if !buf.is_empty() {
                let doc = parse(&buf)?;
                println!("{:?}", doc);
            }
            Ok(())
        }
    }
}

fn main() {
    if let Err(e) = get_config().and_then(run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
