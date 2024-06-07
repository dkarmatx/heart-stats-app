use crate::date_picker::DatePicker;

#[derive(Default, PartialEq, Clone, Copy)]
enum DayTimeRadioValue {
    #[default]
    Morning,
    Noon,
    Evening,
}

impl DayTimeRadioValue {
    fn to_label(&self) -> String {
        match self {
            Self::Morning => t!("app.measurements.day_time.morning"),
            Self::Noon => t!("app.measurements.day_time.noon"),
            Self::Evening => t!("app.measurements.day_time.evening"),
        }
        .into()
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for value in [Self::Morning, Self::Noon, Self::Evening] {
                ui.radio_value(self, value, value.to_label());
            }
        });
    }
}

pub struct InputPanel {
    systolic_input: String,
    diastolic_input: String,
    rate_input: String,
    date_input: time::Date,
    day_time_input: DayTimeRadioValue,
}

impl Default for InputPanel {
    fn default() -> Self {
        Self {
            systolic_input: String::default(),
            diastolic_input: String::default(),
            rate_input: String::default(),
            date_input: time::OffsetDateTime::now_utc().date(),
            day_time_input: DayTimeRadioValue::default(),
        }
    }
}

impl InputPanel {
    fn reset(&mut self) {
        *self = Self::default()
    }
}

impl InputPanel {
    fn add_labeled_integer_input(
        ui: &mut egui::Ui,
        label: impl std::fmt::Display,
        text: &mut String,
    ) {
        ui.horizontal(|ui| {
            ui.label(format!("{}:", label));
            ui.text_edit_singleline(text);
        });
    }

    fn draw_input_form(&mut self, ui: &mut egui::Ui) {
        ui.spacing_mut().item_spacing = ui.spacing().item_spacing * egui::vec2(1.0, 2.0);

        Self::add_labeled_integer_input(
            ui,
            t!("app.measurements.systolic_label"),
            &mut self.systolic_input,
        );
        Self::add_labeled_integer_input(
            ui,
            t!("app.measurements.diastolic_label"),
            &mut self.diastolic_input,
        );
        Self::add_labeled_integer_input(
            ui,
            t!("app.measurements.rate_label"),
            &mut self.rate_input,
        );

        ui.horizontal(|ui| {
            ui.add(DatePicker::new("date_input", ui, &mut self.date_input));
            self.day_time_input.ui(ui);
        });

        if ui.button(t!("app.input_panel.commit_button")).clicked() {
            self.reset();
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> super::Result<()> {
        let (_, rect) = ui.allocate_space(egui::vec2(64.0, ui.available_width()));
        ui.allocate_ui_at_rect(rect, |ui| {
            ui.label("vava");
        });

        ui.separator();
        ui.label("vava");

        ui.group(|ui| {
            self.draw_input_form(ui);
        });

        Ok(())
    }
}
