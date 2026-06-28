//! ============================================================================
//! genpolygon.rs
//!
//! Implementación de un polígono arbitrario.
//!
//! ============================================================================

use eframe::egui::Vec2;

use super::shape::Shape;

/// Polígono arbitrario.
#[derive(Debug, Clone)]
pub struct GenPolygon {
    vertices: Box<[Vec2]>,
}

impl GenPolygon {
    /// Construye un polígono arbitrario.
    ///
    /// # Panics
    ///
    /// Produce panic si el número de vértices es menor que tres.
    #[must_use]
    pub fn new(vertices: Vec<Vec2>) -> Self {
        assert!(
            vertices.len() >= 3,
            "Un polígono debe tener al menos tres vértices."
        );

        Self {
            vertices: vertices.into_boxed_slice(),
        }
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }
}

impl Shape for GenPolygon {
    fn sides(&self) -> usize {
        self.vertices.len()
    }

    fn vertices(&self) -> &[Vec2] {
        &self.vertices
    }
}