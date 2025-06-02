#[derive(Debug)]
pub struct Innings {
    over_history: Vec<String>,
}
impl Innings {
    pub fn new() -> Self {
        Innings {
            over_history: Vec::new(),
        }
    }
    pub fn push_over_history(&mut self, over: String) {
        self.over_history.push(over);
    }
}
