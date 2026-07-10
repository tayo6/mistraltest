# MonoDelay-1 Knobs - Deployment Guide

This guide explains how to build, test, and deploy the MonoDelay-1 Knobs Rust application with eframe.

## Quick Start

### 1. Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Node.js](https://nodejs.org/) (optional, for npm-based HTTP server)
- Git

### 2. Clone and Navigate

```bash
cd MonoDelay-1/rust-ui
```

## Building

### Native Application

Build and run the native desktop application:

```bash
cargo run --release
```

This will open a window with three interactive knobs.

**Controls:**
- Click and drag on any knob to rotate it
- Values range from 0 to 100
- Each knob has a different color gradient (Blue, Green, Purple)

### WASM for Web

#### Method 1: Using the deploy script

```bash
./deploy.sh
```

This will:
1. Install required dependencies
2. Build the WASM module
3. Generate JavaScript bindings
4. Prepare files for deployment

#### Method 2: Manual build

```bash
# Install wasm target
rustup target add wasm32-unknown-unknown

# Install wasm-bindgen
cargo install wasm-bindgen-cli

# Build the library as WASM
cargo build --target wasm32-unknown-unknown --release --lib

# Generate JavaScript bindings
wasm-bindgen --target web --out-dir ./web/pkg \
    ./target/wasm32-unknown-unknown/release/monodelay_knobs.wasm

# Copy WASM file
cp ./target/wasm32-unknown-unknown/release/monodelay_knobs.wasm ./web/pkg/
```

## Local Testing

### Using Python's built-in HTTP server

```bash
python3 -m http.server 8080 --directory rust-ui/web
```

Then open: http://localhost:8080

### Using npm HTTP server

```bash
# Install http-server globally
npm install -g http-server

# Serve the web directory
http-server rust-ui/web -p 8080
```

Then open: http://localhost:8080

### Using cargo web (alternative)

```bash
cargo install cargo-web
cargo web start --target wasm32-unknown-unknown --release
```

## GitHub Actions Deployment

The repository includes a GitHub Actions workflow (`.github/workflows/rust-wasm.yml`) that automatically:

1. **Builds the WASM module** on every push to main
2. **Deploys to GitHub Pages** for preview
3. **Uploads artifacts** for download

### How it works

1. **Trigger**: Push to `main` branch or open a PR
2. **Build job**: Compiles the Rust code to WASM
3. **Deploy job**: Uploads the web files to GitHub Pages

### Accessing the Preview

After pushing to main, the WASM build will be automatically deployed to:

```
https://<username>.github.io/MonoDelay-1/rust-ui/web/
```

Replace `<username>` with your GitHub username.

### Manual Trigger

You can manually trigger the workflow:

1. Go to: `https://github.com/<username>/MonoDelay-1/actions/workflows/rust-wasm.yml`
2. Click "Run workflow"
3. Select the branch and click "Run workflow"

## GitHub Pages Configuration

To enable GitHub Pages for your repository:

1. Go to **Settings** > **Pages**
2. Under "Source", select **GitHub Actions**
3. Click **Save**

The workflow will automatically deploy to GitHub Pages.

## Customization

### Changing Knob Appearance

Edit `src/knob.rs` to modify:

- **Colors**: Change `arc_color` and `track_color`
- **Size**: Adjust `radius` and `stroke_width`
- **Gradient**: Modify the color interpolation in `draw_arc`

### Adding More Knobs

Edit `src/app.rs`:

```rust
// In MonoDelayKnobsApp struct
knob4_value: f32,

// In draw_knobs method
let response = ui.add(
    Knob::new("New Knob")
        .value(self.knob4_value)
        .range(0.0, 100.0)
        .radius(80.0)
        .arc_color(Color32::from_rgb(255, 200, 100))
        .track_color(Color32::from_rgb(50, 50, 40))
        .stroke_width(10.0)
        .id("knob4"),
);

if response.dragged() {
    if let Some(pos) = response.interact_pointer_pos() {
        let center = response.rect.center();
        let angle = (pos.y - center.y).atan2(pos.x - center.x);
        let normalized = (angle + std::f32::consts::PI / 2.0) / (std::f32::consts::PI * 1.8);
        self.knob4_value = 0.0 + normalized.clamp(0.0, 1.0) * 100.0;
    }
}
```

### Changing Value Range

The knobs support any range. Change the range in `src/app.rs`:

```rust
.range(0.0, 100.0)  // Default: 0-100
.range(0.0, 1.0)    // For normalized values (0.0 to 1.0)
.range(-50.0, 50.0) // For bidirectional values (-50 to 50)
```

### Custom Colors

Use RGB values (0-255):

```rust
Color32::from_rgb(255, 0, 0)      // Red
Color32::from_rgb(0, 255, 0)      // Green
Color32::from_rgb(0, 0, 255)      // Blue
Color32::from_rgb(255, 255, 255)  // White
Color32::from_rgb(0, 0, 0)        // Black
```

## Troubleshooting

### WASM Build Issues

**Error: `wasm-bindgen` not found**

```bash
cargo install wasm-bindgen-cli
```

**Error: wasm target not installed**

```bash
rustup target add wasm32-unknown-unknown
```

### Local Server Issues

**Error: Module not found**

Make sure you've built the WASM module first:

```bash
./deploy.sh
```

**Error: CORS issues**

Use a proper HTTP server, not `file://` protocol. Python's HTTP server works well:

```bash
python3 -m http.server 8080 --directory rust-ui/web
```

### GitHub Actions Issues

**Workflow not triggering**

Check that:
1. The workflow file is in `.github/workflows/rust-wasm.yml`
2. The file has correct permissions
3. You've pushed to the correct branch

**Deployment failing**

Check the workflow logs in GitHub Actions for specific errors.

## Project Structure

```
rust-ui/
├── Cargo.toml              # Rust project configuration
├── Cargo.lock              # Dependency lock file (generated)
├── src/
│   ├── main.rs             # Native application entry point
│   ├── lib.rs              # Library exports for WASM
│   ├── app.rs              # Main application state and UI
│   └── knob.rs             # Custom knob widget implementation
├── web/
│   ├── index.html          # HTML page for WASM deployment
│   └── pkg/                # Generated WASM and JS files
├── .gitignore              # Git ignore rules
├── build-wasm.sh           # Script to build WASM version
├── deploy.sh               # Full deployment script
├── README.md               # Main documentation
└── DEPLOYMENT.md           # This file
```

## Performance Tips

1. **Release builds**: Always use `--release` flag for production
2. **Optimization**: The Cargo.toml includes LTO (Link Time Optimization)
3. **WASM size**: The WASM file is optimized with `strip = true`

## Browser Compatibility

The WASM module works in modern browsers:
- Chrome (recommended for development)
- Firefox
- Safari
- Edge

For older browsers, you may need to add polyfills for WebAssembly.

## License

This project is part of the MonoDelay-1 repository. See the main repository for licensing information.
