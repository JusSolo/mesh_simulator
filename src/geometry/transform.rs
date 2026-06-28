//! ============================================================================
//! transform.rs
//!
//! Transformación afín 2D utilizada por el motor gráfico.
//!
//! Una Transform define cómo convertir una figura en coordenadas locales
//! (centrada en el origen) a coordenadas del mundo.
//!
//! El orden de aplicación es:
//!
//!      Escala -> Rotación -> Traslación
//!
//! ============================================================================

use eframe::egui::{Pos2, Vec2};

/// Transformación geométrica 2D.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    /// Posición del centro de la figura.
    pub position: Pos2,

    /// Rotación en radianes.
    pub rotation: f32,

    /// Escala uniforme.
    pub scale: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

impl Transform {
    /// Transformación identidad.
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            position: Pos2::ZERO,
            rotation: 0.0,
            scale: 1.0,
        }
    }

    /// Construye una transformación completa.
    #[must_use]
    pub const fn new(
        position: Pos2,
        rotation: f32,
        scale: f32,
    ) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    /// Cambia únicamente la posición.
    #[must_use]
    pub fn with_position(mut self, position: Pos2) -> Self {
        self.position = position;
        self
    }

    /// Cambia únicamente la rotación.
    #[must_use]
    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    /// Cambia únicamente la escala.
    #[must_use]
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    /// Transforma un punto desde coordenadas locales
    /// hasta coordenadas del mundo.
    #[must_use]
    pub fn transform_point(&self, local: Vec2) -> Pos2 {
        let cos = self.rotation.cos();
        let sin = self.rotation.sin();

        let x = local.x * self.scale;
        let y = local.y * self.scale;

        let xr = x * cos - y * sin;
        let yr = x * sin + y * cos;

        Pos2::new(
            self.position.x + xr,
            self.position.y + yr,
        )
    }

    /// Transforma una lista de vértices.
    #[must_use]
    pub fn transform_into(
        &self,
        source: &[Vec2],
        destination: &mut Vec<Pos2>,
    ) {
        destination.clear();
    
        for &v in source {
        
            destination.push(
                self.transform_point(v)
            );
        
        }
    }
}