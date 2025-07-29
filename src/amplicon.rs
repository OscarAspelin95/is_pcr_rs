use crate::args::Args;
use crate::search::amplicon_search;
use crate::utils::parse_primer_file;
use bio::io::fasta::Reader;
use indicatif::{ProgressBar, ProgressStyle};
use log::info;

use std::{
    fs::File,
    io::{BufWriter, Write},
    time::Duration,
};

pub fn amplicon(args: &Args) {
    // Read and parse primer file.
    let primer_pairs = parse_primer_file(&args.primers);

    let fasta_reader = Reader::from_file(&args.fasta).unwrap();

    // Initialize writer to which we write results.
    let mut writer =
        BufWriter::new(File::create(&args.outfile).expect("Failed to create output file."));

    // Write tsv header.
    writer
        .write_all(format!("{}\t{}\t{}\n", "sequence_id", "primer_name", "amplicon").as_bytes())
        .unwrap();

    let records = fasta_reader.records();

    info!("Finding amplicons...");
    let spinner: ProgressBar = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(200));
    spinner.set_style(ProgressStyle::with_template("{spinner:.blue} [{elapsed_precise}]").unwrap());

    records.for_each(|record| {
        if let Ok(record) = record {
            for primer_pair in &primer_pairs {
                let amplicons = amplicon_search(record.seq(), primer_pair);

                let result_vec: Vec<String> = amplicons
                    .iter()
                    .map(|amplicon| {
                        format!(
                            "{}\t{}\t{}\n",
                            record.id(),
                            primer_pair.primer_name,
                            std::str::from_utf8(amplicon).unwrap()
                        )
                    })
                    .collect();

                writer.write_all(result_vec.join("").as_bytes()).unwrap();
            }
        }
    });

    spinner.finish();
}
