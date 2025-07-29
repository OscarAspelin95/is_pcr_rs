use std::path::PathBuf;

use clap::Parser;

mod amplicon;
use amplicon::amplicon_search;
use bio::io::fasta::Reader;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod utils;
use utils::PrimerPair;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    fasta: PathBuf,

    #[arg(short, long)]
    primers: PathBuf,
}

fn parse_primer_file<'a>(primer_file: &'a PathBuf) -> Vec<PrimerPair> {
    let primer_buf =
        BufReader::new(File::open(primer_file).expect("Failed to open provided primer file."));

    let lines = primer_buf.lines();

    let mut primer_pairs: Vec<PrimerPair> = Vec::new();

    for line in lines {
        if let Ok(primer_line) = line {
            let line_vec: Vec<&str> = primer_line.split("\t").collect();

            match line_vec.len() {
                5 => {
                    // TODO - this sucks.
                    let primer_name = line_vec[0];
                    let forward_primer = line_vec[1].as_bytes();
                    let reverse_primer = line_vec[2].as_bytes();
                    let expected_len = line_vec[3].parse::<usize>().unwrap();
                    let margin = line_vec[4].parse::<usize>().unwrap();

                    let primer_pair = PrimerPair {
                        primer_name: primer_name.to_owned(),
                        forward_primer: forward_primer.to_owned(),
                        reverse_primer: reverse_primer.to_owned(),
                        expected_len: expected_len,
                        margin: margin,
                    };

                    primer_pairs.push(primer_pair);
                }
                _ => continue,
            }
        }
    }

    return primer_pairs;
}

#[allow(unused)]
fn main() {
    let args = Args::parse();

    let primer_pairs = parse_primer_file(&args.primers);

    let fasta_reader = Reader::from_file(&args.fasta).unwrap();

    let records = fasta_reader.records();

    records.for_each(|record| {
        if let Ok(record) = record {
            for primer_pair in &primer_pairs {
                let amplicons = amplicon_search(record.seq(), primer_pair);

                for amplicon in &amplicons {
                    println!(
                        "{}\t{}\t{}",
                        record.id(),
                        primer_pair.primer_name,
                        std::str::from_utf8(amplicon).unwrap()
                    )
                }
            }
        }
    });
}
