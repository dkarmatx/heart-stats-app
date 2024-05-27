use time::{Date, Month, Weekday};

pub struct DatePicker<'a> {
    date: &'a mut Date,

    id: egui::Id,
    title_font_size: f32,
    menu_font_size: f32,
    calendar_font_size: f32,
}

impl<'a> DatePicker<'a> {
    pub fn new(id: impl std::hash::Hash, ui: &egui::Ui, date: &'a mut Date) -> Self {
        Self {
            date: date,
            title_font_size: ui.text_style_height(&egui::TextStyle::Button) * 1.11,
            calendar_font_size: ui.text_style_height(&egui::TextStyle::Button),
            menu_font_size: ui.text_style_height(&egui::TextStyle::Button),
            id: egui::Id::new(id),
        }
    }
}

fn month_to_locales_string(m: Month) -> String {
    match m {
        Month::January => t!("date_picker.month.january"),
        Month::February => t!("date_picker.month.february"),
        Month::March => t!("date_picker.month.march"),
        Month::April => t!("date_picker.month.april"),
        Month::May => t!("date_picker.month.may"),
        Month::June => t!("date_picker.month.june"),
        Month::July => t!("date_picker.month.july"),
        Month::August => t!("date_picker.month.august"),
        Month::September => t!("date_picker.month.september"),
        Month::October => t!("date_picker.month.october"),
        Month::November => t!("date_picker.month.november"),
        Month::December => t!("date_picker.month.december"),
    }
    .into()
}

fn weekday_to_locales_string(w: Weekday) -> String {
    match w {
        Weekday::Monday => t!("date_picker.weekday.monday"),
        Weekday::Tuesday => t!("date_picker.weekday.tuesday"),
        Weekday::Wednesday => t!("date_picker.weekday.wednesday"),
        Weekday::Thursday => t!("date_picker.weekday.thursday"),
        Weekday::Friday => t!("date_picker.weekday.friday"),
        Weekday::Saturday => t!("date_picker.weekday.saturday"),
        Weekday::Sunday => t!("date_picker.weekday.sunday"),
    }
    .into()
}

