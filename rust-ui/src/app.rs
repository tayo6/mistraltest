use crate::knob::Knob;
use egui::{Color32, CentralPanel, Ui};

/// Main application state
pub struct MonoDelayKnobsApp {
    knob1_value: f32,
    knob2_value: f32,
    knob3_value: f32,
    bg_color: Color32,
    // Track which knob is being dragged
    dragging_knob: Option<usize>,
}

impl Default for MonoDelayKnobsApp {
    fn default() -> Self {
        Self {
            knob1_value: 30.0,
            knob2_value: 60.0,
            knob3_value: 90.0,
            bg_color: Color32::from_rgb(30, 30, 30),
            dragging_knob: None,
        }
    }
}

impl MonoDelayKnobsApp {
    pub fn new() -> Self {
        Self::default()
    }

    fn draw_knobs(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = egui::Vec2::new(40.0, 0.0);
            
            // Knob 1 - Blue gradient
            let response = ui.add(
                Knob::new("Delay")
                    .value(self.knob1_value)
                    .range(0.0, 100.0)
                    .radius(80.0)
                    .arc_color(Color32::from_rgb(100, 200, 255))
                    .track_color(Color32::from_rgb(40, 40, 50))
                    .stroke_width(10.0)
                    .id("knob1"),
            );
            
            // Check if this knob is being dragged
            if response.dragged() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let center = response.rect.center();
                    let angle = (pos.y - center.y).atan2(pos.x - center.x);
                    let normalized = (angle + std::f32::consts::PI / 2.0) / (std::f32::consts::PI * 1.8);
                    self.knob1_value = 0.0 + normalized.clamp(0.0, 1.0) * 100.0;
                }
            }
            
            // Knob 2 - Green gradient
            let response = ui.add(
                Knob::new("Feedback")
                    .value(self.knob2_value)
                    .range(0.0, 100.0)
                    .radius(80.0)
                    .arc_color(Color32::from_rgb(100, 255, 150))
                    .track_color(Color32::from_rgb(40, 50, 40))
                    .stroke_width(10.0)
                    .id("knob2"),
            );
            
            if response.dragged() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let center = response.rect.center();
                    let angle = (pos.y - center.y).atan2(pos.x - center.x);
                    let normalized = (angle + std::f32::consts::PI / 2.0) / (std::f32::consts::PI * 1.8);
                    self.knob2_value = 0.0 + normalized.clamp(0.0, 1.0) * 100.0;
                }
            }
            
            // Knob 3 - Purple gradient
            let response = ui.add(
                Knob::new("Mix")
                    .value(self.knob3_value)
                    .range(0.0, 100.0)
                    .radius(80.0)
                    .arc_color(Color32::from_rgb(200, 150, 255))
                    .track_color(Color32::from_rgb(50, 40, 50))
                    .stroke_width(10.0)
                    .id("knob3"),
            );
            
            if response.dragged() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let center = response.rect.center();
                    let angle = (pos.y - center.y).atan2(pos.x - center.x);
                    let normalized = (angle + std::f32::consts::PI / 2.0) / (std::f32::consts::PI * 1.8);
                    self.knob3_value = 0.0 + normalized.clamp(0.0, 1.0) * 100.0;
                }
            }
        });
    }
}

impl eframe::App for MonoDelayKnobsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default()
            .frame(egui::Frame {
                fill: self.bg_color,
                rounding: egui::Rounding::same(0.0),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("MonoDelay-1 Controls");
                    ui.separator();
                    
                    self.draw_knobs(ui);
                    
                    // Display current values
                    ui.horizontal(|ui| {
                        ui.label(format!("Delay: {:.1}%", self.knob1_value));
                        ui.label(format!("Feedback: {:.1}%", self.knob2_value));
                        ui.label(format!("Mix: {:.1}%", self.knob3_value));
                    });
                });
            });
    }
}
