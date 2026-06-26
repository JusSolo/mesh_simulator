//! ============================================================================
//! bottom_panel.rs
//!
//! Panel inferior.
//!
//! Contiene:
//!     - Configuración del experimento (TOML)
//!     - Estadísticas
//!     - Barra de estado
//!
//! ============================================================================

use eframe::egui::{
    Grid,
    TextEdit,
    Ui,
};

use super::app::AutomataLab;

/// Dibuja el panel inferior.
pub fn show(
    ui: &mut Ui,
    app: &mut AutomataLab,
) {

    //----------------------------------------------------------------------
    // Panel superior
    //----------------------------------------------------------------------

    ui.columns(2, |columns| {

        //------------------------------------------------------------------
        // Configuración
        //------------------------------------------------------------------

        columns[0].group(|ui| {

            ui.heading("Configuración del experimento");

            ui.separator();

            ui.add_enabled(
                !app.running(),
                TextEdit::multiline(
                    &mut app.configuration_text,
                )
                .desired_rows(12)
                .code_editor()
            );

        });

        //------------------------------------------------------------------
        // Estadísticas
        //------------------------------------------------------------------

        columns[1].group(|ui| {

            ui.heading("Estadísticas");

            ui.separator();

            Grid::new("statistics_grid")
                .striped(true)
                .show(ui, |ui| {

                    ui.label("Generación");
                    ui.label("0");
                    ui.end_row();

                    ui.label("Población");
                    ui.label("0");
                    ui.end_row();

                    ui.label("Densidad");
                    ui.label("0.0 %");
                    ui.end_row();

                    ui.label("FPS");
                    ui.label("60");
                    ui.end_row();

                });

        });

    });

    //----------------------------------------------------------------------
    // Barra de estado
    //----------------------------------------------------------------------

    ui.separator();

    ui.horizontal(|ui| {

        ui.label("Mouse : (-,-)");

        ui.separator();

        ui.label("Estado : -");

        ui.separator();

        ui.label("Generación : 0");

        ui.separator();

        if app.running() {

            ui.label("Ejecutando");

        } else {

            ui.label("Pausada");

        }

        ui.separator();

        ui.label("100 × 100");

        ui.separator();

        ui.label("Ortogonal");

    });

}