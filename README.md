⚠️ Depreciated since it is now part of [fasta_rs](https://github.com/OscarAspelin95/fasta_rs).

# is_pcr_rs
In-silico PCR through exact primer match.

## Requirements
- Linux OS (Ubuntu 24.04.2)
- Rust >= 1.88.0

## Installation
Clone the repository or download the source code. Enter the is_pcr_rs directory and run:<br>
`cargo build --release`

The generated binary is available in `target/release/is_pcr_rs`.

## Usage
 Requires a primer.tsv TAB separated file that specifies the following for each primer pair:
- Primer name.
- Forward primer sequence (5' -> 3').
- Reverse primer sequence (5' -> 3').
- Expected minimum length of insert size.
- Expected maximum length of insert size.

Run with:<br>
`is_pcr_rs --fasta <sequences.fasta> --primers <primer.tsv> --outfile <out.tsv>`

See the *example_data* directory, which contains the offical [Sars-Cov-2](https://www.ncbi.nlm.nih.gov/nuccore/NC_045512.2/) genome and primers for finding the CDC-N1 region.
