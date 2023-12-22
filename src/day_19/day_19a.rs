use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Context, Result, Ok};
use winnow::{
    ascii::dec_int,
    error::{ContextError, ErrMode},
    token::{one_of, take_till},
    Parser, stream::AsChar
};

use crate::utils::DOUBLE_LINE_ENDING;

#[derive(PartialEq, Clone, Copy)]
pub(super) enum Valuation {
    X,
    M,
    A,
    S,
    Empty,
}

#[derive(PartialEq, Clone, Copy)]
pub(super) enum MyOrd {
    G,
    L,
    Empty,
}

struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
    pub fn sum(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let (_, x, _, m, _, a, _, s, _) = (
            "{x=",
            dec_int::<&str, i32, ContextError>,
            ",m=",
            dec_int::<&str, i32, ContextError>,
            ",a=",
            dec_int::<&str, i32, ContextError>,
            ",s=",
            dec_int::<&str, i32, ContextError>,
            "}",
        )
            .parse_next(&mut s)
            .map_err(|winnow_err| anyhow!(winnow_err))?;
        return Ok(Part { x, m, a, s });
    }
}

pub(super) struct Rule {
    pub id: String,
    pub rule_list: Vec<(Valuation, MyOrd, i32, String)>,
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let (rule_id, _, rule, _) = (
            take_till(1.., |c| c == '{'),
            "{",
            take_till(1.., |c| c == '}'),
            "}",
        )
            .parse_next(&mut s)
            .map_err(|err: ErrMode<ContextError>| anyhow!(err))?;

        let rule_list = itertools::process_results(
            rule.split(",").map(|mut rule_str| {
                if rule_str.chars().all(|c| c.is_alpha()) {
                    return Ok((Valuation::Empty, MyOrd::Empty, 0, String::from(rule_str)))
                }
                let (prop, ord, numeric_val, _, target) = (
                    one_of(['x', 'm', 'a', 's']),
                    one_of(['<', '>']),
                    dec_int::<&str, i32, ContextError>,
                    ":",
                    take_till(1.., |_| false),
                )
                    .parse_next(&mut rule_str)
                    .map_err(|err| anyhow!(err))?;
                let rule_prop = match prop {
                    'x' => Valuation::X,
                    'm' => Valuation::M,
                    'a' => Valuation::A,
                    's' => Valuation::S,
                    _ => Valuation::Empty,
                };
                let rule_ord = match ord {
                    '<' => MyOrd::L,
                    '>' => MyOrd::G,
                    _ if rule_prop == Valuation::Empty => MyOrd::Empty,
                    _ => panic!("incorrect parsing of rule"),
                };
                Ok((
                    rule_prop,
                    rule_ord,
                    numeric_val,
                    String::from(target),
                ))
            }),
            |it| it.collect(),
        )?;

        Ok(Rule {
            id: String::from(rule_id),
            rule_list: rule_list,
        })
    }
}

fn apply_rule(part: &Part, rule_id: &String, auto: &HashMap<String, Rule>) -> Result<String> {
    let rule = auto
        .get(rule_id)
        .context(anyhow!("searched for rule, didn't find it..."))?;
    for (prop, ord, numeric, target) in &rule.rule_list {
        let eval = match prop {
            Valuation::Empty => {
                return Ok(target.clone());
            }
            Valuation::X => part.x,
            Valuation::M => part.m,
            Valuation::A => part.a,
            Valuation::S => part.s,
        };
        match ord {
            MyOrd::G if eval > *numeric => {
                return Ok(target.clone());
            }
            MyOrd::L if eval < *numeric => {
                return Ok(target.clone());
            }
            MyOrd::Empty => {
                return Err(anyhow!(
                    "if this is a default action, prop shoulf have been an empty valuation"
                ))
            }
            _ => {}
        }
    }

    Err(anyhow!("did not match any target"))
}

pub fn solve(input: &String) -> Result<String> {
    let (rules_str, parts_str) = input
        .split_once(DOUBLE_LINE_ENDING)
        .context(anyhow!("no split between parts and rules"))?;

    let rules: Vec<Rule> = rules_str.lines().map(Rule::from_str).collect::<Result<Vec<Rule>>>()?;
    let auto: HashMap<String, Rule> = rules
        .into_iter()
        .map(|rule: Rule| (rule.id.clone(), rule))
        .collect();

    let parts: Vec<Part> = parts_str.lines().map(Part::from_str).collect::<Result<Vec<Part>>>()?;

    let sum: i32 = itertools::process_results(
        parts.into_iter().map(|part| {
            let mut state = String::from("in");
            while !["A", "R"].contains(&state.as_str()) {
                state = apply_rule(&part, &state, &auto)?;
            }
            match state.as_str() {
                "A" => Ok(part.sum()),
                "R" => Ok(0),
                _ => unreachable!()
            }
        }),
        |it| it.sum(),
    )?;

    Ok(sum.to_string())
}
