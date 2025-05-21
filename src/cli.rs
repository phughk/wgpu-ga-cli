use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[command(name = "ga-cli", version = "1.0", about = "Input genetic algorithm description")]
pub struct CliArgs {
    #[arg(value_name = "INPUT")]
    pub input: PathBuf,
}