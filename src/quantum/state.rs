use crate::{gates::gate::Gate, gates::gate::GateType, ket_arr, quantum::ket::Ket};
use num::complex::Complex;
use std::collections::HashSet;

#[derive(Debug)]
pub struct State {
    kets: HashSet<Ket>,
    num_qubits: usize,
}

impl State {
    /// Creates a new `State` with the given number of qubits.
    ///
    /// # Examples
    /// ```
    /// use quantum_simulator::quantum::state::State;
    /// use num::complex::Complex;
    ///
    /// let state = State::new(3);
    /// assert_eq!(state.num_qubits(), 3);
    /// for ket in state.kets() {
    ///    assert_eq!(ket.num_qubits(), 3);
    ///    assert_eq!(ket.amplitude, Complex::new(1.0, 0.0));
    /// }
    /// ```
    pub fn new(num_qubits: usize) -> Self {
        if num_qubits == 0 {
            return Self {
                kets: HashSet::new(),
                num_qubits: 0,
            };
        } else {
            return Self {
                kets: HashSet::from_iter(vec![Ket::new_zero_ket(num_qubits)]),
                num_qubits,
            };
        }
    }

    /// Returns the number of qubits in this state.
    ///
    /// # Examples
    /// ```
    /// use quantum_simulator::quantum::state::State;
    ///
    /// let state = State::new(5);
    /// assert_eq!(state.num_qubits(), 5);
    /// ```
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Returns a non mutable reference to the kets in this state.
    ///
    /// # Examples
    /// ```
    /// use quantum_simulator::quantum::state::State;
    /// use quantum_simulator::quantum::ket::Ket;
    ///
    /// let state = State::new(5);
    /// let state_kets = state.kets();
    /// assert_eq!(state_kets.len(), 1);
    /// assert!(state_kets.contains(&Ket::new_zero_ket(5)));
    /// ```
    pub fn kets(&self) -> &HashSet<Ket> {
        &self.kets
    }

    /// Adds a new `Ket` to this state or adds to the amplitude if the ket
    /// already exists.
    fn add_or_insert(&mut self, ket: Ket) {
        // Ignore inserting a ket with zero amplitude.
        if ket.amplitude.norm() == 0.0 {
            return;
        }

        if let Some(mut found_ket) = self.kets.take(&ket) {
            found_ket.amplitude += ket.amplitude;

            // Only bother adding the ket back to the state if the amplitude is
            // non-zero.
            if found_ket.amplitude.norm() > 0.0 {
                self.kets.insert(found_ket);
            }
        } else {
            self.kets.insert(ket);
        }
    }

    /// Removes a `Ket` from this state, if present.
    fn remove(&mut self, ket: &Ket) {
        self.kets.remove(ket);
    }

    /// Removes all `Ket`s with zero amplitude from this state.
    fn remove_zero_amplitude_kets(&mut self) {
        self.kets.retain(|ket| ket.amplitude.norm() > 0.0);
    }

    fn apply_gate(&mut self, gate: &GateType) {
        gate.apply(self);
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.num_qubits == other.num_qubits && self.kets == other.kets
    }
}

mod tests {
    use super::*;

    #[test]
    /// Test that a new state with zero qubits creates an empty state.
    fn test_new_state_zero_qubits() {
        let state = State::new(0);
        assert!(state.kets.is_empty());
        assert!(state.num_qubits == 0);
    }

    /// Tests to add a basic Ket to the state.
    #[test]
    fn test_add_or_insert_basic() {
        let ket = Ket::new(ket_arr![1], Complex::new(0.5, 0.0));
        let mut state = State::new(1);
        state.add_or_insert(ket);

        let expected_ket = &Ket::new(ket_arr!(1), Complex::new(1.5, 0.0));
        assert!(state.kets.contains(&expected_ket));
        if let Some(found_ket) = state.kets.take(expected_ket) {
            assert_eq!(found_ket.amplitude, expected_ket.amplitude);
        } else {
            panic!("Ket not found in state.");
        }
    }

    /// Tests that a zero amplitude Ket is not added to the state.
    #[test]
    fn test_add_or_insert_zero_amplitude() {
        let bit_arr = [false, true, false];
        let ket = Ket::new(Box::new(bit_arr), Complex::new(0.0, 0.0));
        let mut state = State::new(1);
        state.add_or_insert(ket);

        // Should only have the initial zero ket.
        assert!(state.kets.len() == 1);
    }

    /// Tests that a ket that creates a zero amplitude when added to
    /// the state is removed.
    #[test]
    fn test_add_or_insert_zero_amplitude_existing() {
        let ket = Ket::new(ket_arr![1], Complex::new(-1.0, 0.0));
        let mut state = State::new(1);
        state.add_or_insert(ket);

        assert!(state.kets.is_empty());
    }
}
