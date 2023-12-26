use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day19.txt");

enum Cond {
    Gt(&'static str, i32),
    Lt(&'static str, i32),
    True,
}

impl Cond {
    fn applies_to(&self, fields: &HashMap<&str, i32>) -> bool {
        match self {
            Cond::Gt(name, value) => fields[name] > *value,
            Cond::Lt(name, value) => fields[name] < *value,
            Cond::True => true,
        }
    }
}

struct Rule {
    cond: Cond,
    result: &'static str,
}

fn main() {
    let (workflows, parts) = INPUT.split_once("\n\n").unwrap();
    let workflows: HashMap<_, _> = workflows
        .lines()
        .map(|line| {
            let (name, rules) = line.split_once('{').unwrap();
            let rules = rules
                .strip_suffix('}')
                .unwrap()
                .split(',')
                .map(|rule| {
                    if let Some((cond, result)) = rule.split_once(':') {
                        if let Some((name, value)) = cond.split_once('>') {
                            Rule {
                                cond: Cond::Gt(name, value.parse().unwrap()),
                                result,
                            }
                        } else if let Some((name, value)) = cond.split_once('<') {
                            Rule {
                                cond: Cond::Lt(name, value.parse().unwrap()),
                                result,
                            }
                        } else {
                            unreachable!()
                        }
                    } else {
                        Rule {
                            cond: Cond::True,
                            result: rule,
                        }
                    }
                })
                .collect_vec();
            (name, rules)
        })
        .collect();

    let sum: i32 = parts
        .lines()
        .map(|line| {
            line[1..line.len() - 1]
                .split(',')
                .map(|field| {
                    let (name, value) = field.split_once('=').unwrap();
                    (name, value.parse::<i32>().unwrap())
                })
                .collect::<HashMap<_, _>>()
        })
        .filter(|fields| {
            let mut workflow_name = "in";
            while workflow_name != "A" && workflow_name != "R" {
                workflow_name = workflows[workflow_name]
                    .iter()
                    .find(|rule| rule.cond.applies_to(fields))
                    .unwrap()
                    .result;
            }
            workflow_name == "A"
        })
        .map(|fields| fields.values().copied().sum::<i32>())
        .sum();
    println!("{sum}");
}
