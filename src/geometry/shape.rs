//! ============================================================================
//! shape.rs
//!
//! Define la interfaz común para todas las figuras geométricas utilizadas por
//! el motor gráfico.
//!
//! Una Shape representa únicamente una figura centrada en el origen (0,0).
//! No conoce:
//!
//!     - posición
//!     - escala
//!     - rotación
//!     - color
//!
//! Toda esa información pertenece a Transform.
//!
//! ============================================================================

use eframe::egui::Vec2;

/// Figura geométrica utilizada por el renderer.
///
/// Todas las coordenadas están expresadas en el sistema local de la figura,
/// con el centro ubicado en el origen.
///
/// Normalmente el radio circunscrito será igual a 1.0.
pub trait Shape: Send + Sync {
    /// Nombre de la figura.

    /// Número de vértices.
    fn sides(&self) -> usize;

    /// Devuelve todos los vértices de la figura.
    ///
    /// Los vértices deben estar ordenados en sentido antihorario.
    fn vertices(&self) -> &[Vec2];

    /// Radio circunscrito.
    ///
    /// Es la distancia máxima entre el origen y cualquiera de los vértices.
    ///fn radius(&self) -> f32;
    fn radius(&self) -> f32 {
        self.vertices()
            .iter()
            .map(|v| v.length())
            .fold(0.0, f32::max)
    }

    /// Bounding Box mínimo.
    fn min(&self) -> Vec2 {
        let mut xmin = f32::INFINITY;
        let mut ymin = f32::INFINITY;

        for v in self.vertices() {
            xmin = xmin.min(v.x);
            ymin = ymin.min(v.y);
        }

        Vec2::new(xmin, ymin)
    }

    /// Bounding Box máximo.
    fn max(&self) -> Vec2 {
        let mut xmax = f32::NEG_INFINITY;
        let mut ymax = f32::NEG_INFINITY;

        for v in self.vertices() {
            xmax = xmax.max(v.x);
            ymax = ymax.max(v.y);
        }

        Vec2::new(xmax, ymax)
    }

    /// Anchura del Bounding Box.
    fn width(&self) -> f32 {
        self.max().x - self.min().x
    }

    /// Altura del Bounding Box.
    fn height(&self) -> f32 {
        self.max().y - self.min().y
    }

    /// Devuelve el i-ésimo vértice.
    ///
    /// # Panics
    ///
    /// Produce panic si `index >= sides()`.
    fn vertex(&self, index: usize) -> Vec2 {
        self.vertices()[index]
    }
}
