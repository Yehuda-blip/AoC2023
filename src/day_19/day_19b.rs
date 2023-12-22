use anyhow::{anyhow, Context, Result};

use std::{collections::HashMap, ops::Range, str::FromStr};

use crate::utils::DOUBLE_LINE_ENDING;

use super::day_19a::{MyOrd, Rule, Valuation};

const FIELDS: usize = 4;

#[derive(Clone)]
struct ValRange {
    fields: [Range<i64>; FIELDS],
}

impl ValRange {
    pub fn is_empty(&self) -> bool {
        self.fields.iter().any(|r| r.is_empty())
    }

    pub fn combinations(&self) -> i64 {
        self.fields
            .iter()
            .fold(1, |acc, range| acc * range.clone().count()) as i64
    }

    pub fn spawn(&mut self, thresh: i64, field: Valuation, ord: MyOrd) -> Option<ValRange> {
        let mut clone = self.clone();
        match ord {
            MyOrd::G => {
                clone.fields[field as usize] = (thresh + 1)..clone.fields[field as usize].end;
                self.fields[field as usize] = self.fields[field as usize].start..(thresh + 1);
            }
            MyOrd::L => {
                clone.fields[field as usize] = clone.fields[field as usize].start..thresh;
                self.fields[field as usize] = thresh..(self.fields[field as usize].end);
            }
            MyOrd::Empty => {
                if self.is_empty() {
                    return None;
                };
                return Some(clone);
            }
        };
        if clone.is_empty() {
            return None;
        }
        Some(clone)
    }
}

fn apply_rule(mut values: ValRange, rule_id: &String, auto: &HashMap<String, Rule>) -> Result<i64> {
    if rule_id.as_str() == "A" {
        return Ok(values.combinations());
    }
    if rule_id.as_str() == "R" {
        return Ok(0);
    }

    let rule = auto
        .get(rule_id)
        .context(anyhow!("searched for rule, didn't find it..."))?;

    let mut sum = 0;
    for (field, ord, thresh, target) in &rule.rule_list {
        if let Some(spawn) = values.spawn(*thresh as i64, *field, *ord) {
            sum += apply_rule(spawn, target, auto)?;
        }
    }

    Ok(sum)
}

pub fn solve(input: &String) -> Result<String> {
    let (rules_str, _parts_str) = input
        .split_once(DOUBLE_LINE_ENDING)
        .context(anyhow!("no split between parts and rules"))?;

    let rules: Vec<Rule> = rules_str
        .lines()
        .map(Rule::from_str)
        .collect::<Result<Vec<Rule>>>()?;
    let auto: HashMap<String, Rule> = rules
        .into_iter()
        .map(|rule: Rule| (rule.id.clone(), rule))
        .collect();

    let ranges = ValRange {
        fields: [1..4001, 1..4001, 1..4001, 1..4001],
    };

    let val = apply_rule(ranges, &String::from("in"), &auto)?;

    Ok(val.to_string())
}
