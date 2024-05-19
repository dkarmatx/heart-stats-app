#[derive(Default)]
pub struct InputPanel {
}

impl InputPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, _lang: &str) -> super::Result<()> {
        if ui.button("Fatal error").clicked() {
            return Err(super::PanelError::FatalError("Fatal text is here".to_owned()));
        }

        if ui.button("Error").clicked() {
            return Err(super::PanelError::Error("Error text is here".to_owned()));
        }

        Ok(())
    }
}
