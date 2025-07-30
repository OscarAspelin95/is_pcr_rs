use crate::utils::{AmpliconResult, PrimerPair};
use memchr::memmem;

#[inline]
fn reverse_complement(seq: &[u8]) -> Vec<u8> {
    let reverse_complement: Vec<u8> = seq
        .iter()
        .rev()
        .map(|nt| match nt {
            b'A' => b'T',
            b'C' => b'G',
            b'G' => b'C',
            b'T' => b'A',
            _ => panic!(""),
        })
        .collect();

    return reverse_complement;
}

#[inline]
fn usize_sub(a: usize, b: usize) -> usize {
    if a > b {
        return a - b;
    }

    return 0;
}

#[allow(unused)]
pub fn amplicon_search<'a>(seq: &'a [u8], primer_pair: &PrimerPair) -> Vec<AmpliconResult<'a>> {
    let PrimerPair {
        primer_name,
        forward_primer,
        reverse_primer,
        min_len,
        max_len,
    } = primer_pair;

    let forward_len = forward_primer.len();
    let reverse_len: usize = reverse_primer.len();

    // For reverse primer, we need to 3' -> 5' direction.
    let reverse_complement_primer = reverse_complement(reverse_primer);

    // Find all occurrences of the forward and reverse primers in seq.
    let forward_hits: Vec<usize> = memmem::find_iter(seq, forward_primer).collect();
    let reverse_hits: Vec<usize> =
        memmem::find_iter(seq, reverse_complement_primer.as_slice()).collect();

    let mut amplicons: Vec<AmpliconResult> = Vec::new();

    // This is not ideal if we expect many, many matches.
    for forward_hit in &forward_hits {
        let start = forward_hit + forward_len;

        for reverse_hit in &reverse_hits {
            let insert_length: usize = usize_sub(*reverse_hit, start);

            // If amplicon is within allowed length.
            if insert_length >= *min_len && insert_length <= *max_len {
                let amplicon = &seq[start..*reverse_hit];

                let amplicon_result = AmpliconResult {
                    amplicon: amplicon,
                    start: start,
                    end: *reverse_hit,
                    insert_length: insert_length,
                    total_length: forward_len + insert_length + reverse_len,
                };

                amplicons.push(amplicon_result);
            }
        }
    }

    return amplicons;
}
