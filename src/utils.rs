use log::warn;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum AmpliconError<'a> {
    #[error("Failed to open provided FASTA file")]
    FastaFileOpenError(&'a PathBuf),

    #[error("Failed to open provided primer file")]
    PrimerFileParsingError(&'a PathBuf),

    #[error("Invalid non-integer primer len '{value}'")]
    PrimerLenParsingError { value: String },

    #[error("Invalid primer line format")]
    PrimerLineFormatError,

    #[error("No valid primers found")]
    NoPrimersFoundError,
}

fn extract_primer_info(primer_line: &String) -> Result<PrimerPair, AmpliconError<'_>> {
    let line_vec: Vec<&str> = primer_line.split("\t").map(|l| l.trim()).collect();

    match line_vec.len() {
        5 => {
            // TODO - this is not optimal.
            let primer_name = line_vec[0];
            let forward_primer = line_vec[1].as_bytes();
            let reverse_primer = line_vec[2].as_bytes();
            let min_len =
                line_vec[3]
                    .parse::<usize>()
                    .map_err(|_| AmpliconError::PrimerLenParsingError {
                        value: line_vec[3].to_owned(),
                    })?;
            let max_len =
                line_vec[4]
                    .parse::<usize>()
                    .map_err(|_| AmpliconError::PrimerLenParsingError {
                        value: line_vec[4].to_owned(),
                    })?;

            Ok(PrimerPair {
                primer_name: primer_name.to_owned(),
                forward_primer: forward_primer.to_owned(),
                reverse_primer: reverse_primer.to_owned(),
                min_len: min_len,
                max_len: max_len,
            })
        }
        _ => Err(AmpliconError::PrimerLineFormatError),
    }
}

pub fn parse_primer_file<'a>(
    primer_file: &'a PathBuf,
) -> Result<Vec<PrimerPair>, AmpliconError<'a>> {
    let f =
        File::open(primer_file).map_err(|_| AmpliconError::PrimerFileParsingError(primer_file))?;

    let reader = BufReader::new(f);

    let mut primer_pairs: Vec<PrimerPair> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        if let Ok(primer_line) = line {
            match extract_primer_info(&primer_line) {
                Ok(primer_pair) => primer_pairs.push(primer_pair),
                Err(e) => warn!("{}, line: {} `{}`", e, i, primer_line),
            }
        }
    }

    match primer_pairs.len() {
        0 => Err(AmpliconError::NoPrimersFoundError),
        _ => Ok(primer_pairs),
    }
}
