// #![warn(clippy::pedantic)]

use std::io::{self, Write};
use std::result;

use clap::Parser;
use thiserror;
use tracing::{debug, info, trace, warn};

pub type Result<T> = result::Result<T, MyError>;

#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("there was an IO error")]
    Io(#[from] io::Error),
    #[error("unknown error")]
    Unknown,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
    /// Number of times to greet
    #[arg(short, long, default_value_t = 3)]
    count: u8,
}

#[tracing::instrument]
pub fn run() -> Result<()> {
    let args = Config::parse();

    info!("some info!");
    warn!(excitement = "yay!", "hello! I'm gonna shave a yak.");

    writeln!(io::stdout(), "Hello, world!")?;

    println!("Counting to {}", args.count);
    for i in 1..=args.count {
        writeln!(io::stdout(), "{i}...")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
