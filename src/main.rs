//! ============================================================================
//! main.rs
//!
//! Punto de entrada del laboratorio de autómatas celulares.
//!
//! ============================================================================

mod Neighborhood;
mod geometry;
mod simulation;
mod ui;
mod world;

use ui::AutomataLab;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Simulador de Mundos")
            .with_inner_size([1600.0, 900.0])
            .with_min_inner_size([1000.0, 700.0]),

        ..Default::default()
    };

    eframe::run_native(
        "Simulador de Mundos",
        options,
        Box::new(|_cc| Ok(Box::new(AutomataLab::default()))),
    )
}
