//! ============================================================================
//! neighborhood
//!
//! Define las distintas vecindades utilizadas por el simulador.
//!
//! Una vecindad determina únicamente qué celdas son consideradas vecinas.
//!
//! Es completamente independiente de:
//!
//! - Geometry
//! - World
//! - Simulation
//!
//! ============================================================================

//---------------------------------------------------------------------
// Módulos
//---------------------------------------------------------------------

pub mod neighborhood;

pub mod vecinos_orto;
pub mod vecinos_triangular;
pub mod vecinosh;

//---------------------------------------------------------------------
// Reexportaciones
//---------------------------------------------------------------------

pub use neighborhood::{
    CellIndex,
    Neighborhood,
};

pub use vecinos_orto::{
    VecinosMoore,
    VecinosVonNeumann,
    VecinosDiagonal,
};

pub use vecinos_triangular::{
    VecinosT1,
    VecinosT2,
    VecinosT3,
    VecinosT4,
};

pub use vecinosh::VecinosH;