#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Local, NaiveDate};
use eframe::egui;
use egui_extras::{DatePickerButton, DateTimePickerButton};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    naive_date: NaiveDate,
    date_time: DateTime<Local>,
}

impl Default for MyApp {
    fn default() -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        Self {
            naive_date: NaiveDate::from_ymd_opt(2023, 11, 17).unwrap(),
            date_time: DateTime::from_timestamp(
                now.as_secs().try_into().unwrap(),
                now.subsec_nanos(),
            )
            .unwrap()
            .into(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(DatePickerButton::new(&mut self.naive_date));
            ui.add(DateTimePickerButton::new(&mut self.date_time));
        });
    }
}
