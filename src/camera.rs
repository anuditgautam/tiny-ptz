// src/camera.rs
use std::process::Command;
use anyhow::{Result, bail};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ControlConfig {
    pub min: i32,
    pub max: i32,
    pub step: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CameraConfig {
    pub device: String,
    pub pan: ControlConfig,
    pub tilt: ControlConfig,
    pub zoom: ControlConfig,
}

pub struct CameraController {
    pub config: CameraConfig, // This was just made public
    pan_current: i32,
    tilt_current: i32,
    zoom_current: i32,
    pan_prev: i32,
    tilt_prev: i32,
    zoom_prev: i32,
}

impl CameraController {
    pub fn new(config: CameraConfig) -> Self {
        CameraController {
            config,
            pan_current: 0,
            tilt_current: 0,
            zoom_current: 50,
            pan_prev: 0,
            tilt_prev: 0,
            zoom_prev: 50,
        }
    }

    /// Calculate zoom-adjusted step value for pan/tilt movements
    /// When zoomed in (higher zoom values), movements should be smaller and more precise
    /// When zoomed out (lower zoom values), movements can be larger
    fn get_zoom_adjusted_step(&self, base_step: i32) -> i32 {
        let zoom_range = self.config.zoom.max - self.config.zoom.min;
        let zoom_normalized = (self.zoom_current - self.config.zoom.min) as f64 / zoom_range as f64;
        
        // Calculate zoom factor: 1.0 at min zoom (faster), 0.1 at max zoom (slower/precise)
        // This means movements are 10x slower when fully zoomed in for precise control
        let zoom_factor = 1.0 - (zoom_normalized * 0.9);
        
        (base_step as f64 * zoom_factor) as i32
    }

    /// Get the current zoom-adjusted step values for display purposes
    pub fn get_zoom_adjusted_pan_step(&self) -> i32 {
        self.get_zoom_adjusted_step(self.config.pan.step)
    }

    pub fn get_zoom_adjusted_tilt_step(&self) -> i32 {
        self.get_zoom_adjusted_step(self.config.tilt.step)
    }

    /// Sends a v4l2 command if the value has changed.
    /// Takes `&self` (immutable borrow) and `current_prev_value` by value.
    /// Returns Ok(true) if a command was sent successfully, Ok(false) if no change, or Err on failure.
    fn send_v4l2_command(&self, control_name: &str, value: i32, current_prev_value: i32) -> Result<bool> {
        if current_prev_value == value {
            return Ok(false); // No change, so don't send a command
        }

        let output = Command::new("v4l2-ctl")
            .arg("-d")
            .arg(&self.config.device)
            .arg("--set-ctrl")
            .arg(format!("{}={}", control_name, value))
            .output()?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            bail!("Error setting {} to {}: {}", control_name, value, error_msg);
        } else {
            Ok(true) // Command was successfully sent
        }
    }

    // These methods take &mut self to modify current and prev values
    pub fn set_pan(&mut self, delta: i32) -> Result<()> {
        // Use zoom-adjusted step for pan movements
        let adjusted_step = self.get_zoom_adjusted_step(self.config.pan.step);
        let actual_delta = if delta > 0 { adjusted_step } else { -adjusted_step };
        
        self.pan_current = (self.pan_current + actual_delta).clamp(self.config.pan.min, self.config.pan.max);
        // Call send_v4l2_command (which takes &self) and then update self.pan_prev
        if self.send_v4l2_command("pan_absolute", self.pan_current, self.pan_prev)? {
            self.pan_prev = self.pan_current; // Update only if command was actually sent
        }
        Ok(())
    }

    pub fn set_tilt(&mut self, delta: i32) -> Result<()> {
        // Use zoom-adjusted step for tilt movements
        let adjusted_step = self.get_zoom_adjusted_step(self.config.tilt.step);
        let actual_delta = if delta > 0 { adjusted_step } else { -adjusted_step };
        
        self.tilt_current = (self.tilt_current + actual_delta).clamp(self.config.tilt.min, self.config.tilt.max);
        if self.send_v4l2_command("tilt_absolute", self.tilt_current, self.tilt_prev)? {
            self.tilt_prev = self.tilt_current;
        }
        Ok(())
    }

    pub fn set_zoom(&mut self, delta: i32) -> Result<()> {
        self.zoom_current = (self.zoom_current + delta).clamp(self.config.zoom.min, self.config.zoom.max);
        if self.send_v4l2_command("zoom_absolute", self.zoom_current, self.zoom_prev)? {
            self.zoom_prev = self.zoom_current;
        }
        Ok(())
    }

    pub fn get_pan(&self) -> i32 { self.pan_current }
    pub fn get_tilt(&self) -> i32 { self.tilt_current }
    pub fn get_zoom(&self) -> i32 { self.zoom_current }
}