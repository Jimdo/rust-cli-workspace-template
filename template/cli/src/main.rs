use clap::Parser;
use lib::*;

type Result<T> = lib::miette::Result<T>;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();

    for _ in 0..args.count {
        say_hello(args.name.clone())?;
    }
    Ok(())
}
