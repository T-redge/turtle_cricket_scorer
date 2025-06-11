pub struct Team {
    name: &'static str,
}
impl Team {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
        }
    }
    pub fn return_team_name(&self) -> String {
        self.name.to_string()
    }
}