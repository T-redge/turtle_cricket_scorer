#[derive(Debug, Clone)]
pub struct Extras {
    wide: u8,
    noball: u8,
    byes: u8,
    legbyes: u8,
}
impl Extras {
    pub fn new() -> Self {
        Extras {
            wide: 0,
            noball: 0,
            byes: 0,
            legbyes: 0,
        }
    }
    pub fn add_wide(&mut self, amount: u8) {
        self.wide += amount;
    }
    pub fn add_noball(&mut self) {
        self.noball += 1;
    }
    pub fn add_byes(&mut self, amount: u8) {
        self.byes += amount;
    }
    pub fn add_legbyes(&mut self, amount: u8) {
        self.legbyes += amount;
    }
    pub fn print_extras(&self) -> String {
        let mut ext = String::from("\t\t   Extras ( Wd: ");
        ext.push_str(&self.wide.to_string());
        ext.push_str(" Nb: ");
        ext.push_str(&self.noball.to_string());
        ext.push_str(" B: ");
        ext.push_str(&self.byes.to_string());
        ext.push_str(" Lb: ");
        ext.push_str(&self.legbyes.to_string());
        ext.push_str(" )");
        ext
    }
}
