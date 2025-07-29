use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    pub fasta: PathBuf,

    #[arg(short, long)]
    pub primers: PathBuf,

    #[arg(short, long)]
    pub outfile: PathBuf,
}
