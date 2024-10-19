use num::complex::Complex;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Ket {
    amp: Complex<i64>,
}
