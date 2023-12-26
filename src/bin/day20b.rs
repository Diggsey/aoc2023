use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use num::integer::lcm;

const INPUT: &str = include_str!("../../inputs/day20.txt");

enum ModuleState {
    FlipFlop(bool),
    Conjunction(HashSet<&'static str>),
    Broadcast,
}

struct Module {
    input_count: usize,
    outputs: Vec<&'static str>,
    state: ModuleState,
}

impl Module {
    fn update(&mut self, pulse: &Pulse) -> Option<bool> {
        match &mut self.state {
            ModuleState::FlipFlop(state) => {
                if !pulse.value {
                    *state = !*state;
                    Some(*state)
                } else {
                    None
                }
            }
            ModuleState::Conjunction(state) => {
                if pulse.value {
                    state.insert(pulse.from);
                } else {
                    state.remove(pulse.from);
                }
                Some(state.len() != self.input_count)
            }
            ModuleState::Broadcast => Some(pulse.value),
        }
    }
}

struct Pulse {
    from: &'static str,
    to: &'static str,
    value: bool,
}

fn main() {
    let mut modules = HashMap::new();
    let mut all_outputs = Vec::new();

    for line in INPUT.lines() {
        let (mut name, outputs) = line.split_once(" -> ").unwrap();
        let outputs = outputs.split(", ").collect_vec();
        let state = if let Some(suffix) = name.strip_prefix("%") {
            name = suffix;
            ModuleState::FlipFlop(false)
        } else if let Some(suffix) = name.strip_prefix("&") {
            name = suffix;
            ModuleState::Conjunction(HashSet::new())
        } else {
            ModuleState::Broadcast
        };
        all_outputs.extend(outputs.iter().copied());
        modules.insert(
            name,
            Module {
                input_count: 0,
                outputs,
                state,
            },
        );
    }

    for output in all_outputs {
        if let Some(module) = modules.get_mut(output) {
            module.input_count += 1;
        }
    }

    let mut queue = VecDeque::new();
    let mut acc: u64 = 1;
    for presses in 1..5000 {
        queue.push_back(Pulse {
            from: "button",
            to: "broadcaster",
            value: false,
        });
        while let Some(pulse) = queue.pop_front() {
            if ["st", "tn", "hh", "dt"].contains(&pulse.from) && pulse.value {
                acc = lcm(acc, presses);
            }
            if let Some(module) = modules.get_mut(pulse.to) {
                if let Some(value) = module.update(&pulse) {
                    for &output in &module.outputs {
                        queue.push_back(Pulse {
                            from: pulse.to,
                            to: output,
                            value,
                        });
                    }
                }
            }
        }
    }

    println!("{acc}");
}
