use eframe::egui::Ui;

use crate::timeline::Timeline;

pub fn header(ui: &mut Ui, timeline: &mut Timeline) {
    ui.vertical(|ui| {
        ui.label(timeline.current_time().to_string());
        ui.horizontal(|ui| {
            if ui.button("Play").clicked() {
                timeline.play();
            }
            if ui.button("Pause").clicked() {
                timeline.pause();
            }
        })
    });
}