use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day19.txt");

#[derive(Copy, Clone)]
enum Cond {
    Gt(&'static str, i64),
    Lt(&'static str, i64),
    True,
}

impl Cond {
    fn split(
        self,
        mut fields: HashMap<&'static str, Range<i64>>,
    ) -> (
        HashMap<&'static str, Range<i64>>,
        HashMap<&'static str, Range<i64>>,
    ) {
        let mut fields_fail = fields.clone();
        match self {
            Cond::Gt(name, value) => {
                fields_fail.get_mut(name).unwrap().end = value + 1;
                fields.get_mut(name).unwrap().start = value + 1;
            }
            Cond::Lt(name, value) => {
                fields_fail.get_mut(name).unwrap().start = value;
                fields.get_mut(name).unwrap().end = value;
            }
            Cond::True => {
                for v in fields_fail.values_mut() {
                    v.end = v.start;
                }
            }
        }
        (fields_fail, fields)
    }
}

struct Rule {
    cond: Cond,
    result: &'static str,
}

fn main() {
    let (workflows, _parts) = INPUT.split_once("\n\n").unwrap();
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

    let mut queue = VecDeque::new();
    let mut map = HashMap::new();
    map.insert("x", 1..4001i64);
    map.insert("m", 1..4001i64);
    map.insert("a", 1..4001i64);
    map.insert("s", 1..4001i64);
    queue.push_back(("in", map));

    let mut sum = 0;
    while let Some((workflow_name, mut fields)) = queue.pop_front() {
        if workflow_name == "R" {
            continue;
        }
        if workflow_name == "A" {
            sum += fields.values().map(|v| v.end - v.start).product::<i64>();
            continue;
        }
        for rule in &workflows[workflow_name] {
            let (fields_fail, fields_success) = rule.cond.split(fields);
            fields = fields_fail;
            if fields_success.values().all(|v| !v.is_empty()) {
                queue.push_back((rule.result, fields_success));
            }
            if !fields.values().all(|v| !v.is_empty()) {
                break;
            }
        }
    }
    println!("{sum}");
}
