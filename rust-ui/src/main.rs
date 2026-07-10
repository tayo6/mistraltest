mod app;
mod knob;

use app::MonoDelayKnobsApp;
use egui::ViewportBuilder;

fn main() {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("MonoDelay-1 Knobs")
            .with_inner_size(egui::Vec2::new(500.0, 250.0))
            .with_min_inner_size(egui::Vec2::new(400.0, 200.0)),
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        ..Default::default()
    };
    
    let _ = eframe::run_native(
        "MonoDelay-1 Knobs",
        options,
        Box::new(|_cc| Box::new(MonoDelayKnobsApp::new())),
    );
}
