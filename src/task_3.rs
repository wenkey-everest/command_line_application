use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

pub fn task_3() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("{}!", args.name)
    }
}
