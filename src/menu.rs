#[derive(Default, Clone, Copy)]
pub enum MenuOption {
    #[default]
    HomePanel,
    InputPanel,
    PlotPanel,
}

impl MenuOption {
    const SELECTABLE: [MenuOption; 2] = [MenuOption::InputPanel, MenuOption::PlotPanel];

    pub fn label(&self) -> String {
        match self {
            Self::InputPanel => t!("app.menu.input_panel_button_label"),
            Self::PlotPanel => t!("app.menu.plot_panel_button_label"),
            Self::HomePanel => t!("app.menu.home_panel_button_label"),
        }
        .into()
    }
}

#[derive(Default)]
pub struct Menu {
    selected: MenuOption,
}

impl Menu {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> egui::InnerResponse<MenuOption> {
        egui::menu::bar(ui, |ui| {
            for opt in MenuOption::SELECTABLE {
                if ui.button(opt.label()).clicked() {
                    self.selected = opt;
                }
            }
            self.selected
        })
    }
}
