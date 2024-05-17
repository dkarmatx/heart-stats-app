#[derive(Default, Clone, Copy)]
pub enum MenuOption {
    #[default]
    HomePanel,
    InputPanel,
    PlotPanel,
}

impl MenuOption {
    const SELECTABLE: [MenuOption; 2] = [
        MenuOption::InputPanel,
        MenuOption::PlotPanel,
    ];

    pub fn label(&self, lang: &str) -> String {
        match self {
            Self::InputPanel => locales::t!("app.menu.input_panel_button_label", lang),
            Self::PlotPanel  => locales::t!("app.menu.plot_panel_button_label", lang),
            Self::HomePanel  => locales::t!("app.menu.home_panel_button_label", lang),
        }
    }
}


#[derive(Default)]
pub struct Menu {
    selected: MenuOption,
}

impl Menu {
    pub fn ui(&mut self, ui: &mut egui::Ui, lang: &str) -> egui::InnerResponse<MenuOption> {
        egui::menu::bar(ui, |ui| {
            for opt in MenuOption::SELECTABLE {
                if ui.button(opt.label(lang)).clicked() {
                    self.selected = opt;
                }
            }
            self.selected
        })
    }
}
