#![allow(dead_code)]

use ratatui::style::Color;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Animation effect types
#[derive(Debug, Clone, Copy)]
pub enum EffectType {
    /// Sweep animation from left to right
    Sweep,
    /// Pulse/breathing effect
    Pulse,
    /// Fade in animation
    FadeIn,
    /// Fade out animation
    FadeOut,
    /// Flash highlight
    Flash,
}

/// Active animation effect
#[derive(Debug, Clone)]
pub struct Effect {
    pub effect_type: EffectType,
    pub start_time: Instant,
    pub duration: Duration,
    pub target_id: String,
}

impl Effect {
    pub fn new(effect_type: EffectType, target_id: String, duration_ms: u64) -> Self {
        Self {
            effect_type,
            start_time: Instant::now(),
            duration: Duration::from_millis(duration_ms),
            target_id,
        }
    }

    /// Get the progress of the effect (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        let elapsed = self.start_time.elapsed();
        (elapsed.as_secs_f64() / self.duration.as_secs_f64()).min(1.0)
    }

    /// Check if the effect has completed
    pub fn is_complete(&self) -> bool {
        self.progress() >= 1.0
    }
}

/// Manages animation effects
#[derive(Default)]
pub struct EffectManager {
    effects: HashMap<String, Vec<Effect>>,
}

impl EffectManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new effect for a target
    pub fn add_effect(&mut self, target_id: &str, effect: Effect) {
        self.effects
            .entry(target_id.to_string())
            .or_default()
            .push(effect);
    }

    /// Trigger file change animation
    pub fn trigger_file_change(&mut self, file_path: &str) {
        // Clear existing effects for this file
        self.effects.remove(file_path);

        // Add combined effects for file content change
        let effects = vec![
            Effect::new(EffectType::Sweep, file_path.to_string(), 300),
            Effect::new(EffectType::Pulse, file_path.to_string(), 500),
            Effect::new(EffectType::Flash, file_path.to_string(), 800),
        ];

        self.effects.insert(file_path.to_string(), effects);
    }

    /// Trigger new file animation
    pub fn trigger_new_file(&mut self, file_path: &str) {
        self.effects.remove(file_path);
        let effect = Effect::new(EffectType::FadeIn, file_path.to_string(), 500);
        self.effects.insert(file_path.to_string(), vec![effect]);
    }

    /// Trigger file deletion animation
    pub fn trigger_file_delete(&mut self, file_path: &str) {
        self.effects.remove(file_path);
        let effect = Effect::new(EffectType::FadeOut, file_path.to_string(), 300);
        self.effects.insert(file_path.to_string(), vec![effect]);
    }

    /// Get active effects for a target
    pub fn get_effects(&self, target_id: &str) -> Option<&Vec<Effect>> {
        self.effects.get(target_id)
    }

    /// Clean up completed effects
    pub fn cleanup(&mut self) {
        self.effects.retain(|_, effects| {
            effects.retain(|e| !e.is_complete());
            !effects.is_empty()
        });
    }

    /// Calculate combined visual state for a target
    pub fn calculate_visual_state(&self, target_id: &str) -> VisualState {
        let mut state = VisualState::default();

        if let Some(effects) = self.effects.get(target_id) {
            for effect in effects {
                let progress = effect.progress();
                match effect.effect_type {
                    EffectType::Sweep => {
                        // Sweep position (0.0 to 1.0)
                        state.sweep_position = Some(progress);
                    }
                    EffectType::Pulse => {
                        // Pulse brightness oscillation
                        let pulse = (progress * std::f64::consts::PI * 4.0).sin();
                        state.brightness = Some((pulse + 1.0) / 2.0 * (1.0 - progress));
                    }
                    EffectType::FadeIn => {
                        state.opacity = Some(progress);
                    }
                    EffectType::FadeOut => {
                        state.opacity = Some(1.0 - progress);
                    }
                    EffectType::Flash => {
                        // Flash decreases over time
                        state.flash_intensity = Some(1.0 - progress);
                    }
                }
            }
        }

        state
    }
}

/// Visual state calculated from active effects
#[derive(Default)]
pub struct VisualState {
    pub sweep_position: Option<f64>,
    pub brightness: Option<f64>,
    pub opacity: Option<f64>,
    pub flash_intensity: Option<f64>,
}

impl VisualState {
    /// Calculate the background color based on the visual state
    pub fn calculate_bg_color(&self, base: Color, highlight: Color) -> Color {
        let mut factor: f64 = 0.0;

        if let Some(brightness) = self.brightness {
            factor = factor.max(brightness);
        }

        if let Some(flash) = self.flash_intensity {
            factor = factor.max(flash * 0.5);
        }

        interpolate_color(base, highlight, factor)
    }
}

/// Interpolate between two RGB colors
fn interpolate_color(from: Color, to: Color, factor: f64) -> Color {
    match (from, to) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            let factor = factor.clamp(0.0, 1.0);
            let r = (r1 as f64 + (r2 as f64 - r1 as f64) * factor) as u8;
            let g = (g1 as f64 + (g2 as f64 - g1 as f64) * factor) as u8;
            let b = (b1 as f64 + (b2 as f64 - b1 as f64) * factor) as u8;
            Color::Rgb(r, g, b)
        }
        _ => to,
    }
}
