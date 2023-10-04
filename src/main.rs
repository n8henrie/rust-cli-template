// #![warn(clippy::pedantic)]

use tracing_subscriber::{self, EnvFilter};

use {{ crate_name }}::{run, Result};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    run()
}
