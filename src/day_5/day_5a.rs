use std::{ops::Range, str::FromStr};

use anyhow::{anyhow, Context, Result, Error};

use crate::utils::DOUBLE_LINE_ENDING;

pub(super) struct ConversionRange {
    destination: i64,
    pub(super) range: Range<i64>,
}

impl ConversionRange {
    pub const EMPTY: &ConversionRange = &ConversionRange {
        destination: 0,
        range: 0..0
    };

    #[inline]
    pub fn convert(&self, value: i64) -> i64 {
        if self.range.contains(&value) {
            self.destination + (value - self.range.start as i64)
        } else {
            value
        }
    }

    #[inline]
    pub fn convert_edge(&self, value: i64) -> i64 {
        if self.range.contains(&value) || self.range.end == value {
            self.destination + (value - self.range.start as i64)
        } else {
            value
        }
    }
}

impl FromStr for ConversionRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut values = s.split_whitespace();
        let mut parse_next = || {
            let val = values.next().context("nothing to parse")?;
            val.parse().with_context(|| "could not parse")
        };
        let (dest, source, len) = (parse_next()?, parse_next()?, parse_next()?);
        match values.next() {
            Some(_) => Err(anyhow!("should not have leftover parse values")),
            None => Ok(ConversionRange { destination: dest, range: source..source+len }),
        }
    }
}

pub(super) struct Conversion {
    ranges: Vec<ConversionRange>,
}

impl Conversion {
    pub(super) fn convert(&self, value: i64) -> i64 {
        let target_converter = match self
            .ranges
            .binary_search_by_key(&value, |c_range| c_range.range.start)
        {
            Ok(index) => &self.ranges[index],
            Err(0) => ConversionRange::EMPTY,
            Err(upper_bound) => &self.ranges[upper_bound - 1],
        };
        target_converter.convert(value)
    }

    pub(crate) fn get_ranges(&self) -> &Vec<ConversionRange> {
        &self.ranges
    }
}

impl FromStr for Conversion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next();
        let mut ranges = lines
            .map(|line| ConversionRange::from_str(line))
            .collect::<Result<Vec<ConversionRange>>>()?;
        ranges.sort_by_key(|range| range.range.start);

        if (1..ranges.len()).any(|i| ranges[i - 1].range.contains(&ranges[i].range.start)) {
            return Err(anyhow!("overlapping ranges"));
        };

        Ok(Conversion { ranges: ranges })
    }
}

fn parse_seeds(seeds: &str) -> Result<Vec<i64>> {
    let (_prefix, seeds) = seeds
        .split_once(':')
        .with_context(|| "no semicolon at seeds")?;
    let seeds = seeds
        .split_whitespace()
        .map(|seed_val| seed_val.parse().with_context(|| "bad seed parse"))
        .collect::<Result<Vec<i64>>>()?;
    Ok(seeds)
}

pub fn solve(input: &String) -> Result<String> {
    let (seeds, conversions) = input.split_once(DOUBLE_LINE_ENDING).context("could not find newline")?;
    let mut seeds = parse_seeds(seeds)?;
    conversions.split(DOUBLE_LINE_ENDING).try_for_each(|conversion| {
        let conversion = Conversion::from_str(conversion)?;
        seeds.iter_mut().for_each(|cost| *cost = conversion.convert(*cost));
        Ok::<(), Error>(())
    })?;
    let min_cost = seeds.iter().min().with_context(|| "empty seed collection")?;
    return Ok(min_cost.to_string());
}
