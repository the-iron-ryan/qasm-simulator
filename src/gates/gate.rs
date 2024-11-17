use bitvec::prelude::*;
use num::Complex;

use crate::quantum::{ket::Ket, state::State};
use std::{f64::consts::PI, string::String};

pub enum Gate {
    H { target: usize },
    X { target: usize },
    T { target: usize },
    TDgr { target: usize },
    CX { control: usize, target: usize },
}

pub enum GateKetResult {
    Ket(Ket),
    Kets([Ket; 2]),
    NotImplemented(String),
}

pub fn apply_gate_to_ket(gate: &Gate, mut ket: Ket) -> GateKetResult {
    match gate {
        Gate::H { target } => {
            let mut flipped_ket = ket.clone();
            flipped_ket.flip(*target);

            if ket.get(*target) {
                ket.amplitude *= Complex::new(-1.0, 0.0);
            }

            ket.amplitude *= Complex::new(1.0 / 2.0_f64.sqrt(), 0.0);
            flipped_ket.amplitude *= Complex::new(1.0 / 2.0_f64.sqrt(), 0.0);

            GateKetResult::Kets([ket, flipped_ket])
        }
        Gate::X { target } => {
            ket.flip(*target);
            GateKetResult::Ket(ket)
        }
        Gate::T { target } => {
            if ket.get(*target) {
                ket.amplitude *= Complex::new(1.0 * PI / 4.0, 0.0).exp();
            }

            GateKetResult::Ket(ket)
        }
        Gate::TDgr { target } => {
            if ket.get(*target) {
                ket.amplitude *= Complex::new(-1.0 * PI / 4.0, 0.0).exp();
            }

            GateKetResult::Ket(ket)
        }
        Gate::CX { control, target } => {
            if ket.get(*control) {
                ket.flip(*target);
            }

            GateKetResult::Ket(ket)
        }
    }
}

fn apply_gate_to_state(state: State, gate: &Gate) -> State {
    let mut new_state = State::new(state.num_qubits());
    for ket in state.kets {
        match apply_gate_to_ket(gate, ket) {
            GateKetResult::Ket(new_ket) => {
                new_state.add_or_insert(new_ket);
            }
            GateKetResult::Kets([new_ket1, new_ket2]) => {
                new_state.add_or_insert(new_ket1);
                new_state.add_or_insert(new_ket2);
            }
            GateKetResult::NotImplemented(_) => {
                panic!("Gate not implemented.");
            }
        }
    }
    new_state
}

mod tests {
    use super::*;

    /// Simple test to apply a Hadamard gate to a zero ket.
    #[test]
    fn test_apply_h_to_ket() {
        let ket = Ket::new_zero_ket(1);
        let gate = Gate::H { target: 0 };
        let result = apply_gate_to_ket(&gate, ket);

        let expected_ket1 = Ket::from_bit_vec(bitvec![0], Complex::new(1.0 / 2.0_f64.sqrt(), 0.0));
        let expected_ket2 = Ket::from_bit_vec(bitvec![1], Complex::new(1.0 / 2.0_f64.sqrt(), 0.0));
        match result {
            GateKetResult::Kets([ket1, ket2]) => {
                assert_eq!(ket1, expected_ket1);
                assert_eq!(ket2, expected_ket2);
            }
            _ => panic!("Expected two kets."),
        }
    }

    /// Round trip test to ensure a Hadarmard gate puts a state into superposition and then back.
    #[test]
    fn test_apply_h_to_state() {
        let mut state = State::new(1);
        state.add_or_insert(Ket::new_zero_ket(1));
        let gate = Gate::H { target: 0 };
        let superposition_state = apply_gate_to_state(state, &gate);

        let expected_ket1 = Ket::from_bit_vec(bitvec![0], Complex::new(1.0 / 2.0_f64.sqrt(), 0.0));
        let expected_ket2 = Ket::from_bit_vec(bitvec![1], Complex::new(1.0 / 2.0_f64.sqrt(), 0.0));
        let expected_superposition_state = State::from_ket_vec(&vec![expected_ket1, expected_ket2]);

        assert_eq!(superposition_state, expected_superposition_state);

        let back_to_zero_state = apply_gate_to_state(superposition_state, &gate);
        let expected_zero_state = State::from_ket_vec(&vec![Ket::new_zero_ket(1)]);
        
        assert_eq!(back_to_zero_state, expected_zero_state);
    }

    #[test]
    fn test_apply_gate_to_ket_x() {
        let ket = Ket::from_bit_vec(bitvec![0, 0], Complex::new(1.0, 0.0));
        let gate = Gate::X { target: 1 };
        let result = apply_gate_to_ket(&gate, ket);

        let expected_ket = Ket::from_bit_vec(bitvec![0, 1], Complex::new(1.0, 0.0));
        match result {
            GateKetResult::Ket(ket) => {
                assert_eq!(ket, expected_ket);
            }
            _ => panic!("Expected one ket."),
        }
    }
}
