use crate::gates::h::H;
use crate::quantum::ket::Ket;
use crate::quantum::state::State;
use std::string::String;

/// Trait that defines the behavior of a quantum gate
pub trait Gate {
    /// Apply the gate to the state
    fn apply(&self, state: &mut Ket);
}

pub enum GateType {
    H(H),
    X { target: usize },
    T { target: usize },
    TDgr { target: usize },
    CX { control: usize, target: usize },
}
