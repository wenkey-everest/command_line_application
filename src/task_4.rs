use clap::Parser;
use std::path::PathBuf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    search: String,

    #[arg(short, long)]
    path: PathBuf,
}

pub fn task_4() -> (String, PathBuf) {
    let args = Args::parse();

    (args.search, args.path)
}
