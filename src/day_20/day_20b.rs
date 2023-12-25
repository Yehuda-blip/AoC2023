use std::collections::VecDeque;

use anyhow::{Result, anyhow};
use itertools::Itertools;

use crate::day_20::day_20a::Gate;

use super::day_20a::{build_circuit, Signal, Pulse, Processor};

pub fn solve(input: &String) -> Result<String> {
    let mut circuit = build_circuit(input).map_err(|e| anyhow!(e))?;

    circuit.iter().for_each(|(id, gate)| {
        match gate {
            Gate::C(c) => {println!("{}: {:?}", c.id, c.memory.iter().map(|g| g.0.clone()).collect_vec())},
            _ => {}
        }
    });
    
    // unreachable!()
    Err(anyhow!(""))
}
