# MonoDelay-1 Knobs - Rust UI

Interactive knob controls with angular gradient arcs built with [eframe](https://github.com/emilk/egui) and [egui](https://github.com/emilk/egui).

## Features

- Three interactive knobs with smooth dragging
- Angular gradient arcs showing dial range (0-100)
- 10px outer stroke indicating the full range
- Color-coded knobs (Blue, Green, Purple)
- Real-time value display
- WASM support for web deployment

## Screenshot

The UI displays three knobs labeled "Delay", "Feedback", and "Mix" with gradient arcs that fill as you turn the knobs.

## Project Structure

```
rust-ui/
├── Cargo.toml          # Rust project configuration
├── src/
│   ├── main.rs         # Native application entry point
│   ├── lib.rs          # Library exports for WASM
│   ├── app.rs          # Main application state and UI
│   └── knob.rs         # Custom knob widget implementation
├── web/
│   ├── index.html      # HTML page for WASM deployment
│   └── pkg/            # Generated WASM and JS files (after build)
├── build-wasm.sh       # Script to build WASM version
└── README.md           # This file
```

## Building

### Native Application

```bash
cd rust-ui
cargo run --release
```

This will open a native window with the three knobs.

### WASM for Web

1. **Build the WASM module:**
   ```bash
   cd rust-ui
   ./build-wasm.sh
   ```

2. **Or manually:**
   ```bash
   # Install wasm target
   rustup target add wasm32-unknown-unknown
   
   # Install wasm-bindgen
   cargo install wasm-bindgen-cli
   
   # Build
   cargo build --target wasm32-unknown-unknown --release --lib
   
   # Generate bindings
   wasm-bindgen --target web --out-dir ./web/pkg \
     ./target/wasm32-unknown-unknown/release/monodelay_knobs.wasm
   
   # Copy WASM file
   cp ./target/wasm32-unknown-unknown/release/monodelay_knobs.wasm ./web/pkg/
   ```

3. **Serve locally:**
   ```bash
   # Using Python
   python3 -m http.server 8080 --directory rust-ui/web
   
   # Using npm
   npx http-server rust-ui/web -p 8080
   
   # Then open http://localhost:8080
   ```

## GitHub Actions Workflow

The repository includes a GitHub Actions workflow (`.github/workflows/rust-wasm.yml`) that:

1. **Builds the WASM module** on every push to main
2. **Deploys to GitHub Pages** for preview
3. **Uploads artifacts** for download

### Accessing the Preview

After pushing to main, the WASM build will be automatically deployed to GitHub Pages:

```
https://<username>.github.io/MonoDelay-1/rust-ui/web/
```

Replace `<username>` with your GitHub username.

## Customization

### Changing Knob Colors

Edit the colors in `src/app.rs`:

```rust
// Knob 1 - Blue gradient
.arc_color(Color32::from_rgb(100, 200, 255))
.track_color(Color32::from_rgb(40, 40, 50))

// Knob 2 - Green gradient
.arc_color(Color32::from_rgb(100, 255, 150))
.track_color(Color32::from_rgb(40, 50, 40))

// Knob 3 - Purple gradient
.arc_color(Color32::from_rgb(200, 150, 255))
.track_color(Color32::from_rgb(50, 40, 50))
```

### Adding More Knobs

Add additional knobs in `src/app.rs`:

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
        .stroke_width(10.0),
);
```

### Changing Value Range

The knobs support any range. Change the range in `src/app.rs`:

```rust
.range(0.0, 100.0)  // Default: 0-100
.range(0.0, 1.0)    // For normalized values
.range(-50.0, 50.0) // For bidirectional values
```

## Technical Details

### Knob Widget Implementation

The `Knob` widget in `src/knob.rs` implements:

- **Angular gradient arcs**: Multiple segments with interpolated colors
- **10px outer stroke**: Visual indicator of the full range
- **Smooth dragging**: Mouse interaction with angle calculation
- **Value display**: Numeric value shown above each knob
- **Label**: Text label below each knob

### WASM Integration

The WASM build uses:
- `wasm32-unknown-unknown` target
- `wasm-bindgen` for JavaScript interop
- `web-sys` for browser APIs (via eframe)

## Dependencies

- [eframe](https://crates.io/crates/eframe) - Framework for egui applications
- [egui](https://crates.io/crates/egui) - Immediate mode GUI library
- [epaint](https://crates.io/crates/epaint) - Painting backend for egui

## License

This project is part of the MonoDelay-1 repository. See the main repository for licensing information.
