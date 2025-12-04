use std::ops::{Div, Range, Rem};

pub struct RangesParser {
    input: String,
}

impl RangesParser {
    pub fn new(input: &str) -> RangesParser {
        RangesParser {
            input: input.to_string(),
        }
    }

    pub fn get_input_rest(&self) -> &str {
        &self.input
    }
}

impl Iterator for RangesParser {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        self.input = self.input.trim().to_string();
        let clone = self.input.clone();
        let (current_pair, rest) = {
            let mut input_parts = clone.splitn(2, ",");
            (
                input_parts.next().unwrap(),
                input_parts.next().unwrap_or_default(),
            )
        };
        //println!("{},{}", current_pair, rest);
        self.input = rest.to_string();
        match current_pair.split_once('-') {
            Some((min_str, max_str)) => Some((
                min_str.parse::<u64>().unwrap_or_default(),
                max_str.parse::<u64>().unwrap_or_default(),
            )),
            None => None,
        }
    }
}

fn get_half_point(num: u64) -> u32 {
    // let digits_min = min == 0 ? 1 : floor(log10(min)) + 1
    // let upper_half_digits_min = ceil(digits_min/2);
    // let lower_half_digits_min = digits_min - upper_half_digits_min;
    // let lower_half_min = min % 10^lower_half_digits_min;
    // let upper_half_min = (min - lower_half_min)/10^(lower_half_digits_min)

    let digits = num.checked_ilog10().unwrap_or_default() + 1;
    let upper_half_digits = digits.div(2);

    upper_half_digits
}

fn split_at(num: u64, index: u32) -> (u32, u32, u32, u64, u64) {
    let digits = num.checked_ilog10().unwrap_or_default() + 1;
    let upper_half_digits = index;
    let lower_half_digits = digits - index;
    let upper_half = num.div(u64::pow(10, lower_half_digits));
    let lower_half = num.rem(u64::pow(10, lower_half_digits));
    (
        digits,
        upper_half_digits,
        lower_half_digits,
        upper_half,
        lower_half,
    )
}

fn split_at_get_upper(num: u64, index: u32) -> u64 {
    if index == 0 {
        return num;
    }
    let digits = num.checked_ilog10().unwrap_or_default() + 1;
    let lower_half_digits = digits - index;
    let upper_half = num.div(u64::pow(10, lower_half_digits));

    upper_half
}

#[derive(Debug)]
pub struct InvalidIdIterator {
    min: u64,
    digits_min: u32,
    upper_half_digits_min: u32,
    lower_half_digits_min: u32,
    upper_half_min: u64,
    lower_half_min: u64,
    max: u64,
    digits_max: u32,
    upper_half_digits_max: u32,
    lower_half_digits_max: u32,
    upper_half_max: u64,
    lower_half_max: u64,
    last_pattern: u64,
    digits_last_pattern: u32,
    prefix_digits_limit: u32,
}

impl InvalidIdIterator {
    pub fn new(min: u64, max: u64, prefix_digits_limit: Option<u32>) -> InvalidIdIterator {
        let (
            digits_min,
            upper_half_digits_min,
            lower_half_digits_min,
            upper_half_min,
            lower_half_min,
        ) = split_at(min, prefix_digits_limit.unwrap_or(get_half_point(min)));
        let (
            digits_max,
            upper_half_digits_max,
            lower_half_digits_max,
            upper_half_max,
            lower_half_max,
        ) = split_at(max, prefix_digits_limit.unwrap_or(get_half_point(max)));
        InvalidIdIterator {
            min: min,
            digits_min: digits_min,
            upper_half_digits_min: upper_half_digits_min,
            lower_half_digits_min: lower_half_digits_min,
            upper_half_min: upper_half_min,
            lower_half_min: lower_half_min,
            max: max,
            digits_max: digits_max,
            upper_half_digits_max: upper_half_digits_max,
            lower_half_digits_max: lower_half_digits_max,
            upper_half_max: upper_half_max,
            lower_half_max: lower_half_max,
            last_pattern: 0,
            digits_last_pattern: 0,
            prefix_digits_limit: prefix_digits_limit.unwrap_or(0),
        }
    }
}

impl Iterator for InvalidIdIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current_pattern = match self.last_pattern {
            0 => {
                if self.upper_half_digits_min * 2 < self.digits_min {
                    // next generated number would be < min
                    if self.prefix_digits_limit == 0 {
                        u64::pow(10, std::cmp::max(self.upper_half_digits_min, 0)) // if we have 5(42) as min, start at the next new digit: 10(00)
                    } else {
                        return None; // fixed prefix, no number would be larger than min
                    }
                } else if split_at_get_upper(self.lower_half_min, self.prefix_digits_limit)
                    > self.upper_half_min
                {
                    self.upper_half_min + 1
                } else {
                    self.upper_half_min
                }
            }
            val => val + 1,
        };

        let digits_current_pattern = current_pattern.checked_ilog10().unwrap_or_default() + 1;
        if self.prefix_digits_limit > 0 && digits_current_pattern > self.prefix_digits_limit {
            return None;
        }

        let num = current_pattern + current_pattern * u64::pow(10, digits_current_pattern);

        self.digits_last_pattern = digits_current_pattern;
        self.last_pattern = current_pattern;

        if num > self.max { None } else { Some(num) }
    }
}
