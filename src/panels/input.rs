use crate::date_picker::DatePicker;

pub struct InputPanel {
    date: time::Date,
}

impl Default for InputPanel {
    fn default() -> Self {
        Self {
            date: time::OffsetDateTime::now_utc().date(),
        }
    }
}

impl InputPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> super::Result<()> {
        if ui.button("Fatal error").clicked() {
            return Err(super::PanelError::FatalError(
                "Fatal text is here".to_owned(),
            ));
        }

        if ui.button("Error").clicked() {
            return Err(super::PanelError::Error("Error text is here".to_owned()));
        }

        ui.add(DatePicker::new("date_picker_1", ui, &mut self.date));

        Ok(())
    }
}
