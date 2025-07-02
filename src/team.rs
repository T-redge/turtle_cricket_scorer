pub struct Team {
    name: String,
}

impl Team {
    pub fn new() -> Self {
        Self {
            name: "".to_owned(),
        }
    }
    pub fn set_team_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn return_team_name(&self) -> String {
        self.name.clone()
    }
}
