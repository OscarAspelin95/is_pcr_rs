#[derive(Clone)]
pub struct PrimerPair {
    pub primer_name: String,
    pub forward_primer: Vec<u8>,
    pub reverse_primer: Vec<u8>,
    pub expected_len: usize,
    pub margin: usize,
}
