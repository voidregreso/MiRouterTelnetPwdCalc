#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use eframe::egui::{self, CentralPanel, TextEdit, Label, Layout, Align};
use eframe::{self, App, Frame, NativeOptions};
use md5;

fn hex_md5(s: &str) -> String {
    let digest = md5::compute(s.as_bytes());
    format!("{:x}", digest)
}

fn calc(sn: &str) -> String {
    let others_salt = "d44fb0960aa0-a5e6-4a30-250f-6d2df50a"
        .split('-')
        .rev()
        .collect::<Vec<_>>()
        .join("-");
    let r1d_salt = "A2E371B0-B34B-48A5-8C40-A7133F3B5D88";

    let salt = if sn.contains('/') { &others_salt } else { r1d_salt };
    hex_md5(&(sn.to_string() + salt))[..8].to_string()
}

struct MyCalcApp {
    input_str: String,
    result: String,
}

impl Default for MyCalcApp {
    fn default() -> Self {
        Self {
            input_str: String::new(),
            result: String::new(),
        }
    }
}

impl App for MyCalcApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size_before_wrap();

            // Calculate the total height of our controls
            let total_controls_height = 50.0; // Change this based on your controls' heights

            // Padding for vertical centering
            let padding = (available_size.y - total_controls_height) / 2.0;
            ui.add_space(padding.max(0.0));  // Add space at the top

            // Now add your controls
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add(TextEdit::singleline(&mut self.input_str).desired_width(200.0));
                if ui.button("Calculate").clicked() {
                    self.result = calc(&self.input_str);
                }
                ui.add(Label::new(&self.result).wrap(true));
            });

            ui.add_space(padding.max(0.0));  // Add space at the bottom
        });
    }
}

fn main() {
    let options = NativeOptions {
        // Adjust window size
        min_window_size: Some(egui::vec2(200.0, 100.0)),
        max_window_size: Some(egui::vec2(300.0, 150.0)),
        centered: true,
        ..NativeOptions::default()
    };
    eframe::run_native(
        "MiRouter Telnet Pwd Calc",
        options,
        Box::new(|_cc| Box::new(MyCalcApp::default())),
    );
}