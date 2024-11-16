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

pub enum GateResult {
    Ket(Ket),
    Kets([Ket; 2]),
    NotImplemented(String),
}

pub fn apply_gate_to_ket(gate: Gate, mut ket: Ket) -> GateResult {
    match gate {
        Gate::H { target } => {
            let mut flipped_ket = ket.clone();
            if ket.get(target) {
                ket.amplitude *= Complex::new(-1.0, 0.0);
            }
            ket.amplitude *= Complex::new(1.0 / 2.0_f64.sqrt(), 0.0);
            flipped_ket.amplitude *= Complex::new(1.0 / 2.0_f64.sqrt(), 0.0);
            GateResult::Kets([ket, flipped_ket])
        }
        Gate::X { target } => {
            ket.flip(target);
            GateResult::Ket(ket)
        }
        Gate::T { target } => {
            if ket.get(target) {
                ket.amplitude *= Complex::new(1.0 * PI / 4.0, 0.0).exp();
            }
            GateResult::Ket(ket)
        }
        Gate::TDgr { target } => {
            if ket.get(target) {
                ket.amplitude *= Complex::new(-1.0 * PI / 4.0, 0.0).exp();
            }
            GateResult::Ket(ket)
        }
        Gate::CX { control, target } => {
            if ket.get(control) {
                ket.flip(target);
            }
            GateResult::Ket(ket)
        }
    }
}

fn apply_gate_to_state(mut state: State, gate: Gate) -> State {
    let mut new_state = State::new(state.num_qubits());
    for ket in state.kets() {
        match apply_gate_to_ket(gate, ket) {
            GateResult::Ket(new_ket) => {
                new_state.add_or_insert(new_ket);
            }
            GateResult::Kets([new_ket1, new_ket2]) => {
                new_state.add_or_insert(new_ket1);
                new_state.add_or_insert(new_ket2);
            }
            GateResult::NotImplemented(_) => {
                panic!("Gate not implemented.");
            }
        }
    }
    new_state
}
