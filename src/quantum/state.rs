use crate::quantum::common::Equivalency;
use crate::quantum::ket::Ket;
use std::collections::HashSet;
use std::fmt;

use super::ket;

#[derive(Debug)]
pub struct State {
    pub kets: HashSet<Ket>,
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
    /// assert!(state.kets.is_empty());
    /// ```
    pub fn new(num_qubits: usize) -> Self {
        return Self {
            kets: HashSet::new(),
            num_qubits,
        };
    }

    /// Creates a new `State` from a vector of `Ket`s. Where all kets must have the same
    /// number of qubits.
    ///
    /// # Examples
    /// ```
    /// use quantum_simulator::quantum::state::State;
    /// use quantum_simulator::quantum::ket::Ket;
    /// use num::complex::Complex;
    /// use bitvec::prelude::*;
    ///
    /// let ket1 = Ket::from_bit_vec(bitvec![0, 0], Complex::new(1.0, 0.0));
    /// let ket2 = Ket::from_bit_vec(bitvec![0, 1], Complex::new(1.0, 0.0));
    /// let kets = vec![ket1, ket2];
    /// let state = State::from_ket_vec(&kets);
    /// assert_eq!(state.num_qubits(), 2);
    ///
    /// assert!(state.kets.contains(&kets[0]));
    /// assert!(state.kets.contains(&kets[1]));
    /// ```
    pub fn from_ket_vec(kets: &Vec<Ket>) -> Self {
        let num_qubits = kets[0].bit_vec().len();
        for ket in kets {
            if ket.bit_vec().len() != num_qubits {
                panic!("All kets must have the same number of qubits.");
            }
        }

        let mut state = State::new(num_qubits);
        for ket in kets {
            state.add_or_insert(ket.clone());
        }

        state
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

    /// Adds a new `Ket` to this state or adds to the amplitude if the ket
    /// already exists.
    pub fn add_or_insert(&mut self, ket: Ket) {
        // Ignore inserting a ket with zero amplitude.
        if ket.amplitude.norm() == 0.0 {
            return;
        }

        if let Some(mut found_ket) = self.kets.take(&ket) {
            found_ket.amplitude += ket.amplitude;

            // Only bother adding the ket back to the state if the amplitude is
            // non-zero.
            if found_ket.amplitude.norm() > 1e-6 {
                self.kets.insert(found_ket);
            }
        } else {
            self.kets.insert(ket);
        }
    }

    /// Removes a `Ket` from this state, if present.
    pub fn remove(&mut self, ket: &Ket) {
        self.kets.remove(ket);
    }

    /// Removes all `Ket`s with zero amplitude from this state.
    pub fn remove_zero_amplitude_kets(&mut self) {
        self.kets.retain(|ket| ket.amplitude.norm() > 0.0);
    }
}

impl Equivalency for State {
    /// Special check to see if two kets are considered equivalent.
    ///
    /// # Examples
    /// ```
    /// use num::complex::Complex;
    /// use quantum_simulator::quantum::ket::Ket;
    /// use quantum_simulator::quantum::state::State;
    /// use bitvec::prelude::*;
    /// use quantum_simulator::quantum::common::Equivalency;
    ///
    /// let ket1 = Ket::new_zero_ket(2);
    /// let ket2 = Ket::new_zero_ket(2);
    ///
    /// let state1 = State::from_ket_vec(&vec![ket1.clone(), ket2.clone()]);
    /// let state2 = State::from_ket_vec(&vec![ket2.clone(), ket1.clone()]);
    ///
    /// assert!(state1.are_equivalent(&state2));
    ///
    /// ```
    fn are_equivalent(&self, other: &Self) -> bool {
        if self.num_qubits != other.num_qubits {
            return false;
        }

        let mut our_ket_vec: Vec<&Ket> = self.kets.iter().collect();
        let mut other_ket_vec: Vec<&Ket> = other.kets.iter().collect();

        if our_ket_vec.len() != other_ket_vec.len() {
            return false;
        }

        // Sort the kets and check if each are equivalent.
        our_ket_vec.sort_by(|a, b| a.bit_vec().cmp(&b.bit_vec()));
        other_ket_vec.sort_by(|a, b| a.bit_vec().cmp(&b.bit_vec()));
        for (our_ket, other_ket) in our_ket_vec.iter().zip(other_ket_vec.iter()) {
            if !our_ket.are_equivalent(other_ket) {
                return false;
            }
        }
        return true;
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.num_qubits == other.num_qubits && self.kets == other.kets
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Order the kets by the underlying bit vector.
        let mut ket_vec: Vec<&Ket> = self.kets.iter().collect();
        ket_vec.sort_by(|a, b| a.bit_vec().cmp(&b.bit_vec()));

        let mut ket_iter = ket_vec.iter();
        if let Some(first_ket) = ket_iter.next() {
            write!(f, "{}", first_ket)?;
            for ket in ket_iter {
                write!(f, " + {}", ket)?;
            }
        }
        fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use bitvec::prelude::*;
    use num::complex::Complex;

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
        let ket = Ket::from_bit_vec(bitvec![0], Complex::new(0.5, 0.0));
        let mut state = State::new(1);
        state.add_or_insert(ket);

        let expected_ket = &Ket::from_bit_vec(bitvec![0], Complex::new(1.5, 0.0));
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
        let bit_vec = bitvec![0, 1, 0];
        let ket = Ket::from_bit_vec(bit_vec, Complex::new(0.0, 0.0));
        let mut state = State::new(1);
        state.add_or_insert(ket);

        // Should only have the initial zero ket.
        assert!(state.kets.len() == 1);
    }

    /// Tests that a ket that creates a zero amplitude when added to
    /// the state is removed.
    #[test]
    fn test_add_or_insert_zero_amplitude_existing() {
        let ket = Ket::from_bit_vec(bitvec![1], Complex::new(-1.0, 0.0));
        let mut state = State::new(1);
        state.add_or_insert(ket);

        assert!(state.kets.is_empty());
    }

    #[test]
    fn test_remove_ket() {
        let ket = Ket::from_bit_vec(bitvec![0], Complex::new(0.5, 0.0));
        let mut state = State::new(1);
        state.add_or_insert(ket.clone());

        state.remove(&ket);
        assert!(state.kets.is_empty());
    }

    #[test]
    fn test_remove_zero_amplitude_kets() {
        let ket1 = Ket::from_bit_vec(bitvec![0], Complex::new(0.5, 0.0));
        let ket2 = Ket::from_bit_vec(bitvec![1], Complex::new(0.0, 0.0));
        let mut state = State::new(1);
        state.add_or_insert(ket1);
        state.add_or_insert(ket2);

        state.remove_zero_amplitude_kets();
        assert!(state.kets.len() == 1);
    }

    #[test]
    fn test_fmt_display() {
        let ket1 = Ket::from_bit_vec(bitvec![0], Complex::new(0.5, 0.0));
        let ket2 = Ket::from_bit_vec(bitvec![1], Complex::new(0.5, 0.5));
        let state = State::from_ket_vec(&vec![ket1, ket2]);

        assert_eq!(format!("{}", state), "(0.5+0i)|0⟩ + (0.5+0.5i)|1⟩");
    }
}
