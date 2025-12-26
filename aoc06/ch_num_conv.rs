pub struct CharNumConverter {
    pub num: Option<u64>,
}

impl CharNumConverter {
    pub fn new() -> CharNumConverter {
        CharNumConverter { num: None }
    }

    pub fn push_digit(&mut self, digit: char) {
        if !digit.is_digit(10) {
            return;
        }

        if self.num.is_none() {
            self.num = Some(0);
        }

        self.num = self.num.map(|num| num * 10);
        self.num = self
            .num
            .map(|num| num + (u64::from(digit) - u64::from('0')));
        println!("digit: {} converted: {}", digit, self.num.unwrap());
    }
}
