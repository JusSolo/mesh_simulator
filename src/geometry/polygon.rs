//! ============================================================================
//! polygon.rs
//!
//! Implementación de un polígono regular.
//!
//! Todos los vértices están contenidos en una circunferencia de radio uno y
//! el primer vértice apunta hacia arriba.
//!
//! ============================================================================

use std::f32::consts::{FRAC_PI_2, TAU};

use eframe::egui::Vec2;

use super::shape::Shape;

/// Polígono regular.
#[derive(Debug, Clone)]
pub struct Polygon {
    vertices: Box<[Vec2]>,
}

impl Polygon {
    /// Construye un polígono regular de `sides` lados.
    ///
    /// # Panics
    ///
    /// Produce panic si `sides < 3`.
    #[must_use]
    pub fn new(sides: usize) -> Self {
        assert!(sides >= 3, "Un polígono debe tener al menos tres lados.");

        let vertices: Vec<Vec2> = (0..sides)
            .map(|k| {
                let theta =
                    TAU * (k as f32) / (sides as f32)
                    - FRAC_PI_2;

                Vec2::new(theta.cos(), theta.sin())
            })
            .collect();

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

impl Shape for Polygon {
    fn sides(&self) -> usize {
        self.vertices.len()
    }

    fn vertices(&self) -> &[Vec2] {
        &self.vertices
    }
}