impl DatePicker<'_> {
    fn show_month_button(&mut self, ui: &mut egui::Ui) {
        let current_month = self.date.month();
        let button_text =
            egui::RichText::new(month_to_locales_string(current_month)).size(self.title_font_size);

        ui.menu_button(button_text, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for m in [
                    Month::January,
                    Month::February,
                    Month::March,
                    Month::April,
                    Month::May,
                    Month::June,
                    Month::July,
                    Month::August,
                    Month::September,
                    Month::October,
                    Month::November,
                    Month::December,
                ] {
                    let button_text =
                        egui::RichText::new(month_to_locales_string(m)).size(self.menu_font_size);
                    let button = egui::Button::new(button_text).selected(m == current_month);

                    if ui.add(button).clicked() {
                        *self.date = Date::from_calendar_date(self.date.year(), m, 1).unwrap();
                        ui.close_menu();
                    }
                }
            })
        });
    }

    fn show_year_button(&mut self, ui: &mut egui::Ui) {
        let current_year = self.date.year();
        let button_text = egui::RichText::new(current_year.to_string())
            .size(ui.text_style_height(&egui::TextStyle::Button) * 1.3);

        ui.menu_button(button_text, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for y in 1982..2042 {
                    let button_text = egui::RichText::new(y.to_string()).size(self.menu_font_size);
                    let button = egui::Button::new(button_text).selected(y == current_year);

                    if ui.add(button).clicked() {
                        *self.date = Date::from_calendar_date(y, self.date.month(), 1).unwrap();
                        ui.close_menu();
                    }
                }
            });
        });
    }

    fn show_calendar_title(&mut self, ui: &mut egui::Ui, calendar_title_rect: &egui::Rect) {
        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
        ui.visuals_mut().button_frame = false;
        egui::Grid::new("calendar_title")
            .num_columns(4)
            .spacing(egui::vec2(0.0, 0.0))
            .min_row_height(calendar_title_rect.size().y)
            .min_col_width(calendar_title_rect.size().x / 4.0)
            .show(ui, |ui| {
                ui.centered_and_justified(|ui| ui.label(""));
                ui.centered_and_justified(|ui| {
                    self.show_month_button(ui);
                });
                ui.centered_and_justified(|ui| {
                    self.show_year_button(ui);
                });
                ui.centered_and_justified(|ui| ui.label(""));
                ui.end_row();
            });
    }

    fn show_calendar_weekdays(&mut self, ui: &mut egui::Ui) {
        for w in [
            Weekday::Monday,
            Weekday::Tuesday,
            Weekday::Wednesday,
            Weekday::Thursday,
            Weekday::Friday,
            Weekday::Saturday,
            Weekday::Sunday,
        ] {
            ui.centered_and_justified(|ui| {
                let weekday_text =
                    egui::RichText::new(weekday_to_locales_string(w)).size(self.calendar_font_size);
                ui.label(weekday_text);
            });
        }
        ui.end_row();
    }

    fn show_calendar_day(&mut self, date: &Date, ui: &mut egui::Ui) -> egui::Response {
        ui.centered_and_justified(|ui| {
            let text = egui::RichText::new(date.day().to_string()).size(self.calendar_font_size);

            let button = egui::Button::new(text)
                .frame(false)
                .selected(self.date == date);

            ui.visuals_mut().button_frame = false;
            if ui
                .add_enabled(self.date.month() == date.month(), button)
                .clicked()
            {
                *self.date = *date;
            }
        })
        .response
    }

    fn show_calendar_grid(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("calendar")
            .min_row_height(self.calendar_font_size * 1.7)
            .show(ui, |ui| {
                self.show_calendar_weekdays(ui);

                let first_day_at_month = self.date.replace_day(1).unwrap();
                let first_dat_at_grid = first_day_at_month.saturating_sub(time::Duration::days(
                    first_day_at_month.weekday().number_days_from_monday() as i64,
                ));

                let mut day = first_dat_at_grid;
                for _w in 0..6 {
                    for _d in 0..7 {
                        self.show_calendar_day(&day, ui);
                        day = day.saturating_add(time::Duration::days(1));
                    }
                    ui.end_row();
                }
            });
    }

    fn show_month_move_button(&mut self, ui: &mut egui::Ui, direction: &str) {
        let next_date = match direction {
            ">" => Date::from_calendar_date(
                self.date
                    .year()
                    .saturating_add(if self.date.month() == Month::December {
                        1
                    } else {
                        0
                    }),
                self.date.month().next(),
                1,
            ),
            _ => Date::from_calendar_date(
                self.date
                    .year()
                    .saturating_sub(if self.date.month() == Month::January {
                        1
                    } else {
                        0
                    }),
                self.date.month().previous(),
                1,
            ),
        };
        ui.visuals_mut().button_frame = false;
        ui.centered_and_justified(|ui| {
            let text = egui::RichText::new(direction).size(self.title_font_size);
            if ui.button(text).clicked() {
                *self.date = next_date.unwrap();
            }
        });
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let format = time::format_description::parse("[day].[month].[year]").unwrap();
        let formatted_datetime = self.date.format(&format).unwrap();
        let button_response = ui.button(formatted_datetime.as_str());

        if button_response.clicked() {
            ui.memory_mut(|m| m.toggle_popup(self.id));
        }

        if ui.memory(|m| m.is_popup_open(self.id)) {
            let area = egui::Area::new(self.id)
                .order(egui::Order::Foreground)
                .fixed_pos(button_response.rect.right_top());

            let area_response = area
                .show(ui.ctx(), |ui| {
                    egui::Frame::popup(ui.style()).show(ui, |ui| {
                        let max_rect = ui.max_rect();
                        let margin_top_bottom = self.title_font_size;
                        let title_height = self.title_font_size * 2.0;
                        let month_skip_width = self.title_font_size * 1.5;

                        let calendar_body_rect = egui::Rect::from_min_max(
                            egui::pos2(
                                max_rect.min.x + month_skip_width,
                                max_rect.min.y + title_height + margin_top_bottom * 1.5,
                            ),
                            egui::pos2(max_rect.max.x + month_skip_width, max_rect.max.y),
                        );
                        let calendar_body_rect = ui
                            .allocate_ui_at_rect(calendar_body_rect, |ui| {
                                self.show_calendar_grid(ui);
                                ui.add_space(margin_top_bottom);
                                ui.min_rect()
                            })
                            .inner;

                        let calendar_title_rect = egui::Rect::from_min_size(
                            egui::pos2(max_rect.min.x, max_rect.min.y + margin_top_bottom),
                            egui::vec2(
                                calendar_body_rect.width() + (month_skip_width * 2.0),
                                title_height,
                            ),
                        );
                        ui.allocate_ui_at_rect(calendar_title_rect, |ui| {
                            self.show_calendar_title(ui, &calendar_title_rect);
                        });

                        let month_prev_rect = egui::Rect::from_min_size(
                            egui::pos2(calendar_title_rect.min.x, calendar_title_rect.max.y),
                            egui::vec2(month_skip_width, calendar_body_rect.size().y),
                        );
                        ui.allocate_ui_at_rect(month_prev_rect, |ui| {
                            self.show_month_move_button(ui, "<");
                        });

                        let month_next_rect = egui::Rect::from_min_size(
                            egui::pos2(calendar_body_rect.max.x, calendar_title_rect.max.y),
                            egui::vec2(month_skip_width, calendar_body_rect.size().y),
                        );
                        ui.allocate_ui_at_rect(month_next_rect, |ui| {
                            self.show_month_move_button(ui, ">");
                        });
                    });
                })
                .response;

            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                ui.memory_mut(|m| {
                    m.toggle_popup(self.id);
                });
            }

            if !button_response.clicked() && area_response.clicked_elsewhere() {
                ui.memory_mut(|m| {
                    m.toggle_popup(self.id);
                });
            }
        }

        button_response
    }
}
