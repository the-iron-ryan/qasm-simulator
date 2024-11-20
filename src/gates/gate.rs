use num::Complex;

use crate::quantum::{ket::Ket, state::State};
use std::{f64::consts::PI, string::String};

/// Enum representing all supported quantum gates.
pub enum Gate {
    H { target: usize },
    X { target: usize },
    T { target: usize },
    TDgr { target: usize },
    CX { control: usize, target: usize },
    Toffoli { controls: Vec<usize>, target: usize },
}

/// Struct representing a composite gate composed of multiple basis gates.
pub struct CompositeGate {
    gates: Vec<Gate>,
}

impl CompositeGate {
    /// Creates a new `CompositeGate` with the given gates.
    ///
    /// # Examples
    /// ```
    /// use quantum_simulator::gates::gate::{CompositeGate, Gate};
    ///
    /// let gates = vec![
    ///    Gate::H { target: 0 },
    ///    Gate::X { target: 1 },
    /// ];
    /// let composite_gate = CompositeGate::new(gates);
    /// ```
    pub fn new(gates: Vec<Gate>) -> Self {
        Self { gates }
    }

    pub fn add_gate(&mut self, gate: Gate) {
        self.gates.push(gate);
    }

    pub fn apply_to_ket(&self, ket: Ket) -> Ket {
        let mut new_ket = ket;
        for gate in self.gates.iter() {
            match apply_gate_to_ket(gate, new_ket) {
                GateKetResult::Ket(k) => {
                    new_ket = k;
                }
                _ => panic!("Composite gate must only contain single qubit gates."),
            }
        }
        new_ket
    }

    pub fn apply_to_state(&self, state: State) -> State {
        let mut new_state = state;
        for gate in self.gates.iter() {
            new_state = apply_gate_to_state(new_state, gate);
        }
        new_state
    }
}

/// Enum representing the result of applying a gate to a ket.
pub enum GateKetResult {
    Ket(Ket),
    Kets([Ket; 2]),
    NotImplemented(String),
}

/// Apply a gate to a ket.
///
/// # Examples
/// ```
/// use num::complex::Complex;
/// use quantum_simulator::gates::gate::{apply_gate_to_ket, Gate, GateKetResult};
/// use quantum_simulator::quantum::ket::Ket;
/// use bitvec::prelude::*;
///
/// let ket = Ket::new_zero_ket(1);
/// let gate = Gate::H { target: 0 };
/// let result = apply_gate_to_ket(&gate, ket);
///
/// let expected_ket1 = Ket::from_bit_vec(bitvec![0], Complex::new(1.0 / 2.0_f64.sqrt(), 0.0));
/// let expected_ket2 = Ket::from_bit_vec(bitvec![1], Complex::new(1.0 / 2.0_f64.sqrt(), 0.0));
///
/// match result {
///    GateKetResult::Kets([ket1, ket2]) => {
///      assert_eq!(ket1, expected_ket1);
///      assert_eq!(ket2, expected_ket2);
///   }
///  _ => panic!("Expected two kets."),
/// }
/// ```
pub fn apply_gate_to_ket(gate: &Gate, mut ket: Ket) -> GateKetResult {
    match gate {
        Gate::H { target } => {
            let mut flipped_ket = ket.clone();
            flipped_ket.flip(*target);

            if ket.get(*target) {
                ket.amplitude *= -1.0;
            }

            ket.amplitude *= 1.0 / 2.0_f64.sqrt();
            flipped_ket.amplitude *= 1.0 / 2.0_f64.sqrt();

            GateKetResult::Kets([ket, flipped_ket])
        }
        Gate::X { target } => {
            ket.flip(*target);
            GateKetResult::Ket(ket)
        }
        Gate::T { target } => {
            if ket.get(*target) {
                ket.amplitude *= Complex::new(0.0, 1.0 * PI / 4.0).exp();
            }

            GateKetResult::Ket(ket)
        }
        Gate::TDgr { target } => {
            if ket.get(*target) {
                ket.amplitude *= Complex::new(0.0, -1.0 * PI / 4.0).exp();
            }

            GateKetResult::Ket(ket)
        }
        Gate::CX { control, target } => {
            if ket.get(*control) {
                ket.flip(*target);
            }

            GateKetResult::Ket(ket)
        }
        Gate::Toffoli { controls, target } => {
            for control in controls {
                // If any control is zero, do nothing.
                if !ket.get(*control) {
                    return GateKetResult::Ket(ket);
                }
            }
            
            // If all controls are one, flip the target.
            ket.flip(*target);
            GateKetResult::Ket(ket)
        }
    }
}

