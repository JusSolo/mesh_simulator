//! ============================================================================
//! viewport.rs
//!
//! Área central donde se visualizará el mundo.
//!
//! Por el momento únicamente crea un canvas vacío.
//!
//! ============================================================================

use eframe::egui::{Align2, Color32, FontId, Sense, Stroke, Ui};

use super::app::AutomataLab;

/// Dibuja el viewport principal.
pub fn show(ui: &mut Ui, _app: &mut AutomataLab) {
    //----------------------------------------------------------------------
    // Reservar todo el espacio disponible.
    //----------------------------------------------------------------------

    let available = ui.available_size();

    let (response, painter) = ui.allocate_painter(available, Sense::click_and_drag());

    //----------------------------------------------------------------------
    // Fondo
    //----------------------------------------------------------------------

    painter.rect_filled(response.rect, 0.0, Color32::from_rgb(28, 28, 30));

    //----------------------------------------------------------------------
    // Marco
    //----------------------------------------------------------------------

    painter.rect_stroke(response.rect, 0.0, Stroke::new(1.0, Color32::DARK_GRAY));

    //----------------------------------------------------------------------
    // Texto temporal
    //----------------------------------------------------------------------

    painter.text(
        response.rect.center(),
        Align2::CENTER_CENTER,
        "Viewport",
        FontId::proportional(20.0),
        Color32::GRAY,
    );
}
