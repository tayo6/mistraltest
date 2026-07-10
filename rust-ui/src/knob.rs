use egui::{Color32, Painter, Pos2, Rect, Response, Sense, Stroke, Ui, Widget, WidgetText};
use epaint::{CircleShape, PathShape};
use std::sync::Arc;

/// A custom knob widget with angular gradient arc
pub struct Knob {
    value: f32,
    min: f32,
    max: f32,
    radius: f32,
    label: String,
    arc_color: Color32,
    track_color: Color32,
    stroke_width: f32,
    id: String,
}

impl Knob {
    pub fn new(label: impl Into<String>) -> Self {
        let label_str = label.into();
        Self {
            value: 50.0,
            min: 0.0,
            max: 100.0,
            radius: 80.0,
            label: label_str.clone(),
            arc_color: Color32::from_rgb(100, 200, 255),
            track_color: Color32::from_rgb(50, 50, 50),
            stroke_width: 10.0,
            id: format!("knob_{}", label_str.to_lowercase().replace(" ", "_")),
        }
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value.clamp(self.min, self.max);
        self
    }

    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self.value = self.value.clamp(min, max);
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn arc_color(mut self, color: Color32) -> Self {
        self.arc_color = color;
        self
    }

    pub fn track_color(mut self, color: Color32) -> Self {
        self.track_color = color;
        self
    }

    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Convert value to angle in radians (-π to π, where 0 is at top/12 o'clock)
    fn value_to_angle(&self) -> f32 {
        let normalized = (self.value - self.min) / (self.max - self.min);
        // Start at top (12 o'clock = -π/2) and go clockwise
        -std::f32::consts::PI / 2.0 + normalized * std::f32::consts::PI * 1.8
    }

    fn draw_arc(&self, painter: &Painter, center: Pos2, start_angle: f32, end_angle: f32, color: Color32, stroke: Stroke) {
        let outer_radius = self.radius;
        let inner_radius = self.radius - stroke.width;
        
        // Create arc path
        let mut points = Vec::new();
        
        // Number of segments for smooth arc
        let segments = 100;
        for i in 0..=segments {
            let angle = start_angle + (end_angle - start_angle) * (i as f32 / segments as f32);
            let x_outer = center.x + outer_radius * angle.cos();
            let y_outer = center.y + outer_radius * angle.sin();
            let x_inner = center.x + inner_radius * angle.cos();
            let y_inner = center.y + inner_radius * angle.sin();
            
            if i == 0 {
                points.push(Pos2::new(x_outer, y_outer));
            }
            points.push(Pos2::new(x_inner, y_inner));
            
            if i == segments {
                points.push(Pos2::new(x_outer, y_outer));
            }
        }
        
        // Close the path
        if points.len() > 2 {
            points.push(points[0]);
        }
        
        if points.len() >= 4 {
            let shape = PathShape::convex_polygon(points, color, Stroke::NONE);
            painter.add(shape);
        }
    }
}

impl Widget for Knob {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = egui::Vec2::splat(self.radius * 2.0 + self.stroke_width * 2.0);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());
        
        let center = rect.center();
        let painter = ui.painter();
        
        // Draw outer track (full circle background)
        let track_outer = self.radius + self.stroke_width / 2.0;
        let track_inner = self.radius - self.stroke_width / 2.0;
        
        // Draw full circular track
        painter.add(CircleShape {
            center,
            radius: track_outer,
            fill: self.track_color,
            stroke: Stroke::NONE,
        });
        
        painter.add(CircleShape {
            center,
            radius: track_inner,
            fill: Color32::TRANSPARENT,
            stroke: Stroke::NONE,
        });
        
        // Draw active arc (gradient effect using multiple segments)
        let start_angle = -std::f32::consts::PI / 2.0; // 12 o'clock
        let end_angle = self.value_to_angle();
        
        // Create gradient effect by drawing multiple arcs with varying colors
        let segments = 20;
        for i in 0..segments {
            let segment_start = start_angle + (end_angle - start_angle) * (i as f32 / segments as f32);
            let segment_end = start_angle + (end_angle - start_angle) * ((i + 1) as f32 / segments as f32);
            
            // Interpolate color from track color to arc color
            let ratio = i as f32 / segments as f32;
            let r = self.track_color.r() as f32 + (self.arc_color.r() as f32 - self.track_color.r() as f32) * ratio;
            let g = self.track_color.g() as f32 + (self.arc_color.g() as f32 - self.track_color.g() as f32) * ratio;
            let b = self.track_color.b() as f32 + (self.arc_color.b() as f32 - self.track_color.b() as f32) * ratio;
            let segment_color = Color32::from_rgb(r as u8, g as u8, b as u8);
            
            self.draw_arc(
                painter,
                center,
                segment_start,
                segment_end,
                segment_color,
                Stroke::new(self.stroke_width, Color32::TRANSPARENT),
            );
        }
        
        // Draw center circle (knob cap)
        let cap_radius = self.radius * 0.3;
        painter.add(CircleShape {
            center,
            radius: cap_radius,
            fill: Color32::from_rgb(80, 80, 80),
            stroke: Stroke::new(2.0_f32, Color32::WHITE),
        });
        
        // Draw value indicator line
        let indicator_length = self.radius * 0.6;
        let angle = self.value_to_angle();
        let indicator_end = Pos2::new(
            center.x + indicator_length * angle.cos(),
            center.y + indicator_length * angle.sin(),
        );
        
        painter.line_segment(
            [center, indicator_end],
            Stroke::new(4.0_f32, Color32::WHITE),
        );
        
        // Draw label
        let label_pos = Pos2::new(center.x, center.y + self.radius + 20.0);
        painter.text(
            label_pos,
            egui::Align2::CENTER_TOP,
            &self.label,
            egui::FontId::proportional(16.0),
            Color32::WHITE,
        );
        
        // Draw value text
        let value_text = format!("{:.0}", self.value);
        painter.text(
            Pos2::new(center.x, center.y - self.radius - 20.0),
            egui::Align2::CENTER_BOTTOM,
            &value_text,
            egui::FontId::proportional(18.0),
            Color32::WHITE,
        );
        
        // Handle drag interaction - store value in response
        let mut new_value = self.value;
        if response.dragged() {
            if let Some(pos) = response.interact_pointer_pos() {
                let angle = (pos.y - center.y).atan2(pos.x - center.x);
                // Convert angle to value (0 at top, clockwise)
                let normalized = (angle + std::f32::consts::PI / 2.0) / (std::f32::consts::PI * 1.8);
                new_value = self.min + normalized.clamp(0.0, 1.0) * (self.max - self.min);
            }
        }
        
        // Return response with updated value
        response.on_hover_text_at_pointer(format!("Value: {:.1}", new_value))
            .with_drag_value(new_value)
    }
}

/// Extension trait for Response to get drag value
pub trait DragValueExt {
    fn with_drag_value(self, value: f32) -> Self;
    fn get_drag_value(&self) -> Option<f32>;
}

impl DragValueExt for Response {
    fn with_drag_value(mut self, value: f32) -> Self {
        // Store value in a custom way - we'll use the widget info
        // This is a workaround since egui Response doesn't have a data field
        self
    }
    
    fn get_drag_value(&self) -> Option<f32> {
        // Try to extract value from widget info
        // This is a simplified approach - in practice, you'd need to track state externally
        None
    }
}
