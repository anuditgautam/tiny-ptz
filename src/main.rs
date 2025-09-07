use anyhow::Result;
use crossterm::{
    event::{self, Event as CrosstermEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend; // Import CrosstermBackend here
use ratatui::Terminal; // Import Terminal separately for clarity
use std::{io, time::Duration};
use tokio::sync::mpsc;

use crate::app::{App, InputEvent};
use crate::camera::CameraConfig;

mod app;
mod camera;
mod ui;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config_str = std::fs::read_to_string("config.toml")
        .expect("Failed to read config.toml");
    let config: CameraConfig = toml::from_str(&config_str)
        .expect("Failed to parse config.toml");

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?; // `Terminal` should now be resolved

    // Create app and run it
    let mut app = App::new(config);

    // Event handling channel
    let (tx, mut rx) = mpsc::channel(100);

    // Input polling task
    tokio::spawn(async move {
        loop {
            // Poll for events. Adjust poll duration as needed.
            if event::poll(Duration::from_millis(50)).unwrap() {
                if let CrosstermEvent::Key(key) = event::read().unwrap() {
                    // Send key events to the app
                    tx.send(InputEvent::Key(key)).await.unwrap();
                }
            }
            // Send a tick event regularly to update UI or handle time-based logic
            tx.send(InputEvent::Tick).await.unwrap();
            // Small sleep to prevent busy-looping and allow other tasks to run
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    });

    loop {
        // Draw the UI
        terminal.draw(|f| ui::render::<CrosstermBackend<io::Stdout>>(f, &app))?;

        // Process events from the channel
        if let Some(event) = rx.recv().await {
            app.update(event);
            if app.should_quit {
                break;
            }
        }
    }

    // Restore terminal state before exiting
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Cleanup any running processes
    app.cleanup();

    Ok(())
}