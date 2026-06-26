//! ============================================================================
//! left_panel.rs
//!
//! Panel izquierdo de la aplicación.
//!
//! Contiene:
//!   - Lista de simulaciones.
//!   - Configuración de colores (futuro).
//!   - Otros controles secundarios.
//!
//! ============================================================================

use eframe::egui::{ScrollArea, Ui};

use super::app::AutomataLab;

/// Dibuja el panel izquierdo.
pub fn show(ui: &mut Ui, _app: &mut AutomataLab) {
    ScrollArea::vertical().show(ui, |ui| {
        //------------------------------------------------------------------
        // Simulaciones
        //------------------------------------------------------------------

        ui.group(|ui| {
            ui.heading("Simulaciones");

            ui.separator();

            ui.selectable_label(true, "Juego de la Vida");
            ui.selectable_label(false, "Sand Simulation");
            ui.selectable_label(false, "Kristal Krustal");
            ui.selectable_label(false, "Maze");
        });

        ui.add_space(10.0);

        //------------------------------------------------------------------
        // Espacio reservado
        //------------------------------------------------------------------

        ui.group(|ui| {
            ui.heading("Espacio reservado");

            ui.separator();

            ui.label(
                "Aquí aparecerán futuras herramientas\n\
                 o configuraciones de la simulación.",
            );
        });
    });
}
