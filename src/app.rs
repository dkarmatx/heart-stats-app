use egui::TextBuffer;

use super::dialog::Window;
use super::menu::{Menu, MenuOption};
use super::panels::{HomePanel, InputPanel};

pub struct Application {
    // Modal window
    dialog: Window,
    // If true, exit
    need_exit: bool,
    // Top panel
    menu: Menu,
    // Central panels
    home_panel: HomePanel,
    input_panel: InputPanel,
    plot_panel: HomePanel,
}

impl Application {
    pub fn new() -> Self {
        Self {
            dialog: Window::default(),
            need_exit: false,
            menu: Menu::default(),
            home_panel: HomePanel::default(),
            input_panel: InputPanel::default(),
            plot_panel: HomePanel::default(),
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.dialog.ui(ctx);
        if self.need_exit && !self.dialog.is_opened() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        let selected_menu_option = egui::TopBottomPanel::top("top_panel")
            .show(ctx, |ui| {
                ui.set_enabled(!self.dialog.is_opened());

                self.menu.ui(ui).inner
            })
            .inner;

        let panel_error_opt = egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.set_enabled(!self.dialog.is_opened());

                match selected_menu_option {
                    MenuOption::HomePanel => self.home_panel.ui(ui).err(),
                    MenuOption::PlotPanel => self.plot_panel.ui(ui).err(),
                    MenuOption::InputPanel => self.input_panel.ui(ui).err(),
                }
            })
            .inner;

        if let Some(error) = panel_error_opt {
            let (title_text, button_text) = if error.is_fatal() {
                self.need_exit = true;
                (t!("app.dialog.fatal_title"), t!("app.dialog.fatal_button"))
            } else {
                (t!("app.dialog.title"), t!("app.dialog.button"))
            };

            self.dialog.open(
                title_text.as_str(),
                button_text.as_str(),
                error.desc().as_str(),
            );
        }
    }
}
