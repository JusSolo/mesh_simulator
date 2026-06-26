//! ============================================================================
//! vecinos_triangular.rs
//!
//! Vecindades para el retículo triangular.
//!
//! Traducción directa de la implementación en Python.
//!
//! ============================================================================

use super::neighborhood::{CellIndex, Neighborhood};

/// Filtra los vecinos que se encuentran fuera del mundo.
fn filter_neighbors(
    candidates: &[(isize, isize)],
    rows: usize,
    cols: usize,
) -> Vec<CellIndex> {

    let mut neighbors = Vec::new();

    for &(r, c) in candidates {

        if r >= 0
            && r < rows as isize
            && c >= 0
            && c < cols as isize
        {
            neighbors.push((r as usize, c as usize));
        }

    }

    neighbors
}

////////////////////////////////////////////////////////////////////////////////
// VecinosT1
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, Default)]
pub struct VecinosT1;

impl Neighborhood for VecinosT1 {

    fn name(&self) -> &'static str {
        "VecinosT1"
    }

    fn neighbors(
        &self,
        row: usize,
        col: usize,
        rows: usize,
        cols: usize,
    ) -> Vec<CellIndex> {

        let mut vecinos = vec![
            (row as isize, col as isize - 1),
            (row as isize, col as isize + 1),
        ];

        if (row + col) % 2 == 0 {
            vecinos.push((row as isize + 1, col as isize));
        } else {
            vecinos.push((row as isize - 1, col as isize));
        }

        filter_neighbors(&vecinos, rows, cols)
    }
}

////////////////////////////////////////////////////////////////////////////////
// VecinosT2
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, Default)]
pub struct VecinosT2;

impl Neighborhood for VecinosT2 {

    fn name(&self) -> &'static str {
        "VecinosT2"
    }

    fn neighbors(
        &self,
        row: usize,
        col: usize,
        rows: usize,
        cols: usize,
    ) -> Vec<CellIndex> {

        let vecinos = if (row + col) % 2 == 0 {

            [
                (row as isize - 1, col as isize),
                (row as isize + 1, col as isize - 2),
                (row as isize + 1, col as isize + 2),
            ]

        } else {

            [
                (row as isize + 1, col as isize),
                (row as isize - 1, col as isize - 2),
                (row as isize - 1, col as isize + 2),
            ]

        };

        filter_neighbors(&vecinos, rows, cols)

    }

}

////////////////////////////////////////////////////////////////////////////////
// VecinosT3
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, Default)]
pub struct VecinosT3;

impl Neighborhood for VecinosT3 {

    fn name(&self) -> &'static str {
        "VecinosT3"
    }

    fn neighbors(
        &self,
        row: usize,
        col: usize,
        rows: usize,
        cols: usize,
    ) -> Vec<CellIndex> {

        let vecinos = [

            (row as isize - 1, col as isize - 1),
            (row as isize - 1, col as isize + 1),

            (row as isize, col as isize - 2),
            (row as isize, col as isize + 2),

            (row as isize + 1, col as isize - 1),
            (row as isize + 1, col as isize + 1),

        ];

        filter_neighbors(&vecinos, rows, cols)

    }

}

////////////////////////////////////////////////////////////////////////////////
// VecinosT4
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, Default)]
pub struct VecinosT4;

impl Neighborhood for VecinosT4 {

    fn name(&self) -> &'static str {
        "VecinosT4"
    }

    fn neighbors(
        &self,
        row: usize,
        col: usize,
        rows: usize,
        cols: usize,
    ) -> Vec<CellIndex> {

        let mut vecinos = vec![

            (row as isize - 1, col as isize - 1),
            (row as isize - 1, col as isize + 1),

            (row as isize, col as isize - 2),
            (row as isize, col as isize + 2),

            (row as isize + 1, col as isize - 1),
            (row as isize + 1, col as isize + 1),

            (row as isize, col as isize - 1),
            (row as isize, col as isize + 1),

            (row as isize - 1, col as isize),
            (row as isize + 1, col as isize),

        ];

        if (row + col) % 2 == 0 {

            vecinos.push((row as isize + 1, col as isize - 2));
            vecinos.push((row as isize + 1, col as isize + 2));

        } else {

            vecinos.push((row as isize - 1, col as isize - 2));
            vecinos.push((row as isize - 1, col as isize + 2));

        }

        filter_neighbors(&vecinos, rows, cols)

    }

}