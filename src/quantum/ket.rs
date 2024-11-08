use num::complex::Complex;
use ordered_float::OrderedFloat;
use std::hash::{Hash, Hasher};

/// A struct that contains data for a binary ket vector
#[derive(Debug)]
pub struct Ket {
    pub amplitude: Complex<f64>,
    bits: Box<[bool]>,
}

/// Helper macro used to construct boxed up data of ket data
#[macro_export]
macro_rules! ket_arr {
    ($x:expr ) => {
        vec![false; $x].into_boxed_slice()
    };
}

impl Ket {
    /// Creates a new `Ket` with the given number of qubits, amplitude, and bits in the
    /// corresponding states.
    ///
    /// # Examples
    ///
    /// ```
    /// use num::complex::Complex;
    /// use quantum_simulator::quantum::ket::Ket;
    ///
    /// let bit_arr = [false, true, false];
    /// let ket = Ket::new(Box::new(bit_arr), Complex::new(1.0, 0.0));
    /// assert_eq!(ket.num_qubits(), 3);
    /// assert_eq!(ket.amplitude, Complex::new(1.0, 0.0));
    /// assert_eq!(**ket.bits(), bit_arr);
    /// ```
    pub fn new(ket_bits: Box<[bool]>, amplitude: Complex<f64>) -> Ket {
        assert!(ket_bits.len() > 0);
        Ket {
            amplitude,
            bits: ket_bits,
        }
    }
    /// Creates a new `Ket` of size `num_qubits` with all bits set to 0 and
    /// an amplitude of 1.0.
    ///
    /// # Examples
    ///
    /// ```
    /// use num::complex::Complex;
    /// use quantum_simulator::quantum::ket::Ket;
    ///
    /// let ket = Ket::new_zero_ket(10);
    /// let expected_bit_arr = [false; 10];
    /// assert_eq!(ket.num_qubits(), 10);
    /// assert_eq!(ket.amplitude, Complex::new(1.0, 0.0));
    /// assert_eq!(**ket.bits(), expected_bit_arr)
    /// ```
    pub fn new_zero_ket(num_qubits: usize) -> Ket {
        assert!(num_qubits > 0);
        Ket {
            amplitude: Complex::new(1.0, 0.0),
            bits: ket_arr!(num_qubits),
        }
    }

    /// Get the number of qubits in the ket
    ///
    /// # Examples
    ///
    /// ```
    /// use num::complex::Complex;
    /// use quantum_simulator::quantum::ket::Ket;
    ///
    /// let ket = Ket::new_zero_ket(10);
    ///
    /// assert_eq!(ket.num_qubits(), 10);
    ///
    /// ```
    pub fn num_qubits(&self) -> usize {
        self.bits.len()
    }

    /// Get a borrowed immutable reference to the ket bitvector
    ///
    /// # Examples
    ///
    /// ```
    /// use num::complex::Complex;
    /// use quantum_simulator::quantum::ket::Ket;
    ///
    ///
    /// let bit_arr = [false, true, false];
    /// let ket = Ket::new(Box::new(bit_arr), Complex::new(1.0,1.0));
    ///
    /// assert_eq!(**ket.bits(), bit_arr)
    ///
    /// ```
    pub fn bits(&self) -> &Box<[bool]> {
        &self.bits
    }

    /// Gets a bit at the desired index.
    ///
    /// # Examples
    ///
    /// ```
    /// use num::complex::Complex;
    /// use quantum_simulator::quantum::ket::Ket;
    ///
    /// let mut ket = Ket::new_zero_ket(3);
    ///
    /// ket.flip(0);
    /// assert_eq!(ket.get(0), true);
    /// ```
    pub fn get(&self, index: usize) -> bool {
        self.bits[index]
    }

    /// Flips a bit at the desired index.
    ///
    /// # Examples
    ///
    /// ```
    /// use num::complex::Complex;
    /// use quantum_simulator::quantum::ket::Ket;
    ///
    /// let mut ket = Ket::new_zero_ket(1);
    ///
    /// assert_eq!(ket.get(0), false);
    /// ```
    ///
    pub fn flip(&mut self, index: usize) {
        self.bits[index] = !self.bits[index];
    }
}

impl PartialEq for Ket {
    fn eq(&self, other: &Self) -> bool {
        *self.bits == *other.bits
    }
}

impl Eq for Ket {}

impl Hash for Ket {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self.bits).hash(state);
    }
}

mod tests {}
