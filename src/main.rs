use eframe::NativeOptions;

mod recorder;
mod playback;
mod gui;

use gui::TaskSchedulerApp;

fn main() -> eframe::Result<()> {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Task Scheduler",
        options,
        Box::new(|_cc| Box::new(TaskSchedulerApp::new())),
    )
} 