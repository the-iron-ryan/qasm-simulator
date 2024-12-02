use num::Complex;

use crate::quantum::{common::Equivalency, ket::Ket, state::State};
use std::{f64::consts::PI, mem, string::String};

/// Enum representing all supported quantum gates.
#[derive(Debug, PartialEq)]
pub enum Gate {
    H { target: usize },
    X { target: usize },
    T { target: usize },
    TDgr { target: usize },
    CX { control: usize, target: usize },
    Toffoli { controls: Vec<usize>, target: usize },

    Composite { gates: Vec<Gate> },
}

/// Enum representing the result of applying a gate to a ket.
pub enum GateKetResult {
    Ket(Ket),
    Kets(Vec<Ket>),
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

            GateKetResult::Kets(vec![ket, flipped_ket])
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
        Gate::Composite { gates } => {
            let mut cur_kets: Vec<Ket> = vec![ket];
            let mut result_kets: Vec<Ket> = Vec::new();
            for gate in gates.iter() {
                while let Some(ket) = cur_kets.pop() {
                    match apply_gate_to_ket(gate, ket) {
                        GateKetResult::Ket(k) => {
                            result_kets.push(k);
                        }
                        GateKetResult::Kets(mut kets) => {
                            result_kets.append(&mut kets);
                        }
                        GateKetResult::NotImplemented(_) => {
                            panic!("Gate not implemented.");
                        }
                    }
                }

                // Swap our ket pointers
                cur_kets = mem::take(&mut result_kets);
            }
            if cur_kets.len() == 1 {
                GateKetResult::Ket(cur_kets.pop().unwrap())
            } else {
                GateKetResult::Kets(cur_kets)
            }
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
            GateKetResult::Kets(kets) => {
                for ket in kets {
                    new_state.add_or_insert(ket);
                }
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
        assert!(ket1.are_equivalent(ket2));
    }

    /// Helper function to assert if a ket is within a vector of kets.
    fn assert_contains_ket(kets: &Vec<Ket>, contains_ket: &Ket) {
        for ket in kets {
            if ket.are_equivalent(contains_ket) {
                return;
            }
        }
        assert!(false);
    }

    /// Helper function to assert that two states are equal.
    fn assert_state_eq(state1: &State, state2: &State) {
        assert!(state1.are_equivalent(state2));
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
            GateKetResult::Kets(kets) => {
                assert_contains_ket(&kets, &expected_ket1);
                assert_contains_ket(&kets, &expected_ket2);
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
    
    fn apply_composite_gate_to_ket() {}

    #[test]
    fn apply_composite_gate_to_state_single_ket() {
        let mut state = State::new(2);
        state.add_or_insert(Ket::from_bit_vec(bitvec![0, 0], Complex::new(1.0, 0.0)));
        let gate = Gate::Composite {
            gates: vec![Gate::X { target: 0 }, Gate::X { target: 1 }],
        };

        let new_state = apply_gate_to_state(state, &gate);

        let expected_state = State::from_ket_vec(&vec![Ket::from_bit_vec(
            bitvec![1, 1],
            Complex::new(1.0, 0.0),
        )]);

        assert_state_eq(&new_state, &expected_state);
    }
}
