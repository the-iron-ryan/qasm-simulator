use bitvec::prelude::*;
use num::complex::Complex;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Ket {
    pub amplitude: Complex<f64>,
    bits: BitVec<u8>,
}

/// Helper macro used to construct bit data for a `Ket`
#[macro_export]
macro_rules! ket_bit_vec {
    ($x:expr ) => {
        bitvec![u8, Lsb0; $x]
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
    /// let ket_bits = 0b0000_0100;
    /// let ket = Ket::new(ket_bits, Complex::new(1.0, 0.0));
    /// assert_eq!(ket.amplitude, Complex::new(1.0, 0.0));
    /// assert_eq!(ket.bits, ket_bits);
    /// ```
    pub fn new(ket_bits: u8, amplitude: Complex<f64>) -> Ket {
        Ket {
            amplitude,
            bits: BitVec::from_element(ket_bits),
        }
    }

    pub fn from_bit_vec(ket_bits: BitVec<u8>, amplitude: Complex<f64>) -> Ket {
        Ket {
            amplitude,
            bits: ket_bits,
        }
    }

    pub fn from_bit_slice(ket_bits: &BitSlice<u8>, amplitude: Complex<f64>) -> Ket {
        Ket {
            amplitude,
            bits: BitVec::from_bitslice(ket_bits),
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
    pub fn new_zero_ket() -> Ket {
        Ket {
            amplitude: Complex::new(1.0, 0.0),
            bits: BitVec::from_element(0b0000_0000),
        }
    }

    /// Get an immutable bitvector reference to the underlying bits
    ///
    /// # Examples
    ///
    /// ```
    /// use num::complex::Complex;
    /// use quantum_simulator::quantum::ket::Ket;
    ///
    ///
    /// let ket = Ket::new(0b0000_0100, Complex::new(1.0, 0.0));
    /// let bit_vec = ket.bit_vec();
    ///
    ///
    /// assert_eq!(bit_vec.value(), 0b0000_0100);
    ///
    /// ```
    pub fn bit_vec(&self) -> &BitVec<u8> {
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
        if let Some(bit) = self.bits.get(index) {
            return *bit;
        } else {
            panic!(
                "Index out of bounds. Needs to be less than {}",
                self.bits.len()
            );
        }
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
        let cur_val = self.get(index);
        self.bits.set(index, !cur_val);
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
