use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

pub mod gates;
pub mod parsing;
pub mod quantum;

use parsing::parse_gate_line;
use quantum_simulator::gates::gate::{apply_gate_to_state, Gate};
use quantum_simulator::quantum::ket::Ket;
use quantum_simulator::quantum::register::Register;
use quantum_simulator::quantum::state::State;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    // let filename = "./qasm/f2_232.qasm";

    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut reader_lines = reader.lines().peekable();

    let mut line_number = 1;

    // Handle QASM version header.
    let header_re = Regex::new(r"OPENQASM\s+(\d+\.\d+)").unwrap();
    if let Some(Ok(header)) = reader_lines.next() {
        if let Some(caps) = header_re.captures(&header) {
            let version = caps.get(1).unwrap().as_str();
            println!("Using QASM version: {}", version);
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid header"));
        }
    }

    // Handle any includes.
    let include_re = Regex::new(r"^include.*").unwrap();
    while let Some(line_result) = reader_lines.peek() {
        line_number += 1;
        match line_result {
            Ok(line) => {
                if include_re.is_match(line) {
                    // For now, just skip the include and advance to the next line.
                    reader_lines.next();
                } else {
                    break;
                }
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!["Could not parse include on line {line_number}"],
                ));
            }
        }
    }

    // Search for register definitions.
    let register_re = Regex::new(r"(qreg|creg)\s([\w]+)(?:\[(\d+)\])").unwrap();
    let mut classical_register: Option<Register> = Option::None;
    let mut quantum_register: Option<Register> = Option::None;
    while let Some(line_result) = reader_lines.next() {
        line_number += 1;
        match line_result {
            Ok(line) => {
                if let Some(caps) = register_re.captures(&line) {
                    let (_, [register_type, register_name, register_size]) = caps.extract();
                    match register_type {
                        "qreg" => {
                            quantum_register = Option::Some(Register {
                                name: register_name.to_string(),
                                size: register_size.parse().unwrap(),
                            });
                        }
                        "creg" => {
                            classical_register = Option::Some(Register {
                                name: register_name.to_string(),
                                size: register_size.parse().unwrap(),
                            });
                        }
                        _ => {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                format![
                                    "Unknown register type '{register_type}' on line {line_number}"
                                ],
                            ));
                        }
                    }
                }
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!["Could not parse register on line {line_number}"],
                ));
            }
        }

        // Break if we have found both registers.
        if quantum_register.is_some() && classical_register.is_some() {
            break;
        }
    }

    // Parse any custom gates.
    let mut custom_gate_map: HashMap<String, Gate> = HashMap::new();
    let gate_start_re = Regex::new(r"(?m)^gate\s+(\w+)\s+([^{]*)\s*\{").unwrap();
    let gate_end_re = Regex::new(r"}").unwrap();
    let mut is_parsing_gate = false;
    let mut current_gate_name = String::new();
    for line_result in &mut reader_lines {
        line_number += 1;
        match line_result {
            Ok(line) => {
                if is_parsing_gate {
                    // Do gate parsing....
                } else if gate_start_re.is_match(&line) {
                    // Check if we can parse the gate.
                    if let Some(caps) = gate_start_re.captures(&line) {
                        current_gate_name = caps[1].to_string();
                        is_parsing_gate = true;
                    } else {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!["Could not parse gate on line {line_number}"],
                        ));
                    }
                } else if gate_end_re.is_match(&line) {
                    // End of gate parsing...
                    is_parsing_gate = false;
                } else {
                    // We're done parsing gates.
                    break;
                }
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!["Could not parse gate on line {line_number}"],
                ));
            }
        }
    }

    // Create a new quantum state.
    let mut state = match &quantum_register {
        Some(register) => {
            let num_qubits = register.size;

            println!("Simulating file {filename} with {num_qubits} qubits");

            let mut state = State::new(num_qubits);
            state.add_or_insert(Ket::new_zero_ket(num_qubits));
            state
        }
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "No quantum register was defined",
            ));
        }
    };

    // Handle instructions.

    // Creates three matching groups. One for the instruction, and two for the possible
    // qubit registers.
    let qreg_name = quantum_register.unwrap().name;
    let instruction_re_str =
        format![r"([a-z]+)\s(?:{qreg_name}\[([0-9]+)\])*(?:(?:,|\s){qreg_name}\[([0-9]+)\])*"];
    let instruction_re = Regex::new(&instruction_re_str).unwrap();
    let start = Instant::now();
    for line_result in &mut reader_lines {
        line_number += 1;
        match line_result {
            Ok(line) => {
                if let Some(caps) = instruction_re.captures(&line) {
                    let instruction = caps.get(1).unwrap().as_str();
                    let qubit1: Option<usize> =
                        caps.get(2).map(|qubit| qubit.as_str().parse().unwrap());
                    let qubit2: Option<usize> =
                        caps.get(3).map(|qubit| qubit.as_str().parse().unwrap());
                    match instruction {
                        "h" => {
                            state = apply_gate_to_state(
                                state,
                                &Gate::H {
                                    target: qubit1.unwrap(),
                                },
                            );
                        }
                        "x" => {
                            state = apply_gate_to_state(
                                state,
                                &Gate::X {
                                    target: qubit1.unwrap(),
                                },
                            );
                        }
                        "t" => {
                            state = apply_gate_to_state(
                                state,
                                &Gate::T {
                                    target: qubit1.unwrap(),
                                },
                            )
                        }
                        "tdg" => {
                            state = apply_gate_to_state(
                                state,
                                &Gate::TDgr {
                                    target: qubit1.unwrap(),
                                },
                            )
                        }
                        "cx" => {
                            state = apply_gate_to_state(
                                state,
                                &Gate::CX {
                                    control: qubit1.unwrap(),
                                    target: qubit2.unwrap(),
                                },
                            )
                        }
                        _ => {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                format![
                                    "Unknown instruction '{instruction}' on line {line_number}"
                                ],
                            ));
                        }
                    }
                }
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!["Could not parse data on line {line_number}"],
                ));
            }
        }
    }
    let duration = start.elapsed();

    println!("Final state: {}", state);
    println!("Execution time: {:?}\n", duration);

    Ok(())
}
