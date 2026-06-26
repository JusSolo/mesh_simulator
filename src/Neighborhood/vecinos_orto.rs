//! ============================================================================
//! vecinos_orto.rs
//!
//! Vecindades para el retículo ortogonal.
//!
//! ============================================================================

use super::neighborhood::{CellIndex, Neighborhood};

/// Filtra las coordenadas fuera del dominio.

////////////////////////////////////////////////////////////////////////////////
// Moore (8 vecinos)
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, Default)]
pub struct VecinosMoore;

impl Neighborhood for VecinosMoore {
    fn name(&self) -> &'static str {
        "VecinosMoore"
    }

    fn neighbors(&self, i: usize, j: usize, L: usize, C: usize) -> Vec<CellIndex> {
        let candidatos = [
            (i as isize - 1, j as isize - 1),
            (i as isize - 1, j as isize),
            (i as isize - 1, j as isize + 1),
            (i as isize, j as isize - 1),
            (i as isize, j as isize + 1),
            (i as isize + 1, j as isize - 1),
            (i as isize + 1, j as isize),
            (i as isize + 1, j as isize + 1),
        ];

        filter_neighbors(&candidatos, L, C)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Von Neumann (4 vecinos)
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, Default)]
pub struct VecinosVonNeumann;

impl Neighborhood for VecinosVonNeumann {
    fn name(&self) -> &'static str {
        "VecinosVonNeumann"
    }

    fn neighbors(&self, i: usize, j: usize, L: usize, C: usize) -> Vec<CellIndex> {
        let candidatos = [
            (i as isize - 1, j as isize),
            (i as isize + 1, j as isize),
            (i as isize, j as isize - 1),
            (i as isize, j as isize + 1),
        ];

        filter_neighbors(&candidatos, L, C)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Diagonal (4 vecinos)
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, Default)]
pub struct VecinosDiagonal;

impl Neighborhood for VecinosDiagonal {
    fn name(&self) -> &'static str {
        "VecinosDiagonal"
    }

    fn neighbors(&self, i: usize, j: usize, L: usize, C: usize) -> Vec<CellIndex> {
        let candidatos = [
            (i as isize - 1, j as isize - 1),
            (i as isize - 1, j as isize + 1),
            (i as isize + 1, j as isize - 1),
            (i as isize + 1, j as isize + 1),
        ];

        filter_neighbors(&candidatos, L, C)
    }
}
