//! ============================================================================
//! vecinos_triangular.rs
//!
//! Vecindades para el retículo triangular.
//!
//! Traducción directa de la implementación en Python.
//!
//! VecinosT1 : triángulos que comparten lado.
//! VecinosT2 : triángulos opuestos.
//! VecinosT3 : triángulos que comparten vértice (6 vecinos).
//! VecinosT4 : triángulos que comparten vértice (12 vecinos).
//!
//! ============================================================================

use super::neighborhood::{
    filter_neighbors,
    CellIndex,
    Neighborhood,
};

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
        i: usize,
        j: usize,
        L: usize,
        C: usize,
    ) -> Vec<CellIndex> {

        let mut vecinos = vec![
            (i as isize, j as isize - 1),
            (i as isize, j as isize + 1),
        ];

        if (i as isize - j as isize) % 2 == 0 {
            vecinos.push((i as isize + 1, j as isize));
        } else {
            vecinos.push((i as isize - 1, j as isize));
        }

        filter_neighbors(&vecinos, L, C)

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
        i: usize,
        j: usize,
        L: usize,
        C: usize,
    ) -> Vec<CellIndex> {

        let vecinos = if (i as isize - j as isize) % 2 == 0 {

            [
                (i as isize - 1, j as isize),
                (i as isize + 1, j as isize - 2),
                (i as isize + 1, j as isize + 2),
            ]

        } else {

            [
                (i as isize + 1, j as isize),
                (i as isize - 1, j as isize - 2),
                (i as isize - 1, j as isize + 2),
            ]

        };

        filter_neighbors(&vecinos, L, C)

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
        i: usize,
        j: usize,
        L: usize,
        C: usize,
    ) -> Vec<CellIndex> {

        let vecinos = [

            (i as isize - 1, j as isize - 1),
            (i as isize - 1, j as isize + 1),

            (i as isize, j as isize - 2),
            (i as isize, j as isize + 2),

            (i as isize + 1, j as isize - 1),
            (i as isize + 1, j as isize + 1),

        ];

        filter_neighbors(&vecinos, L, C)

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
        i: usize,
        j: usize,
        L: usize,
        C: usize,
    ) -> Vec<CellIndex> {

        let mut vecinos = vec![

            (i as isize - 1, j as isize - 1),
            (i as isize - 1, j as isize + 1),

            (i as isize, j as isize - 2),
            (i as isize, j as isize + 2),

            (i as isize + 1, j as isize - 1),
            (i as isize + 1, j as isize + 1),

            (i as isize, j as isize - 1),
            (i as isize, j as isize + 1),

            (i as isize - 1, j as isize),
            (i as isize + 1, j as isize),

        ];

        if (i as isize - j as isize) % 2 == 0 {

            vecinos.push((i as isize + 1, j as isize - 2));
            vecinos.push((i as isize + 1, j as isize + 2));

        } else {

            vecinos.push((i as isize - 1, j as isize - 2));
            vecinos.push((i as isize - 1, j as isize + 2));

        }

        filter_neighbors(&vecinos, L, C)

    }

}
