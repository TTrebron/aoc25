pub struct Dial {
    current_state: u8,
}

impl Dial {
    pub fn new(state: u8) -> Dial {
        Dial {
            current_state: state,
        }
    }

    pub fn rotate(&mut self, instruction: i8) -> u8 {
        if instruction >= 0 {
            self.current_state = (self.current_state + instruction as u8) % 100;
        } else {
            self.current_state = (self.current_state + (100 + instruction) as u8) % 100;
        }

        self.current_state
    }

    pub fn parse_line(line: &str) -> Option<i8> {
        let mut instruction = line.trim();
        // return None if string too small
        if instruction.len() < 2 {
            return None;
        }

        // remove and parse first letter (L or R)
        let direction: i8;
        match instruction.chars().next() {
            Some('L') => direction = -1,
            Some('R') => direction = 1,
            Some(_) => return None,
            None => return None,
        };

        // parse rest of string as u32
        instruction = &instruction[1..];
        let rotation: i8;
        match instruction.parse::<u32>() {
            Ok(value) => rotation = (value % 100) as i8,
            Err(_) => return None,
        }

        // return parsed number mod 100 as i8
        return Some(rotation * direction);
    }
}
