use crate::args::Args;
use crate::search::amplicon_search;
use crate::utils::AmpliconError;
use crate::utils::parse_primer_file;
use bio::io::fasta::Reader;
use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

use std::{
    fs::File,
    io::{BufWriter, Write},
    time::Duration,
};

fn write_header(writer: &Arc<Mutex<BufWriter<File>>>) {
    writer
        .lock()
        .unwrap()
        .write_all(
            format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                "sequence_id",
                "primer_name",
                "start",
                "end",
                "insert_length",
                "actual_length",
                "amplicon"
            )
            .as_bytes(),
        )
        .unwrap();
}

pub fn amplicon(args: &Args) -> Result<(), AmpliconError<'_>> {
    // Read and parse primer file.
    let primer_pairs = parse_primer_file(&args.primers)?;

    let fasta_reader = Reader::from_file(&args.fasta)
        .map_err(|_| AmpliconError::FastaFileOpenError(&args.fasta))?;

    // Initialize writer to which we write results.
    let writer = Arc::new(Mutex::new(BufWriter::new(
        File::create(&args.outfile).expect("Failed to create output file."),
    )));

    // Write tsv header.
    write_header(&writer);

    info!("Finding amplicons...");
    let spinner: ProgressBar = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(200));
    spinner.set_style(ProgressStyle::with_template("{spinner:.blue} [{elapsed_precise}]").unwrap());

    fasta_reader.records().par_bridge().for_each(|record| {
        if let Ok(record) = record {
            for primer_pair in &primer_pairs {
                let amplicons = amplicon_search(record.seq(), primer_pair);

                let result_vec: Vec<String> = amplicons
                    .iter()
                    .map(|amplicon| {
                        format!(
                            "{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                            record.id(),
                            primer_pair.primer_name,
                            amplicon.start,
                            amplicon.end,
                            amplicon.insert_length,
                            amplicon.total_length,
                            std::str::from_utf8(amplicon.amplicon).unwrap()
                        )
                    })
                    .collect();

                // Is is probably not ideal to write results after each primer pair.
                // A better approach would be to write after each record.
                if result_vec.len() > 0 {
                    writer
                        .lock()
                        .unwrap()
                        .write_all(result_vec.concat().as_bytes())
                        .unwrap();
                }
            }
        }
    });

    spinner.finish();

    Ok(())
}
