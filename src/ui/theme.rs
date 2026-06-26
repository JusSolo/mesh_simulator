//! ============================================================================
//! theme.rs
//!
//! Tema visual de la aplicación.
//!
//! ============================================================================

use eframe::egui::{
    self,
    Color32,
    Context,
    Visuals,
};

/// Aplica el tema del laboratorio.
pub fn apply(ctx: &Context) {

    let mut visuals = Visuals::dark();

    //----------------------------------------------------------------------
    // Colores generales
    //----------------------------------------------------------------------

    visuals.window_fill = Color32::from_rgb(28, 28, 30);

    visuals.panel_fill = Color32::from_rgb(35, 35, 38);

    visuals.faint_bg_color = Color32::from_rgb(45, 45, 48);

    visuals.extreme_bg_color = Color32::from_rgb(20, 20, 20);

    //----------------------------------------------------------------------
    // Selección
    //----------------------------------------------------------------------

    visuals.selection.bg_fill = Color32::from_rgb(70, 110, 180);

    visuals.selection.stroke.color = Color32::WHITE;

    //----------------------------------------------------------------------
    // Hipervínculos
    //----------------------------------------------------------------------

    visuals.hyperlink_color = Color32::from_rgb(120, 180, 255);

    //----------------------------------------------------------------------
    // Aplicar tema
    //----------------------------------------------------------------------

    ctx.set_visuals(visuals);

}