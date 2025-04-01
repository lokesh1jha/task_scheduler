use anyhow::Result;
use chrono::Utc;
use device_query::{DeviceQuery, DeviceState, Keycode};
use screenshots::Screen;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    MouseMove {
        x: i32,
        y: i32,
        timestamp: i64,
    },
    MouseClick {
        x: i32,
        y: i32,
        button: u8,
        timestamp: i64,
    },
    KeyPress {
        key: String,
        timestamp: i64,
    },
    ScreenCapture {
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        timestamp: i64,
    },
}

pub struct Recorder {
    device_state: DeviceState,
    capture_interval: Duration,
    last_capture: Instant,
}

impl Recorder {
    pub fn new() -> Self {
        Self {
            device_state: DeviceState::new(),
            capture_interval: Duration::from_secs(1), // Capture screen every second
            last_capture: Instant::now(),
        }
    }

    fn capture_screen_region(&self, x: i32, y: i32) -> Result<()> {
        let screens = Screen::all()?;
        if let Some(screen) = screens.first() {
            let image = screen.capture_area(x, y, 100, 100)?; // Capture 100x100 region around cursor
            let timestamp = Utc::now().timestamp_millis();
            
            // Save the image with timestamp
            let filename = format!("captures/capture_{}.png", timestamp);
            std::fs::create_dir_all("captures")?;
            image.save(filename)?;
        }
        Ok(())
    }

    pub fn record(&mut self, output_path: PathBuf) -> Result<()> {
        let mut events = Vec::new();
        let mut last_mouse_pos = (0, 0);

        println!("Recording started. Press 'Esc' to stop.");

        loop {
            let timestamp = Utc::now().timestamp_millis();

            // Record mouse movement with improved tracking
            let mouse = self.device_state.get_mouse();
            if mouse.coords != last_mouse_pos {
                let event = Event::MouseMove {
                    x: mouse.coords.0,
                    y: mouse.coords.1,
                    timestamp,
                };
                events.push(event);
                last_mouse_pos = mouse.coords;

                // Capture screen region around cursor periodically
                if self.last_capture.elapsed() >= self.capture_interval {
                    if let Err(e) = self.capture_screen_region(mouse.coords.0, mouse.coords.1) {
                        eprintln!("Screen capture error: {}", e);
                    }
                    self.last_capture = Instant::now();
                }
            }

            // Record mouse clicks with all buttons
            for (i, &pressed) in mouse.button_pressed.iter().enumerate() {
                if pressed {
                    let event = Event::MouseClick {
                        x: mouse.coords.0,
                        y: mouse.coords.1,
                        button: i as u8,
                        timestamp,
                    };
                    events.push(event);
                }
            }

            // Record keyboard events
            let keys: Vec<Keycode> = self.device_state.get_keys();
            for key in keys {
                let event = Event::KeyPress {
                    key: format!("{:?}", key),
                    timestamp,
                };
                events.push(event);
            }

            // Check for escape key to stop recording
            if self.device_state.get_keys().contains(&Keycode::Escape) {
                break;
            }

            std::thread::sleep(Duration::from_millis(10));
        }

        // Write all events as a single JSON array
        let mut file = File::create(output_path)?;
        file.write_all(serde_json::to_string_pretty(&events)?.as_bytes())?;

        println!("Recording stopped.");
        Ok(())
    }
} 