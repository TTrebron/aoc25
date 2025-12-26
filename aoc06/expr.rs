#[derive(Clone, Debug)]
pub struct Expression {
    pub nums: Vec<u64>,
    pub operation: Option<char>,
}

impl Expression {
    pub fn new() -> Expression {
        Expression {
            nums: vec![],
            operation: None,
        }
    }

    pub fn calculate(&self) -> u64 {
        match self.operation {
            Some('+') => self.nums.iter().sum::<u64>(),
            Some('*') => self.nums.iter().product::<u64>(),
            Some(_) | None => 0,
        }
    }
}
