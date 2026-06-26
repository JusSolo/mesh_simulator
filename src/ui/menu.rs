//! ============================================================================
//! menu.rs
//!
//! Barra de menú principal.
//!
//! ============================================================================

use eframe::egui::{menu, Ui};

use super::app::AutomataLab;

/// Dibuja la barra de menú superior.
pub fn show(
    ui: &mut Ui,
    _app: &mut AutomataLab,
) {
    menu::bar(ui, |ui| {

        //------------------------------------------------------------------
        // Archivo
        //------------------------------------------------------------------

        ui.menu_button("Archivo", |ui| {

            if ui.button("Nuevo").clicked() {
                ui.close_menu();
            }

            if ui.button("Abrir").clicked() {
                ui.close_menu();
            }

            if ui.button("Guardar").clicked() {
                ui.close_menu();
            }

            ui.separator();

            if ui.button("Salir").clicked() {
                ui.close_menu();
            }

        });

        //------------------------------------------------------------------
        // Editar
        //------------------------------------------------------------------

        ui.menu_button("Editar", |ui| {

            if ui.button("Deshacer").clicked() {
                ui.close_menu();
            }

            if ui.button("Rehacer").clicked() {
                ui.close_menu();
            }

            ui.separator();

            if ui.button("Limpiar mundo").clicked() {
                ui.close_menu();
            }

            if ui.button("Inicializar aleatoriamente").clicked() {
                ui.close_menu();
            }

        });

        //------------------------------------------------------------------
        // Simulación
        //------------------------------------------------------------------

        ui.menu_button("Simulación", |ui| {

            if ui.button("Play").clicked() {
                ui.close_menu();
            }

            if ui.button("Pause").clicked() {
                ui.close_menu();
            }

            if ui.button("Step").clicked() {
                ui.close_menu();
            }

            if ui.button("Reset").clicked() {
                ui.close_menu();
            }

        });

        //------------------------------------------------------------------
        // Ver
        //------------------------------------------------------------------

        ui.menu_button("Ver", |ui| {

            if ui.button("Centrar mundo").clicked() {
                ui.close_menu();
            }

            if ui.button("Zoom +").clicked() {
                ui.close_menu();
            }

            if ui.button("Zoom -").clicked() {
                ui.close_menu();
            }

        });

        //------------------------------------------------------------------
        // Herramientas
        //------------------------------------------------------------------

        ui.menu_button("Herramientas", |ui| {

            if ui.button("Preferencias").clicked() {
                ui.close_menu();
            }

        });

        //------------------------------------------------------------------
        // Ayuda
        //------------------------------------------------------------------

        ui.menu_button("Ayuda", |ui| {

            if ui.button("Documentación").clicked() {
                ui.close_menu();
            }

            if ui.button("Acerca de...").clicked() {
                ui.close_menu();
            }

        });

    });
}