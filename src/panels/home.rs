#[derive(Default)]
pub struct HomePanel {}

impl HomePanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, lang: &str) -> super::Result<()> {
        let content =
            egui::RichText::new(locales::t!("app.home_panel.content", lang))
                .color(egui::Color32::GRAY)
                .size(16.0);

        ui.centered_and_justified(|ui| {
            ui.label(egui::RichText::new(content.text()));
        });

        Ok(())
    }
}
