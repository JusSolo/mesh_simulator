//! ============================================================================
//! geometry.rs
//! ============================================================================

use eframe::egui::{Pos2, Rect};

use super::cell_geometry::CellGeometry;

/// Índice de una celda.
///
/// (fila, columna)
pub type CellIndex = (usize, usize);

/// Interfaz común para cualquier geometría.
pub trait Geometry: Send + Sync {
    fn name(&self) -> &'static str;

    fn cell_geometry(&self, i: usize, j: usize, cell_size: f32) -> CellGeometry;

    fn pick(
        &self,
        mouse: Pos2,
        canvas: Rect,
        L: usize,
        C: usize,
        cell_size: f32,
    ) -> Option<CellIndex>;

    fn canvas_size(&self, L: usize, C: usize, cell_size: f32) -> (f32, f32);
}
