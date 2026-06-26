//! ============================================================================
//! layout.rs
//!
//! Organización general de la interfaz.
//!
//! Este módulo únicamente distribuye los paneles de la aplicación.
//! Cada panel es responsable de dibujar su propio contenido.
//!
//! ============================================================================

use eframe::egui::{
    CentralPanel,
    Context,
    SidePanel,
    TopBottomPanel,
};

use super::{
    app::AutomataLab,
    bottom_panel,
    left_panel,
    menu,
    right_panel,
    viewport,
};

/// Construye la interfaz principal.
pub fn show(
    ctx: &Context,
    app: &mut AutomataLab,
) {

    //----------------------------------------------------------------------
    // Barra de menú
    //----------------------------------------------------------------------

    TopBottomPanel::top("menu_bar")
        .resizable(false)
        .show(ctx, |ui| {

            menu::show(ui, app);

        });

    //----------------------------------------------------------------------
    // Panel izquierdo
    //----------------------------------------------------------------------

    SidePanel::left("left_panel")
        .default_width(220.0)
        .min_width(180.0)
        .resizable(true)
        .show(ctx, |ui| {

            left_panel::show(ui, app);

        });

    //----------------------------------------------------------------------
    // Panel derecho
    //----------------------------------------------------------------------

    SidePanel::right("right_panel")
        .default_width(260.0)
        .min_width(220.0)
        .resizable(true)
        .show(ctx, |ui| {

            right_panel::show(ui, app);

        });

    //----------------------------------------------------------------------
    // Panel inferior
    //----------------------------------------------------------------------

    TopBottomPanel::bottom("bottom_panel")
        .default_height(180.0)
        .min_height(120.0)
        .resizable(true)
        .show_separator_line(true)
        .show(ctx, |ui| {

            bottom_panel::show(ui, app);

        });

    //----------------------------------------------------------------------
    // Viewport
    //----------------------------------------------------------------------

    CentralPanel::default()
        .show(ctx, |ui| {

            viewport::show(ui, app);

        });

}