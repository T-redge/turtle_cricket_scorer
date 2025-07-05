pub struct Player {
    name: String,
    runs: u8,
    balls: u8,
}

impl Player {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            runs: 0,
            balls: 0,
        }
    }
    pub fn return_profile(&self) -> (String,String) {
        let mut profile = String::new();
        profile.push_str(&self.return_name());
        let mut profile_scores = String::new();
        profile_scores.push_str(&self.return_runs().to_string());
        profile_scores.push_str("(");
        profile_scores.push_str(&self.return_balls().to_string());
        profile_scores.push_str(")");
        (profile,profile_scores)
    }
    pub fn add_runs(&mut self) {
        self.runs += 1;
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
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
