use std::{ops::Range, cmp::max, str::FromStr};

use crate::utils::DOUBLE_LINE_ENDING;

use super::day_5a::{ConversionRange, Conversion};

use anyhow::{Result, Context, Error};
use itertools::Itertools;

fn convert_range(converter: &ConversionRange, transforming: Range<i64>) -> Result<(Vec<Range<i64>>, Option<Range<i64>>)> {
    let converted;
    let residual;

    let converter_range = &converter.range;

    if transforming.contains(&converter_range.start) && transforming.contains(&(converter_range.end - 1)) {
        converted = vec![
            (transforming.start..converter_range.start),
            (converter.convert_edge(converter_range.start)..converter.convert_edge(converter_range.end))
        ];
        residual = Some(converter_range.end..transforming.end);
        return Ok((converted, residual));
    }

    if converter_range.start >= transforming.end {
        converted = vec![transforming];
        residual = None;
    } else if converter_range.start > transforming.start && transforming.end > converter_range.start {
        converted = vec![
            (transforming.start..converter_range.start),
            (converter.convert_edge(converter_range.start)..converter.convert_edge(transforming.end))
        ];
        residual = None;
    } else if converter_range.contains(&transforming.start) && converter_range.contains(&(transforming.end - 1)) {
        converted = vec![converter.convert_edge(transforming.start)..converter.convert_edge(transforming.end)];
        residual = None;
        return Ok((converted, residual));
    } else if transforming.start < converter_range.end && converter_range.end < transforming.end {
        converted = vec![converter.convert_edge(transforming.start)..converter.convert_edge(converter_range.end)];
        residual = Some(converter_range.end..transforming.end);
    } else if transforming.start >= converter_range.end {
        converted = vec![];
        residual = Some(transforming);
    } else {
        panic!("could not relate range to transformation")
    }
    return Ok((converted, residual));
}

pub fn solve(input: &String) -> Result<String> {
    let (seed_ranges, conversions) = input.split_once(DOUBLE_LINE_ENDING).context("could not find newline")?;
    let mut seed_ranges = parse_seed_ranges(seed_ranges)?;
    seed_ranges = sorted_and_united_ranges(seed_ranges);
    conversions.split(DOUBLE_LINE_ENDING).try_for_each(|conversion_map| {
        let conversion = Conversion::from_str(conversion_map)?;
        let transformed = convert_ranges(&conversion, &seed_ranges)?;
        seed_ranges = sorted_and_united_ranges(transformed);
        return Ok::<(), Error>(())
    })?;
    let result = seed_ranges.iter().min_by_key(|range| range.start).context("no seed ranges left after proccessing")?;
    return Ok(result.start.to_string());
}

fn parse_seed_ranges(ranges: &str) -> Result<Vec<Range<i64>>> {
    let (_prefix, ranges) = ranges
        .split_once(':')
        .with_context(|| "no semicolon at seeds")?;
    let ranges = ranges
        .split_whitespace()
        .map(|seed_val| seed_val.parse().with_context(|| "bad seed parse"))
        .collect::<Result<Vec<i64>>>()?
        .iter()
        .tuple_windows()
        .step_by(2)
        .map(|(start, step)| *start..*start+*step)
        .collect::<Vec<Range<i64>>>();
    Ok(ranges)
}

fn sorted_and_united_ranges(mut ranges: Vec<Range<i64>>) -> Vec<Range<i64>> {
    ranges.sort_by_key(|range| range.start);
    let first;
    if ranges.len() > 0 {
        first = ranges[0].clone();
    } else {
        panic!("fuckfuckfuck");
    }
    let (mut ranges, last) = ranges[1..].into_iter().fold((Vec::<Range<i64>>::new(), first), |(mut united, mut computed_range), curr_range| {
        if curr_range.start > computed_range.end {
            united.push(computed_range);
            computed_range = curr_range.clone();
            return (united, computed_range)
        };
        computed_range.end = max(curr_range.end, computed_range.end);
        return (united, computed_range)
    });
    ranges.push(last);
    return ranges;
}

fn convert_ranges(converter: &Conversion, transforming: &Vec<Range<i64>>) -> Result<Vec<Range<i64>>> {
    let converters = converter.get_ranges();
    let mut converters_index = 0;
    let transformed = itertools::process_results(transforming.iter().map(|transforming_now| {
        let mut residual = Some(transforming_now.to_owned());
        let mut collector = Vec::<Range::<i64>>::new();
        while let Some(transforming_now) = residual.clone() {
            if converters_index < converters.len() && transforming_now.start >= converters[converters_index].range.end {
                converters_index += 1;
            }
            if converters_index < converters.len() {
                let (mut newly_tranformed, current_residual) = convert_range(&converters[converters_index], transforming_now.clone())?;
                residual = current_residual;
                collector.append(&mut newly_tranformed);
            } else {
                collector.push(transforming_now.clone());
                residual = None;
            }
        }
        return Ok(collector)
    }), |it| it.flat_map(|collection| collection).collect());
    return transformed;
}