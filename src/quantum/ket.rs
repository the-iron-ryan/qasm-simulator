use num::complex::Complex;

/// A struct that contains data for a binary ket vector
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Ket {
    pub amplitude: Complex<i64>,
    pub num_qubits: usize,
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
    /// use crate::quantum::ket::Ket;
    ///
    /// let ket = Ket::new(3, Complex<i64>::new(1,0), [false, true, false]);
    /// assert_eq!(ket.amplitude, Complex<i64>::new(1,0));
    /// ```
    fn new<T: Into<usize>>(num_qubits: T, amplitude: Complex<i64>, ket_bits: Box<[bool]>) -> Ket {
        let num_qubits_size = num_qubits.into();
        Ket {
            amplitude,
            num_qubits: num_qubits_size,
            bits: ket_bits,
        }
    }
    fn new_zero_ket<T: Into<usize>>(num_qubits: T) -> Ket {
        let num_qubits_size = num_qubits.into();
        Ket {
            amplitude: Complex::new(0, 0),
            num_qubits: num_qubits_size,
            bits: ket_arr!(num_qubits_size),
        }
    }

    fn flip(&mut self, index: usize) {
        self.bits[index] = !self.bits[index];
    }

    fn get(&self, index: usize) -> bool {
        self.bits[index]
    }
}
