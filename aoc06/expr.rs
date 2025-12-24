#[derive(Clone)]
pub struct Expression {
    pub nums: Vec<i64>,
    pub operation: Option<char>,
}

impl Expression {
    pub fn new() -> Expression {
        Expression {
            nums: vec![],
            operation: None,
        }
    }
}
