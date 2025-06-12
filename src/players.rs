pub struct Player {
    name: String,
}
impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }
    pub fn return_name(&self) -> &str {
        &self.name
    }
}