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
        match self.input.clone().split_once(',') {
            Some((current_pair, rest)) => {
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
            None => None,
        }
    }
}

pub struct InvalidIdIterator {
    min: u64,
    max: u64,
    last: u64,
}

impl InvalidIdIterator {
    pub fn new(min: u64, max: u64) -> InvalidIdIterator {
        InvalidIdIterator {
            min: min,
            max: max,
            last: 0,
        }
    }
}

impl Iterator for InvalidIdIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
