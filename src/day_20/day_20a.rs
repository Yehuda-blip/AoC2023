use std::collections::{HashMap, VecDeque};

use anyhow::{anyhow, Ok, Result};
use winnow::{
    combinator::alt,
    error::{ContextError, ErrMode},
    token::take_while,
    Parser,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super)enum Pulse {
    Low,
    High,
}

#[derive(Clone)]
pub(super) struct FlipFlop {
    id: String,
    on: bool,
    targets: Vec<String>,
}

#[derive(Clone)]
pub(super) struct Conjunction {
    pub id: String,
    pub memory: Vec<(String, Pulse)>,
    pub targets: Vec<String>,
}

#[derive(Clone)]
pub(super) struct Signal {
    pub source: String,
    pub target: String,
    pub value: Pulse,
}

#[derive(Clone)]
pub(super) enum Gate {
    F(FlipFlop),
    C(Conjunction),
    Broadcaster(Vec<String>),
}

impl Gate {
    pub(super) fn targets(&self) -> &Vec<String> {
        match self {
            Gate::Broadcaster(targets) => targets,
            Gate::C(c) => &c.targets,
            Gate::F(f) => &f.targets
        }
    }
}

impl Processor for Gate {
    fn process(&mut self, input: &Signal) -> Vec<Signal> {
        match self {
            Gate::C(c) => c.process(input),
            Gate::F(f) => f.process(input),
            Gate::Broadcaster(targets) => {
                targets.iter().map(|t_id| {
                    Signal { source: "".to_string(), target: t_id.to_string(), value: Pulse::Low }
                }).collect()
            }
        }
    }
}

pub(super) trait Processor {
    fn process(&mut self, input: &Signal) -> Vec<Signal>;
}

impl Processor for FlipFlop {
    fn process(&mut self, input: &Signal) -> Vec<Signal> {
        match input.value {
            Pulse::High => vec![],
            Pulse::Low => {
                self.on = !self.on;
                let value = match self.on {
                    true => Pulse::High,
                    false => Pulse::Low,
                };
                self.targets
                    .iter()
                    .map(|t_id| Signal {
                        source: self.id.clone(),
                        target: t_id.clone(),
                        value: value,
                    })
                    .collect()
            }
        }
    }
}

impl Processor for Conjunction {
    fn process(&mut self, input: &Signal) -> Vec<Signal> {
        let caller_index = self
            .memory
            .iter()
            .position(|(caller, _)| *caller == input.source)
            .unwrap();
        self.memory[caller_index].1 = input.value;
        let value = match self.memory.iter().all(|(_, p_val)| *p_val == Pulse::High) {
            true => Pulse::Low,
            false => Pulse::High,
        };
        self.targets
            .iter()
            .map(|t_id| Signal {
                source: self.id.clone(),
                target: t_id.clone(),
                value: value,
            })
            .collect()
    }
}

pub fn solve(input: &String) -> Result<String> {
    let mut circuit = build_circuit(input).map_err(|e| anyhow!(e))?;

    let (mut high_count, mut low_count): (u64, u64) = (0, 0);
    for _ in 0..1000 {
        low_count += 1;
        let mut queue: VecDeque<Signal> = circuit.get("broadcaster").unwrap().targets().iter().map(|tar| Signal { source: "".to_string(), target: tar.clone(), value: Pulse::Low }).collect();
        while let Some(sig) = queue.pop_front() {
            if let Some(processor) = circuit.get_mut(&sig.target) {
                let sigs_next = processor.process(&sig);
                queue.extend(sigs_next.into_iter());
            }

            match sig.value {
                Pulse::High => {high_count += 1},
                Pulse::Low => {low_count += 1}
            }
        }
    }
    

    Ok((high_count * low_count).to_string())
}

pub(super) fn build_circuit(
    input: &str,
) -> Result<HashMap<String, Gate>> {
    let mut circuit = HashMap::<String, Gate>::new();
    let parse_targets = |t_str: &str| t_str.split(", ").map(|s| s.to_string()).collect();
    let gate_ids: Vec<String> = itertools::process_results(
        input.lines().map(|line| {
            let cursor = &mut line.clone();
            let gate_type = alt(["&", "%", "broadcaster"])
                .parse_next(cursor)
                .map_err(|e: ErrMode<ContextError>| anyhow!(e))?;
            let (id, gate) = match gate_type {
                "&" => {
                    let (id, _, targets) = (
                        take_while(0.., 'a'..='z'),
                        " -> ",
                        take_while(0.., |_| true),
                    )
                        .parse_next(cursor)
                        .map_err(|e: ErrMode<ContextError>| anyhow!(e))?;
                    (
                        id,
                        Gate::C(Conjunction {
                            id: id.to_string(),
                            memory: vec![],
                            targets: parse_targets(targets),
                        }),
                    )
                }
                "%" => {
                    let (id, _, targets) = (
                        take_while(0.., 'a'..='z'),
                        " -> ",
                        take_while(0.., |_| true),
                    )
                        .parse_next(cursor)
                        .map_err(|e: ErrMode<ContextError>| anyhow!(e))?;
                    (
                        id,
                        Gate::F(FlipFlop {
                            id: id.to_string(),
                            on: false,
                            targets: parse_targets(targets),
                        }),
                    )
                }
                "broadcaster" => {
                    let (_, targets) = (" -> ", take_while(0.., |_| true))
                        .parse_next(cursor)
                        .map_err(|e: ErrMode<ContextError>| anyhow!(e))?;
                    ("broadcaster", Gate::Broadcaster(parse_targets(targets)))
                }
                _ => {
                    return Err(anyhow!("fucking winnow errors I'm gonna die"));
                }
            };
            circuit.insert(id.to_string(), gate);
            Ok(id.to_string())
        }),
        |it| it.collect(),
    )?;

    let mut new_circuit = circuit.clone();

    gate_ids.iter().for_each(|g_id| {
        let gate_targets = circuit.get(g_id).unwrap().targets().to_owned();
        for t in gate_targets {
            if let Some(act) = new_circuit.get_mut(&t) {
                match act {
                    Gate::C(conj) => conj.memory.push((g_id.clone(), Pulse::Low)),
                    _ => {}
                }
            }
        }
    });

    Ok(new_circuit)
}
