pub struct Player {
    name: String,
    runs: u8,
    balls: u8,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            runs: 0,
            balls: 0,
        }
    }
    pub fn return_profile(&self) -> String {
        let mut profile = String::new();
        profile.push_str(&self.return_name());
        profile.push_str("\t");
        profile.push_str(&self.return_runs().to_string());
        profile.push_str("(");
        profile.push_str(&self.return_balls().to_string());
        profile.push_str(")");
        profile
    }
    pub fn add_runs(&mut self) {
        self.runs += 1;
    }


    fn return_name(&self) -> String {
        self.name.clone()
    }
    fn return_runs(&self) -> u8 {
        self.runs
    }
    fn return_balls(&self) -> u8 {
        self.balls
    }
}