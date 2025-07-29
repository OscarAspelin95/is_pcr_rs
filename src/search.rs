use crate::utils::PrimerPair;
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
pub fn amplicon_search<'a>(seq: &'a [u8], primer_pair: &PrimerPair) -> Vec<&'a [u8]> {
    let PrimerPair {
        primer_name,
        forward_primer,
        reverse_primer,
        min_len,
        max_len,
    } = primer_pair;

    let forward_len = forward_primer.len();

    // We'll find matches for the reverse complemented reverse primer.
    // Primers should always we written in 5' -> 3' direction.
    let reverse_complement_primer = reverse_complement(reverse_primer);

    // Find all occurrences of the forward and reverse primers.
    let forward_hits: Vec<usize> = memmem::find_iter(seq, forward_primer).collect();
    let reverse_hits: Vec<usize> =
        memmem::find_iter(seq, reverse_complement_primer.as_slice()).collect();

    // Store results
    let mut amplicons: Vec<&[u8]> = Vec::new();

    // This is not ideal if we expect many, many matches.
    for forward_hit in &forward_hits {
        let start = forward_hit + forward_len;

        for reverse_hit in &reverse_hits {
            let amplicon_len: usize = usize_sub(*reverse_hit, start);

            // If amplicon is within allowed length.
            if amplicon_len >= *min_len && amplicon_len <= *max_len {
                let amplicon = &seq[start..*reverse_hit];

                amplicons.push(amplicon);
            }
        }
    }

    return amplicons;
}
