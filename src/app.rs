use crate::camera::{CameraController, CameraConfig};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::time::{Duration, Instant};
use std::process::Command;

pub enum InputEvent {
    Key(KeyEvent),
    Tick,
}

pub struct App {
    pub camera_controller: CameraController, // Make this field public
    pub should_quit: bool,
    pub status_message: String,
    last_command_time: Instant,
    command_interval: Duration,
    video_feed_pid: Option<u32>,
}

impl App {
    pub fn new(config: CameraConfig) -> Self {
        App {
            camera_controller: CameraController::new(config),
            should_quit: false,
            status_message: "Press 'q' to quit. Arrow keys for Pan/Tilt. Shift+Arrows for Zoom. 'v' for video feed.".to_string(),
            last_command_time: Instant::now(),
            command_interval: Duration::from_millis(100),
            video_feed_pid: None,
        }
    }

    fn toggle_video_feed(&mut self) {
        if let Some(pid) = self.video_feed_pid.take() {
            // Video feed is running, kill it
            let _ = Command::new("kill")
                .arg(pid.to_string())
                .output();
            self.status_message = "Video feed stopped.".to_string();
        } else {
            // Start video feed in background
            let device = self.camera_controller.config.device.clone();
            let command = format!(
                "ffplay {} -fflags nobuffer -flags low_delay -framedrop -sync ext -hide_banner -loglevel error >/dev/null 2>&1 & echo $!",
                device
            );
            
            match Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output() {
                Ok(output) => {
                    if output.status.success() {
                        // Parse the PID from the output
                        if let Ok(pid_str) = String::from_utf8(output.stdout) {
                            let pid = pid_str.trim();
                            if let Ok(pid_num) = pid.parse::<u32>() {
                                self.video_feed_pid = Some(pid_num);
                                self.status_message = format!("Video feed started (PID: {}). Press 'v' again to stop.", pid_num);
                            } else {
                                self.status_message = "Video feed started in background. Press 'v' again to stop.".to_string();
                            }
                        } else {
                            self.status_message = "Video feed started in background. Press 'v' again to stop.".to_string();
                        }
                    } else {
                        self.status_message = "Failed to start video feed.".to_string();
                    }
                }
                Err(e) => {
                    self.status_message = format!("Failed to start video feed: {}", e);
                }
            }
        }
    }

    pub fn update(&mut self, event: InputEvent) {
        match event {
            InputEvent::Key(key) => {
                let now = Instant::now();
                if now.duration_since(self.last_command_time) >= self.command_interval {
                    let result = match (key.code, key.modifiers) {
                        (KeyCode::Left, _) => self.camera_controller.set_pan(-self.camera_controller.config.pan.step),
                        (KeyCode::Right, _) => self.camera_controller.set_pan(self.camera_controller.config.pan.step),
                        (KeyCode::Up, KeyModifiers::SHIFT) => self.camera_controller.set_zoom(self.camera_controller.config.zoom.step),
                        (KeyCode::Down, KeyModifiers::SHIFT) => self.camera_controller.set_zoom(-self.camera_controller.config.zoom.step),
                        (KeyCode::Up, _) => self.camera_controller.set_tilt(self.camera_controller.config.tilt.step),
                        (KeyCode::Down, _) => self.camera_controller.set_tilt(-self.camera_controller.config.tilt.step),
                        (KeyCode::Char('v'), _) => {
                            self.toggle_video_feed();
                            Ok(())
                        }
                        (KeyCode::Char('q'), _) => {
                            self.should_quit = true;
                            Ok(())
                        }
                        _ => Ok(()), // Ignore other keys
                    };

                    match result {
                        Ok(_) => {
                            if !matches!(key.code, KeyCode::Char('v')) {
                                self.status_message = "Command sent.".to_string();
                            }
                        }
                        Err(e) => self.status_message = format!("Error: {}", e),
                    }
                    self.last_command_time = now;
                }
            }
            InputEvent::Tick => {
                // Update any time-sensitive UI elements if needed
            }
        }
    }

    pub fn get_pan(&self) -> i32 { self.camera_controller.get_pan() }
    pub fn get_tilt(&self) -> i32 { self.camera_controller.get_tilt() }
    pub fn get_zoom(&self) -> i32 { self.camera_controller.get_zoom() }
    
    // Add getters for zoom-adjusted step values
    pub fn get_zoom_adjusted_pan_step(&self) -> i32 { 
        self.camera_controller.get_zoom_adjusted_pan_step() 
    }
    
    pub fn get_zoom_adjusted_tilt_step(&self) -> i32 { 
        self.camera_controller.get_zoom_adjusted_tilt_step() 
    }

    /// Cleanup method to be called when the app is shutting down
    pub fn cleanup(&mut self) {
        if let Some(pid) = self.video_feed_pid.take() {
            let _ = Command::new("kill")
                .arg(pid.to_string())
                .output();
        }
    }
}