/// A register in a quantum circuit.
#[derive(Debug)]
pub struct Register {
    pub name: String,
    pub size: usize,
}
