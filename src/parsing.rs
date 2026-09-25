use quantum_simulator::gates::gate::Gate;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum GateLineResult {
    SingleTarget {
        gate_name: String,
        target: usize,
    },
    SingleControl {
        gate_name: String,
        control: usize,
        target: usize,
    },
    MultiControl {
        gate_name: String,
        controls: Vec<usize>,
        target: usize,
    },
}
#[derive(Debug, PartialEq)]
pub enum GateLineError {
    ParsingError { message: String },
}
pub fn parse_gate_line(line: &str) -> Result<GateLineResult, GateLineError> {
    let mut registers: Vec<usize> = Vec::new();

    // Search for gate name among any controls.
    let gate_name_re = Regex::new(r"(?:ctrl\s*@*\s*)*(\w+)").unwrap();
    let gate_name = gate_name_re
        .captures(line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string();

    let gate_register_re = Regex::new(r"q\[*(\d+)\]*").unwrap();
    for (_, [index]) in gate_register_re
        .captures_iter(&line)
        .map(|cap| cap.extract())
    {
        registers.push(index.parse().unwrap());
    }

    if registers.len() == 1 {
        Ok(GateLineResult::SingleTarget {
            gate_name,
            target: registers[0],
        })
    } else if registers.len() == 2 {
        Ok(GateLineResult::SingleControl {
            gate_name,
            control: registers[0],
            target: registers[1],
        })
    } else if registers.len() > 2 {
        Ok(GateLineResult::MultiControl {
            gate_name,
            controls: registers[0..registers.len() - 1].to_vec(),
            target: registers[registers.len() - 1],
        })
    } else {
        Err(GateLineError::ParsingError {
            message: "Could not parse gate line.".to_string(),
        })
    }
}

pub struct GateResult {
    gate_name: String,
    gate: Gate,
}
pub enum GateError {
    GateNotFound { gate_name: String, message: String },
}

pub fn build_gate_from_line_result(line_result: GateLineResult) -> Result<GateResult, GateError> {
    match line_result {
        GateLineResult::SingleTarget { gate_name, target } => {
            match gate_name.to_lowercase().as_str() {
                "h" => Ok(GateResult {
                    gate_name,
                    gate: Gate::H { target },
                }),
                "x" => Ok(GateResult {
                    gate_name,
                    gate: Gate::X { target },
                }),
                "t" => Ok(GateResult {
                    gate_name,
                    gate: Gate::T { target },
                }),
                "tdg" => Ok(GateResult {
                    gate_name,
                    gate: Gate::TDgr { target },
                }),
                _ => Err(GateError::GateNotFound {
                    gate_name,
                    message: String::from("Unknown single target gate name."),
                }),
            }
        }
        GateLineResult::SingleControl {
            gate_name,
            control,
            target,
        } => match gate_name.to_lowercase().as_str() {
            "cx" => Ok(GateResult {
                gate_name,
                gate: Gate::CX { control, target },
            }),
            _ => Err(GateError::GateNotFound {
                gate_name,
                message: String::from("Unknown single control gate name."),
            }),
        },
        GateLineResult::MultiControl {
            gate_name,
            controls,
            target,
        } => match gate_name.to_lowercase().as_str() {
            "x" => Ok(GateResult {
                gate_name,
                gate: Gate::Toffoli { controls, target },
            }),
            _ => {
                panic!("Unknown multi control gate name: {gate_name}");
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gate_line() {
        let line = "h q[5]";
        let result = parse_gate_line(line);
        assert_eq!(
            result,
            Ok(GateLineResult::SingleTarget {
                gate_name: String::from("h"),
                target: 5
            })
        );

        let line = "cx q0, q1";
        let result = parse_gate_line(line);
        assert_eq!(
            result,
            Ok(GateLineResult::SingleControl {
                gate_name: String::from("cx"),
                control: 0,
                target: 1
            })
        );

        let line = "x ctrl @ q[0], q[1]";
        let result = parse_gate_line(line);
        assert_eq!(
            result,
            Ok(GateLineResult::SingleControl {
                gate_name: String::from("x"),
                control: 0,
                target: 1
            })
        );

        let line = "x ctrl @ ctrl @ q[0], q[1], q[2]";
        let result = parse_gate_line(line);
        assert_eq!(
            result,
            Ok(GateLineResult::MultiControl {
                gate_name: String::from("x"),
                controls: vec![0, 1],
                target: 2
            })
        );
    }
}
