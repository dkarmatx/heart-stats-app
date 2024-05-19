
#[derive(Default)]
pub struct Window {
    title: String,
    text: String,
    button: String,
    opened: bool,
}

impl Window {
    pub fn ui(&mut self, ctx: &egui::Context) {
        if self.opened {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("dialog"),
                egui::ViewportBuilder::default()
                    .with_title(self.title.as_str())
                    .with_inner_size([320.0, 96.0])
                    .with_min_inner_size([320.0, 96.0])
                    .with_titlebar_buttons_shown(false)
                    .with_always_on_top(),
                |ctx, _class| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let text_area_size = egui::Vec2::new(ui.available_width(), ui.available_height() - 32.0);
                        let button_area_size = egui::Vec2::new(ui.available_width(), 32.0);

                        ui.allocate_ui_with_layout(
                            text_area_size,
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui|{
                                ui.style_mut().wrap = Some(true);
                                ui.label(self.text.as_str());
                            });

                        ui.allocate_ui_with_layout(
                            button_area_size,
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                if ui.add_sized([72.0, 16.0], egui::Button::new(self.button.as_str())).clicked() {
                                    self.close()
                                }
                            });
                    });
                });
        }
    }

    pub fn open(&mut self, title: &str, button: &str, text: &str) {
        self.title = title.to_owned();
        self.text = text.to_owned();
        self.button = button.to_owned();
        self.opened = true;
    }

    pub fn close(&mut self) {
        self.opened = false;
    }

    pub fn is_opened(&self) -> bool {
        self.opened
    }
}