use num::complex::Complex;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Ket {
    amplitude: Complex<i64>,
    num_qubits: usize,
    bits: Box<[bool]>,
}

#[macro_export]
macro_rules! ket_arr {
    ($x:expr ) => {
        vec![false; $x].into_boxed_slice()
    };
}

impl Ket {
    fn new<T: Into<usize>>(num_qubits: T) -> Ket {
        let num_qubits_size = num_qubits.into();
        Ket {
            amplitude: Complex::new(0, 0),
            num_qubits: num_qubits_size,
            bits: ket_arr!(num_qubits_size),
        }
    }
}
