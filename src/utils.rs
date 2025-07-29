use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub struct AmpliconResult<'a> {
    pub amplicon: &'a [u8],
    pub start: usize,
    pub end: usize,
    pub insert_length: usize,
    pub total_length: usize,
}
pub struct PrimerPair {
    pub primer_name: String,
    pub forward_primer: Vec<u8>,
    pub reverse_primer: Vec<u8>,
    pub min_len: usize,
    pub max_len: usize,
}

pub fn parse_primer_file<'a>(primer_file: &'a PathBuf) -> Vec<PrimerPair> {
    let primer_buf =
        BufReader::new(File::open(primer_file).expect("Failed to open provided primer file."));

    let lines = primer_buf.lines();

    let mut primer_pairs: Vec<PrimerPair> = Vec::new();

    for line in lines {
        if let Ok(primer_line) = line {
            let line_vec: Vec<&str> = primer_line.split("\t").collect();

            match line_vec.len() {
                5 => {
                    // TODO - this sucks. Need to re-write.
                    let primer_name = line_vec[0];
                    let forward_primer = line_vec[1].as_bytes();
                    let reverse_primer = line_vec[2].as_bytes();
                    let min_len = line_vec[3].parse::<usize>().unwrap();
                    let max_len = line_vec[4].parse::<usize>().unwrap();

                    let primer_pair = PrimerPair {
                        primer_name: primer_name.to_owned(),
                        forward_primer: forward_primer.to_owned(),
                        reverse_primer: reverse_primer.to_owned(),
                        min_len: min_len,
                        max_len: max_len,
                    };

                    primer_pairs.push(primer_pair);
                }
                _ => continue,
            }
        }
    }

    return primer_pairs;
}
