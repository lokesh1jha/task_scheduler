# Task Scheduler

A Rust-based desktop application for recording and replaying mouse and keyboard actions.

## Features

- Record mouse movements and clicks
- Record keyboard inputs
- Save recorded actions to a JSON file
- Replay recorded actions with timing accuracy
- Cross-platform support (Windows, macOS, Linux)

## Installation

1. Make sure you have Rust installed on your system
2. Clone this repository
3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Recording Actions

To record mouse and keyboard actions:

```bash
cargo run -- record --output recorded_actions.json
```

- The recording will start immediately
- Press the 'Esc' key to stop recording
- The recorded actions will be saved to the specified output file

### Playing Back Actions

To replay recorded actions:

```bash
cargo run -- playback --input recorded_actions.json
```

- The playback will start immediately
- The actions will be replayed with the same timing as recorded

## Requirements

- Rust 1.70 or later
- macOS, Windows, or Linux operating system
- Appropriate permissions for mouse and keyboard control

## Notes

- The application requires appropriate system permissions to control the mouse and keyboard
- Some systems may require additional setup or permissions for automation
- Use with caution when recording sensitive information 