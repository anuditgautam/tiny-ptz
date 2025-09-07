# Tiny PTZ Camera Controller

A lightweight, terminal-based PTZ (Pan-Tilt-Zoom) camera controller built in Rust. Control your V4L2-compatible PTZ cameras with an intuitive keyboard interface and real-time video feed.

## Features

- **Intuitive Controls**: Arrow keys for pan/tilt, Shift+arrows for zoom
- **Live Video Feed**: Toggle video preview with a single key press
- **Zoom-Adjusted Speed**: Movement speed automatically adjusts based on zoom level for precise control
- **Terminal UI**: Clean, modern dashboard interface using Ratatui
- **Configurable**: TOML-based configuration for camera settings and limits
- **V4L2 Integration**: Direct control via v4l2-ctl commands

## Screenshots

The application provides a real-time dashboard showing:
- Current pan, tilt, and zoom values
- Movement speed information (zoom-adjusted)
- Live status updates
- Interactive help and keybindings

## Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- V4L2 utilities (`v4l2-ctl`)
- FFmpeg (for video feed functionality)
- A V4L2-compatible PTZ camera

### Installing Dependencies

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install v4l-utils ffmpeg
```

**Arch Linux:**
```bash
sudo pacman -S v4l-utils ffmpeg
```

**macOS (with Homebrew):**
```bash
brew install v4l-utils ffmpeg
```

## Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/tiny-ptz.git
   cd tiny-ptz
   ```

2. **Build the project:**
   ```bash
   # Debug build
   cargo build
   
   # Release build (recommended)
   cargo build --release
   ```

3. **Run the application:**
   ```bash
   # Run directly
   cargo run
   
   # Or run the compiled binary
   ./target/release/tiny-ptz
   ```

## Configuration

The application uses a `config.toml` file for configuration. An example configuration is provided:

```toml
device = "/dev/video0"

[pan]
min = -468000
max = 468000
step = 30000

[tilt]
min = -324000
max = 324000
step = 30000

[zoom]
min = 0
max = 100
step = 10
```

### Configuration Options

- `device`: Path to your camera device (usually `/dev/video0`)
- `pan/tilt/zoom.min`: Minimum value for the control
- `pan/tilt/zoom.max`: Maximum value for the control
- `pan/tilt/zoom.step`: Base step size for movements

## Usage

### Controls

| Key | Action |
|-----|--------|
| `←` `→` | Pan left/right (speed varies with zoom) |
| `↑` `↓` | Tilt up/down (speed varies with zoom) |
| `Shift + ↑` `↓` | Zoom in/out |
| `v` | Toggle video feed |
| `q` | Quit application |

### Smart Movement Speed

The application automatically adjusts movement speed based on zoom level:
- **Zoomed out**: Faster, larger movements for quick positioning
- **Zoomed in**: Slower, precise movements for fine control

This ensures optimal control precision at all zoom levels.

## Building from Source

### Requirements

- Rust 1.70+
- Cargo (comes with Rust)

### Build Commands

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check code
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Development

### Project Structure

```
tiny-ptz/
├── src/
│   ├── main.rs      # Application entry point
│   ├── app.rs       # Main application logic
│   ├── camera.rs    # Camera control implementation
│   └── ui.rs        # Terminal UI rendering
├── docs/            # Documentation files
├── examples/        # Example configurations
├── scripts/         # Build and utility scripts
├── config.toml      # Configuration file
├── Cargo.toml       # Rust project configuration
├── Cargo.lock       # Dependency lock file
├── .gitignore       # Git ignore rules
├── LICENSE          # License file
└── README.md        # This file
```

### Adding New Features

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Troubleshooting

### Common Issues

**Camera not detected:**
- Ensure your camera is V4L2-compatible
- Check device permissions: `ls -la /dev/video*`
- Try different device paths (e.g., `/dev/video1`)

**Video feed not working:**
- Install FFmpeg: `sudo apt install ffmpeg`
- Check camera permissions
- Verify the device path in `config.toml`

**Build errors:**
- Update Rust: `rustup update`
- Clean build cache: `cargo clean`
- Check Rust version: `rustc --version`

### Debug Mode

Run with debug output:
```bash
RUST_LOG=debug cargo run
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Ratatui](https://github.com/ratatui-org/ratatui) for the terminal UI
- Uses [Crossterm](https://github.com/crossterm-rs/crossterm) for cross-platform terminal handling
- Camera control via V4L2 utilities

## Roadmap

- [ ] Preset positions and saved configurations
- [ ] Web-based control interface
- [ ] Recording functionality
- [ ] Multiple camera support
- [ ] Configuration GUI
- [ ] Remote control via network

---

**Note**: This project is designed for V4L2-compatible PTZ cameras. Make sure your camera supports the required V4L2 controls before using this application.