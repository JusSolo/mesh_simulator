//! ============================================================================
//! vecinosh.rs
//!
//! Vecindad hexagonal sobre un retículo ortogonal.
//!
//! Traducción directa de la implementación en Python.
//!
//! ============================================================================

use super::neighborhood::{CellIndex, Neighborhood, filter_neighbors};

/// Vecindad hexagonal.
#[derive(Debug, Clone, Copy, Default)]
pub struct VecinosH;

impl Neighborhood for VecinosH {
    fn name(&self) -> &'static str {
        "VecinosH"
    }

    fn neighbors(&self, i: usize, j: usize, L: usize, C: usize) -> Vec<CellIndex> {
        let candidatos = if i % 2 == 0 {
            [
                (i as isize, j as isize - 1),
                (i as isize - 1, j as isize - 1),
                (i as isize - 1, j as isize),
                (i as isize, j as isize + 1),
                (i as isize + 1, j as isize),
                (i as isize + 1, j as isize - 1),
            ]
        } else {
            [
                (i as isize, j as isize - 1),
                (i as isize - 1, j as isize),
                (i as isize - 1, j as isize + 1),
                (i as isize, j as isize + 1),
                (i as isize + 1, j as isize + 1),
                (i as isize + 1, j as isize),
            ]
        };

        filter_neighbors(&candidatos, L, C)
    }
}
