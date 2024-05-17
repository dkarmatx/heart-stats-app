use super::menu::{Menu, MenuOption};
use super::panels::HomePanel;

pub struct Application {
    // Locale language
    lang: String,
    // Top panel
    menu: Menu,
    // Cental panels
    home_panel: HomePanel,
    input_panel: HomePanel,
    plot_panel: HomePanel,
}

impl Application {
    pub fn new(lang: &str) -> Self {
        Self {
            lang: lang.to_owned(),
            menu: Menu::default(),
            home_panel: HomePanel::default(),
            input_panel: HomePanel::default(),
            plot_panel: HomePanel::default(),
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let selected_menu_option =
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                self.menu.ui(ui, self.lang.as_str()).inner
            }).inner;

        egui::CentralPanel::default().show(ctx, |ui| {
            match selected_menu_option {
                MenuOption::HomePanel   => self.home_panel.ui(ui, self.lang.as_str()),
                MenuOption::InputPanel  => self.input_panel.ui(ui, self.lang.as_str()),
                MenuOption::PlotPanel   => self.plot_panel.ui(ui, self.lang.as_str()),
            }
        });
    }
}
