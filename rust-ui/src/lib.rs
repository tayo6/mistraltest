//! MonoDelay-1 Knobs UI Library
//!
//! This library provides interactive knob widgets with angular gradient arcs
//! for use in audio plugin UIs and other applications.

#![warn(missing_docs)]

mod app;
mod knob;

pub use app::MonoDelayKnobsApp;
pub use knob::Knob;

/// Re-export egui types for convenience
pub use egui::{Color32, Ui};
pub use eframe::App;
