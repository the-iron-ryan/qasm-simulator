use crate::gates::gate::Gate;
use crate::quantum::ket::Ket;

pub struct H {
    target: usize,
}

impl Gate for H {
    fn apply(&self, state: &mut Ket) {
        println!("Applying H gate");
        
        ``
    }
}
