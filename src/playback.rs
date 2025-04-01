use anyhow::Result;
use enigo::{Enigo, KeyboardControllable, MouseControllable, MouseButton, Key};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

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
}

pub struct Playback {
    enigo: Enigo,
}

impl Playback {
    pub fn new() -> Self {
        Self {
            enigo: Enigo::new(),
        }
    }

    pub fn play(&mut self, input_path: PathBuf) -> Result<()> {
        let file_content = std::fs::read_to_string(input_path)?;
        let events: Vec<Event> = serde_json::from_str(&file_content)?;
        let mut last_timestamp = 0;

        println!("Starting playback...");

        for event in events {
            let event_time = match &event {
                Event::MouseMove { timestamp, .. } => *timestamp,
                Event::MouseClick { timestamp, .. } => *timestamp,
                Event::KeyPress { timestamp, .. } => *timestamp,
            };
            
            if last_timestamp > 0 {
                let delay = event_time - last_timestamp;
                if delay > 0 {
                    std::thread::sleep(Duration::from_millis(delay as u64));
                }
            }
            
            last_timestamp = event_time;

            // Execute the event
            match event {
                Event::MouseMove { x, y, .. } => {
                    self.enigo.mouse_move_to(x, y);
                }
                Event::MouseClick { x, y, button, .. } => {
                    self.enigo.mouse_move_to(x, y);
                    // Convert u8 button to MouseButton
                    let mouse_button = match button {
                        1 => MouseButton::Left,
                        2 => MouseButton::Right,
                        3 => MouseButton::Middle,
                        _ => MouseButton::Left, // Default to left click
                    };
                    self.enigo.mouse_click(mouse_button);
                }
                Event::KeyPress { key, .. } => {
                    // Parse the key string and simulate the key press
                    match key.as_str() {
                        "KeyA" => self.enigo.key_click(Key::Layout('a')),
                        "KeyB" => self.enigo.key_click(Key::Layout('b')),
                        "KeyC" => self.enigo.key_click(Key::Layout('c')),
                        "KeyD" => self.enigo.key_click(Key::Layout('d')),
                        "KeyE" => self.enigo.key_click(Key::Layout('e')),
                        "KeyF" => self.enigo.key_click(Key::Layout('f')),
                        "KeyG" => self.enigo.key_click(Key::Layout('g')),
                        "KeyH" => self.enigo.key_click(Key::Layout('h')),
                        "KeyI" => self.enigo.key_click(Key::Layout('i')),
                        "KeyJ" => self.enigo.key_click(Key::Layout('j')),
                        "KeyK" => self.enigo.key_click(Key::Layout('k')),
                        "KeyL" => self.enigo.key_click(Key::Layout('l')),
                        "KeyM" => self.enigo.key_click(Key::Layout('m')),
                        "KeyN" => self.enigo.key_click(Key::Layout('n')),
                        "KeyO" => self.enigo.key_click(Key::Layout('o')),
                        "KeyP" => self.enigo.key_click(Key::Layout('p')),
                        "KeyQ" => self.enigo.key_click(Key::Layout('q')),
                        "KeyR" => self.enigo.key_click(Key::Layout('r')),
                        "KeyS" => self.enigo.key_click(Key::Layout('s')),
                        "KeyT" => self.enigo.key_click(Key::Layout('t')),
                        "KeyU" => self.enigo.key_click(Key::Layout('u')),
                        "KeyV" => self.enigo.key_click(Key::Layout('v')),
                        "KeyW" => self.enigo.key_click(Key::Layout('w')),
                        "KeyX" => self.enigo.key_click(Key::Layout('x')),
                        "KeyY" => self.enigo.key_click(Key::Layout('y')),
                        "KeyZ" => self.enigo.key_click(Key::Layout('z')),
                        "Space" => self.enigo.key_click(Key::Space),
                        "Enter" => self.enigo.key_click(Key::Return),
                        "Escape" => self.enigo.key_click(Key::Escape),
                        _ => println!("Unsupported key: {}", key),
                    }
                }
            }
        }

        println!("Playback completed.");
        Ok(())
    }
} 