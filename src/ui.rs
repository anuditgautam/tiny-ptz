use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};
use crate::app::App;

pub fn render<B: Backend>(f: &mut Frame, app: &App) { // Remove <B> from Frame
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title and Status
            Constraint::Min(0),    // Main content
        ])
        .split(f.size());

    // Title Block
    f.render_widget(
        Paragraph::new("Camera PTZ Controller")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title("Info")),
        chunks[0],
    );

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[1]);

    // Pan/Tilt Block
    let ptz_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Pan
            Constraint::Length(3), // Tilt
            Constraint::Length(3), // Zoom
            Constraint::Length(4), // Movement Speed Info
            Constraint::Min(0),    // Status/Help
        ])
        .split(main_chunks[0]);

    // Pan
    f.render_widget(
        Paragraph::new(format!("Pan: {}", app.get_pan()))
            .block(Block::default().borders(Borders::ALL).title("Pan")),
        ptz_chunks[0],
    );

    // Tilt
    f.render_widget(
        Paragraph::new(format!("Tilt: {}", app.get_tilt()))
            .block(Block::default().borders(Borders::ALL).title("Tilt")),
        ptz_chunks[1],
    );

    // Zoom (using Gauge for visual representation)
    let zoom_config = &app.camera_controller.config.zoom;
    let zoom_percentage = ((app.get_zoom() - zoom_config.min) as f64 / (zoom_config.max - zoom_config.min) as f64) * 100.0;
    f.render_widget(
        Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Zoom"))
            .gauge_style(Style::default().fg(Color::Magenta).bg(Color::Black))
            .percent(zoom_percentage as u16)
            .label(format!("{}%", app.get_zoom())),
        ptz_chunks[2],
    );

    // Movement Speed Info
    let pan_step = app.get_zoom_adjusted_pan_step();
    let tilt_step = app.get_zoom_adjusted_tilt_step();
    let base_pan_step = app.camera_controller.config.pan.step;
    let base_tilt_step = app.camera_controller.config.tilt.step;
    
    let speed_info = format!(
        "Movement Speed (Zoom-Adjusted):\n\
         Pan: {} (base: {})\n\
         Tilt: {} (base: {})",
        pan_step, base_pan_step, tilt_step, base_tilt_step
    );
    
    f.render_widget(
        Paragraph::new(speed_info)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Speed Info")),
        ptz_chunks[3],
    );

    // Status/Help
    f.render_widget(
        Paragraph::new(app.status_message.clone())
            .block(Block::default().borders(Borders::ALL).title("Status")),
        ptz_chunks[4],
    );

    // Keybindings Block
    f.render_widget(
        Paragraph::new(
            "Keybindings:\n\
             ←/→: Pan (speed varies with zoom)\n\
             ↑/↓: Tilt (speed varies with zoom)\n\
             Shift+↑/↓: Zoom\n\
             v: Toggle video feed\n\
             q: Quit\n\
             \n\
             Note: Movement speed automatically\n\
             adjusts based on zoom level"
        )
        .block(Block::default().borders(Borders::ALL).title("Help")),
        main_chunks[1],
    );
}