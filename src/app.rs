use eframe::egui;
pub struct Scoreboard {}
impl Scoreboard {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {

        }
    }
}
impl eframe::App for Scoreboard {
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {

    }
}