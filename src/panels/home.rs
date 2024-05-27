#[derive(Default)]
pub struct HomePanel {}

impl HomePanel {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> super::Result<()> {
        let content =
            egui::RichText::new(t!("app.home_panel.content"))
                .color(egui::Color32::GRAY)
                .size(16.0);

        ui.centered_and_justified(|ui| {
            ui.label(egui::RichText::new(content.text()));
        });

        Ok(())
    }
}
