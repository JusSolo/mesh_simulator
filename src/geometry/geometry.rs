//! ============================================================================
//! geometry.rs
//!
//! Define la interfaz común para todas las geometrías del simulador.
//!
//! Una geometría es responsable únicamente de:
//!
//! - Posicionar las celdas.
//! - Seleccionar el prototipo geométrico correspondiente.
//! - Convertir coordenadas del mouse en coordenadas del mundo.
//!
//! No conoce:
//!
//! - Estados.
//! - Vecindades.
//! - Reglas de simulación.
//!
//! ============================================================================

use std::sync::Arc;

use eframe::egui::{Pos2, Rect};

use super::cell_geometry;

use super::{
    shape::Shape,
    transform::Transform,
};

/// Índice de una celda dentro del mundo.
///
/// (fila, columna)
pub type CellIndex = (usize, usize);

/// Describe completamente cómo debe dibujarse una celda.
#[derive(Clone)]


/// Interfaz común para cualquier geometría.
///
/// La geometría determina:
///
/// - qué figura representa una celda;
/// - dónde está ubicada;
/// - cómo seleccionar una celda mediante el mouse.
///
/// Es completamente independiente de la topología.
pub trait Geometry: Send + Sync {

    /// Nombre mostrado en la interfaz.
    fn name(&self) -> &'static str;

    /// Devuelve la geometría de una celda.
    ///
    /// Esto incluye:
    ///
    /// - figura
    /// - posición
    /// - rotación
    /// - escala
    fn cell_geometry(
        &self,
        row: usize,
        col: usize,
        cell_size: f32,
    ) -> CellGeometry;

    /// Convierte una posición del mouse en una celda.
    ///
    /// Devuelve `None` si el punto está fuera del canvas.
    fn pick(
        &self,
        mouse: Pos2,
        canvas: Rect,
        rows: usize,
        cols: usize,
        cell_size: f32,
    ) -> Option<CellIndex>;

    /// Tamaño recomendado del canvas.
    fn canvas_size(
        &self,
        rows: usize,
        cols: usize,
        cell_size: f32,
    ) -> (f32, f32);

}