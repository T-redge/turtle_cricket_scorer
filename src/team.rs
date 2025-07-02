pub struct Team {
    name: String,
}

impl Team {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
    pub fn return_team_name(&self) -> &str {
        &self.name
    }
}