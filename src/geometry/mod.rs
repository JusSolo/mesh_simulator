//! ============================================================================
//! geometry
//!
//! Módulo encargado de la representación geométrica de los retículos.
//!
//! Este módulo es completamente independiente de:
//!
//! - World
//! - Topology
//! - Simulation
//!
//! Únicamente describe cómo representar gráficamente una celda.
//!
//! ============================================================================

pub mod shape;
pub mod polygon;
pub mod genpolygon;
pub mod transform;
pub mod geometry;
pub mod cell_geometry;

//---------------------------------------------------------------------
// Reexportaciones
//---------------------------------------------------------------------
pub use cell_geometry::CellGeometry;

pub use geometry::{
    CellGeometry,
    CellIndex,
    Geometry,
};

pub use genpolygon::GenPolygon;

pub use polygon::Polygon;

pub use shape::Shape;

pub use transform::Transform;