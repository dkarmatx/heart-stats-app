mod app;
mod menu;
mod panels;
mod dialog;
mod util;
mod date_picker;


#[macro_use]
extern crate rust_i18n;
rust_i18n::i18n!("locales");

fn main() -> eframe::Result<()> {
    env_logger::init();
    rust_i18n::set_locale("ru");

    let application = app::Application::new();
    let native_options = eframe::NativeOptions{
        follow_system_theme: true,
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([640.0, 480.0])
            .with_active(true)
            .with_icon(
                eframe::icon_data::from_png_bytes(
                    include_bytes!("../assets/icon.png")
                ).expect("failed to load an icon")
            ),
        ..Default::default()
    };

    eframe::run_native(
        t!("app.name").to_string().as_str(),
        native_options,
        Box::new(|_cc| {
            Box::new(application)
        })
    )
}
