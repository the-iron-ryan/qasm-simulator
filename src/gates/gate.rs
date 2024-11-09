use crate::quantum::state::State;
use std::string::String;

/// Trait that defines the behavior of a quantum gate
pub trait Gate {
    /// Apply the gate to the state
    fn apply(&self, state: &mut State);
}

pub enum GateType {
    H { target: usize },
    X { target: usize },
    T { target: usize },
    TDgr { target: usize },
    CX { control: usize, target: usize },
}

impl Gate for GateType {
    fn apply(&self, state: &mut State) {
        match self {
            GateType::H { target } => {
                println!("Applying H gate to qubit {}", target);
            }
            GateType::X { target } => {
                println!("Applying X gate to qubit {}", target);
            }
            GateType::T { target } => {
                println!("Applying T gate to qubit {}", target);
            }
            GateType::TDgr { target } => {
                println!("Applying T^dagger gate to qubit {}", target);
            }
            GateType::CX { control, target } => {
                println!(
                    "Applying CX gate with control qubit {} and target qubit {}",
                    control, target
                );
            }
        }
    }
}
