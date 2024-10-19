use crate::quantum::ket::Ket;
use std::collections::HashSet;

#[derive(Debug)]
pub struct State {
    kets: HashSet<Ket>,
    num_qubits: u32,
}

impl State {
    pub fn new(num_qubits: u32) -> Self {
        Self {
            kets: HashSet::new(),
            num_qubits,
        }
    }

    fn add_or_insert(ket_set: &mut HashSet<Ket>, ket: &Ket) {
        if let Some(mut found_ket) = ket_set.take(&ket) {
            found_ket.amp += ket.amp;
        }
        if ket_set.contains(ket) {}
    }
}
