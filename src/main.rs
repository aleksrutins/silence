#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use header::header;
use timeline::Timeline;

mod header;
mod timeline;

fn main() {
    let options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: None,
        min_window_size: None,
        max_window_size: None,
        resizable: true,
        transparent: false,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
    };
    let app = SilenceApp::default();
    eframe::run_native(
        "Silence",
        options,
        Box::new(|_cc| Box::new(app)),
    );
}

struct SilenceApp {
    name: String,
    age: u32,
    timeline: Timeline,
}

impl Default for SilenceApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            timeline: Timeline::new()
        }
    }
}

impl eframe::App for SilenceApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header_panel").show(ctx, |ui| {
            header(ui, &mut self.timeline);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
        self.timeline.update();
    }
}