/// Apply a gate to a state.
///
/// # Examples
/// ```
/// use num::complex::Complex;
/// use quantum_simulator::gates::gate::{apply_gate_to_state, Gate};
/// use quantum_simulator::quantum::ket::Ket;
/// use quantum_simulator::quantum::state::State;
/// use bitvec::prelude::*;
///
/// let mut state = State::new(1);
/// state.add_or_insert(Ket::new_zero_ket(1));
/// let gate = Gate::H { target: 0 };
/// let superposition_state = apply_gate_to_state(state, &gate);
///
/// let expected_ket1 = Ket::from_bit_vec(bitvec![0], Complex::new(1.0 / 2.0_f64.sqrt(), 0.0));
/// let expected_ket2 = Ket::from_bit_vec(bitvec![1], Complex::new(1.0 / 2.0_f64.sqrt(), 0.0));
/// let expected_superposition_state = State::from_ket_vec(&vec![expected_ket1, expected_ket2]);
/// assert_eq!(superposition_state, expected_superposition_state);
/// ```
pub fn apply_gate_to_state(state: State, gate: &Gate) -> State {
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

#[cfg(test)]
mod tests {

    use super::*;
    use bitvec::prelude::*;
    use num::Complex;

    /// Helper function to assert that two kets are equal.
    fn assert_ket_eq(ket1: &Ket, ket2: &Ket) {
        assert_eq!(ket1.amplitude, ket2.amplitude);
        assert_eq!(ket1.bit_vec(), ket2.bit_vec());
    }

    /// Helper function to assert that two states are equal.
    fn assert_state_eq(state1: &State, state2: &State) {
        assert_eq!(state1.num_qubits(), state2.num_qubits());
        assert_eq!(state1.kets.len(), state2.kets.len());
        for ket in state1.kets.iter() {
            assert!(state2.kets.contains(ket));
        }
    }

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
                assert_ket_eq(&ket1, &expected_ket1);
                assert_ket_eq(&ket2, &expected_ket2);
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

        assert_state_eq(&superposition_state, &expected_superposition_state);

        let back_to_zero_state = apply_gate_to_state(superposition_state, &gate);
        let expected_zero_state = State::from_ket_vec(&vec![Ket::new_zero_ket(1)]);

        assert_state_eq(&back_to_zero_state, &expected_zero_state);
    }

    /// Test to apply an X gate to a ket.
    #[test]
    fn test_apply_x_to_ket() {
        let ket = Ket::from_bit_vec(bitvec![0, 0], Complex::new(1.0, 0.0));
        let gate = Gate::X { target: 1 };
        let result = apply_gate_to_ket(&gate, ket);

        let expected_ket = Ket::from_bit_vec(bitvec![0, 1], Complex::new(1.0, 0.0));
        match result {
            GateKetResult::Ket(ket) => {
                assert_ket_eq(&ket, &expected_ket);
            }
            _ => panic!("Expected one ket."),
        }
    }

    /// Test to apply an X gate to a state.
    #[test]
    fn test_apply_x_to_gate() {
        let mut state = State::new(2);
        state.add_or_insert(Ket::from_bit_vec(bitvec![0, 0], Complex::new(1.0, 0.0)));
        let gate = Gate::X { target: 1 };

        let new_state = apply_gate_to_state(state, &gate);

        let expected_ket = Ket::from_bit_vec(bitvec![0, 1], Complex::new(1.0, 0.0));
        let expected_state = State::from_ket_vec(&vec![expected_ket]);

        assert_state_eq(&new_state, &expected_state);
    }

    /// Test to apply a T gate to a ket.
    #[test]
    fn tets_apply_t_to_ket() {
        let ket = Ket::from_bit_vec(bitvec![1], Complex::new(1.0, 0.0));
        let gate = Gate::T { target: 0 };
        let result = apply_gate_to_ket(&gate, ket);

        let expected_ket = Ket::from_bit_vec(
            bitvec![1],
            Complex::new(1.0, 0.0) * Complex::new(0.0, 1.0 * PI / 4.0).exp(),
        );
        match result {
            GateKetResult::Ket(ket) => {
                assert_ket_eq(&ket, &expected_ket);
            }
            _ => panic!("Expected one ket."),
        }
    }

    /// Test to apply a T gate to a state.
    #[test]
    fn test_apply_t_to_gate() {
        let mut state = State::new(1);
        state.add_or_insert(Ket::from_bit_vec(bitvec![1], Complex::new(1.0, 0.0)));
        let gate = Gate::T { target: 0 };

        let new_state = apply_gate_to_state(state, &gate);

        let expected_ket = Ket::from_bit_vec(
            bitvec![1],
            Complex::new(1.0, 0.0) * Complex::new(0.0, 1.0 * PI / 4.0).exp(),
        );
        let expected_state = State::from_ket_vec(&vec![expected_ket]);

        assert_state_eq(&new_state, &expected_state);
    }

    /// Test to apply a TDgr gate to a ket.
    #[test]
    fn tets_apply_tdgr_to_ket() {
        let ket = Ket::from_bit_vec(bitvec![1], Complex::new(1.0, 0.0));
        let gate = Gate::TDgr { target: 0 };
        let result = apply_gate_to_ket(&gate, ket);

        let expected_ket = Ket::from_bit_vec(
            bitvec![1],
            Complex::new(1.0, 0.0) * Complex::new(0.0, -1.0 * PI / 4.0).exp(),
        );
        match result {
            GateKetResult::Ket(ket) => {
                assert_ket_eq(&ket, &expected_ket);
            }
            _ => panic!("Expected one ket."),
        }
    }

    /// Test to apply a TDgr gate to a state.
    #[test]
    fn test_apply_tdgr_to_state() {
        let mut state = State::new(1);
        state.add_or_insert(Ket::from_bit_vec(bitvec![1], Complex::new(1.0, 0.0)));
        let gate = Gate::TDgr { target: 0 };

        let new_state = apply_gate_to_state(state, &gate);

        let expected_ket = Ket::from_bit_vec(
            bitvec![1],
            Complex::new(1.0, 0.0) * Complex::new(0.0, -1.0 * PI / 4.0).exp(),
        );

        let expected_state = State::from_ket_vec(&vec![expected_ket]);

        assert_state_eq(&new_state, &expected_state);
    }

    /// Test to apply a CX gate to a ket.
    #[test]
    fn test_apply_cx_to_ket() {
        let ket = Ket::from_bit_vec(bitvec![1, 0], Complex::new(1.0, 0.0));
        let gate = Gate::CX {
            control: 0,
            target: 1,
        };
        let result = apply_gate_to_ket(&gate, ket);

        let expected_ket = Ket::from_bit_vec(bitvec![1, 1], Complex::new(1.0, 0.0));
        match result {
            GateKetResult::Ket(ket) => {
                assert_ket_eq(&ket, &expected_ket);
            }
            _ => panic!("Expected one ket."),
        }
    }

    #[test]
    fn test_apply_cx_to_state() {
        let mut state = State::new(2);
        state.add_or_insert(Ket::from_bit_vec(bitvec![1, 1], Complex::new(1.0, 0.0)));
        let gate = Gate::CX {
            control: 0,
            target: 1,
        };

        let new_state = apply_gate_to_state(state, &gate);

        let expected_ket = Ket::from_bit_vec(bitvec![1, 0], Complex::new(1.0, 0.0));
        let expected_state = State::from_ket_vec(&vec![expected_ket]);

        assert_state_eq(&new_state, &expected_state);
    }
}
