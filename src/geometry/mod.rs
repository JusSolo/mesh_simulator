//! ============================================================================
//! geometry
//!
//! Geometrías utilizadas para representar el mundo.
//!
//! Este módulo es completamente independiente de:
//!
//! - World
//! - Neighborhood
//! - Simulation
//!
//! Únicamente describe cómo dibujar cada celda.
//!
//! ============================================================================

//---------------------------------------------------------------------
// Módulos
//---------------------------------------------------------------------

pub mod shape;

pub mod polygon;

pub mod genpolygon;

pub mod transform;

pub mod cell_geometry;

pub mod geometry;

//---------------------------------------------------------------------
// Reexportaciones
//---------------------------------------------------------------------

pub use shape::Shape;

pub use polygon::Polygon;

pub use genpolygon::GenPolygon;

pub use transform::Transform;

pub use cell_geometry::CellGeometry;

pub use geometry::{CellIndex, Geometry};
