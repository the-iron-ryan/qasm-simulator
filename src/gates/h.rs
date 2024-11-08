use crate::gates::gate::Gate;
use crate::gates::gate::GateData;
use crate::quantum::state::State;

pub struct H {
    data: GateData,
}

impl H {
    pub fn new() -> Self {
        Self {
            data: GateData {
                name: String::from("H"),
            },
        }
    }
}

impl Gate for H {
    fn apply(&self, state: &mut State) {
        println!("Applying H gate");
    }
}
