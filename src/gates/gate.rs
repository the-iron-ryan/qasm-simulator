use crate::quantum::state::State;
use std::string::String;

/// A struct that contains data for a gate
#[derive(Debug)]
pub struct GateData {
    pub name: String,
}

/// Trait that defines the behavior of a quantum gate
pub trait Gate {
    /// Returns the data of the gate
    fn get_data(&self) -> GateData;

    /// Returns the name of the gate
    fn get_name(&self) -> String {
        self.get_data().name.clone()
    }

    /// Apply the gate to the state
    fn apply(&self, state: &mut State);
}
