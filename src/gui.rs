use anyhow::Result;
use eframe::egui;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use device_query::{DeviceQuery, DeviceState};

use crate::recorder::Recorder;
use crate::playback::Playback;

pub struct TaskSchedulerApp {
    recording: Arc<AtomicBool>,
    playback_file: Option<PathBuf>,
    record_file: Option<PathBuf>,
    mouse_position: Arc<Mutex<(i32, i32)>>,
    status_message: String,
    device_state: DeviceState,
}

impl TaskSchedulerApp {
    pub fn new() -> Self {
        Self {
            recording: Arc::new(AtomicBool::new(false)),
            playback_file: None,
            record_file: None,
            mouse_position: Arc::new(Mutex::new((0, 0))),
            status_message: String::new(),
            device_state: DeviceState::new(),
        }
    }

    fn start_recording(&mut self) -> Result<()> {
        if self.recording.load(Ordering::SeqCst) {
            return Ok(());
        }

        let output_path = PathBuf::from("recorded_actions.json");
        self.record_file = Some(output_path.clone());
        self.recording.store(true, Ordering::SeqCst);
        self.status_message = "Recording started...".to_string();

        let recording = self.recording.clone();
        // let _mouse_position = self.mouse_position.clone();
        
        thread::spawn(move || {
            let mut recorder = Recorder::new();
            if let Err(e) = recorder.record(output_path) {
                eprintln!("Recording error: {}", e);
            }
            recording.store(false, Ordering::SeqCst);
            println!("Recording stopped."); // This will now print when recording actually stops
        });

        Ok(())
    }

    fn stop_recording(&mut self) -> Result<()> {
        self.recording.store(false, Ordering::SeqCst);
        self.status_message = "Recording stopped.".to_string();
        Ok(())
    }

    fn start_playback(&mut self) -> Result<()> {
        if let Some(input_path) = &self.playback_file {
            let mut playback = Playback::new();
            if let Err(e) = playback.play(input_path.clone()) {
                self.status_message = format!("Playback error: {}", e);
            } else {
                self.status_message = "Playback completed.".to_string();
            }
        }
        Ok(())
    }

    fn update_mouse_position(&mut self) {
        let mouse = self.device_state.get_mouse();
        if let Ok(mut pos) = self.mouse_position.lock() {
            *pos = mouse.coords;
        }
    }
}

impl eframe::App for TaskSchedulerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update mouse position
        self.update_mouse_position();
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Task Scheduler");
            
            // Mouse position display
            let mouse_pos = if let Ok(pos) = self.mouse_position.lock() {
                *pos
            } else {
                (0, 0)
            };
            ui.label(format!("Mouse Position: {:?}", mouse_pos));

            // Recording controls
            ui.horizontal(|ui| {
                if self.recording.load(Ordering::SeqCst) {
                    if ui.button("Stop Recording").clicked() {
                        if let Err(e) = self.stop_recording() {
                            self.status_message = format!("Error stopping recording: {}", e);
                        }
                    }
                } else {
                    if ui.button("Start Recording").clicked() {
                        if let Err(e) = self.start_recording() {
                            self.status_message = format!("Error starting recording: {}", e);
                        }
                    }
                }

                // File selection for playback
                if ui.button("Select Playback File").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("JSON", &["json"])
                        .pick_file()
                    {
                        self.playback_file = Some(path);
                    }
                }

                if ui.button("Start Playback").clicked() {
                    if let Err(e) = self.start_playback() {
                        self.status_message = format!("Error during playback: {}", e);
                    }
                }
            });

            // Status message
            ui.label(&self.status_message);
        });

        // Request continuous updates
        ctx.request_repaint();
    }
} 