//! ============================================================================
//! right_panel.rs
//!
//! Panel derecho.
//!
//! Contiene:
//!     - Controles de simulación.
//!     - Selector de retículo.
//!     - Información de la celda.
//!     - Estadísticas.
//!
//! ============================================================================

use eframe::egui::{Slider, Ui};

use super::app::AutomataLab;

/// Dibuja el panel derecho.
pub fn show(
    ui: &mut Ui,
    app: &mut AutomataLab,
) {

    //======================================================================
    // Controles
    //======================================================================

    ui.group(|ui| {

        ui.heading("Controles");

        ui.separator();

        ui.horizontal_centered(|ui| {

            //----------------------------------------------------------
            // Paso atrás
            //----------------------------------------------------------

            ui.add_enabled(
                false,
                eframe::egui::Button::new("⏮"),
            );

            //----------------------------------------------------------
            // Play / Pause
            //----------------------------------------------------------

            if app.running() {

                if ui.button("⏸").clicked() {

                    app.set_running(false);

                }

            } else {

                if ui.button("▶").clicked() {

                    app.set_running(true);

                }

            }

            //----------------------------------------------------------
            // Paso adelante
            //----------------------------------------------------------

            if ui.button("⏭").clicked() {

                // TODO
                // Ejecutar una generación.

            }

        });

        ui.add_space(5.0);

        ui.horizontal_centered(|ui| {

            if ui.button("↺ Reiniciar").clicked() {

                // TODO

            }

        });

        ui.separator();

        ui.label("Velocidad");

        let mut speed = app.speed();

        ui.add(

            Slider::new(
                &mut speed,
                0.1..=100.0,
            )
            .text("pasos/s")

        );

        app.set_speed(speed);

        ui.label(format!("{:.1} pasos/s", speed));

    });

    ui.add_space(10.0);

    //======================================================================
    // Retículo
    //======================================================================

    ui.group(|ui| {

        ui.heading("Retículo");

        ui.separator();

        ui.radio_value(
            &mut app.selected_lattice,
            0,
            "Ortogonal",
        );

        ui.radio_value(
            &mut app.selected_lattice,
            1,
            "Hexagonal",
        );

        ui.radio_value(
            &mut app.selected_lattice,
            2,
            "Triangular",
        );

    });

    ui.add_space(10.0);

    //======================================================================
    // Información
    //======================================================================

    ui.group(|ui| {

        ui.heading("Información de la celda");

        ui.separator();

        ui.label("Posición : (-,-)");
        ui.label("Estado   : -");
        ui.label("Vecinos  : -");

    });

    ui.add_space(10.0);

    //======================================================================
    // Estadísticas
    //======================================================================

    ui.group(|ui| {

        ui.heading("Estadísticas");

        ui.separator();

        ui.label("Generación : 0");
        ui.label("Población  : 0");
        ui.label("Densidad   : 0.0 %");
        ui.label("FPS        : 0");

    });

}