//! ============================================================================
//! world.rs
//!
//! Representación del mundo.
//!
//! El mundo se interpreta como una imagen discreta cuyos píxeles contienen
//! estados enteros.
//!
//! Internamente mantiene dos framebuffers:
//!
//!     front : generación actual.
//!     back  : siguiente generación.
//!
//! Las simulaciones leen únicamente del framebuffer frontal y escriben sobre
//! el trasero.
//!
//! ============================================================================

use std::{
    mem,
    ops::{Index, IndexMut},
};

/// Estado de una celda.
pub type State = u8;

/// Mundo del autómata.
#[derive(Clone, Debug)]
pub struct World {
    /// Número de filas.
    L: usize,

    /// Número de columnas.
    C: usize,

    /// Imagen actual.
    front: Vec<State>,

    /// Imagen siguiente.
    back: Vec<State>,
}

impl World {
    //======================================================================
    // Constructor
    //======================================================================

    #[must_use]
    pub fn new(L: usize, C: usize) -> Self {
        let size = L * C;

        Self {
            L,
            C,
            front: vec![0; size],
            back: vec![0; size],
        }
    }

    //======================================================================
    // Información
    //======================================================================

    #[inline]
    pub const fn rows(&self) -> usize {
        self.L
    }

    #[inline]
    pub const fn cols(&self) -> usize {
        self.C
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.front.len()
    }

    //======================================================================
    // Índices
    //======================================================================

    #[inline]
    fn linear_index(&self, i: usize, j: usize) -> usize {
        i * self.C + j
    }

    //======================================================================
    // Front framebuffer
    //======================================================================

    #[inline]
    pub fn state(&self, i: usize, j: usize) -> State {
        self.front[self.linear_index(i, j)]
    }

    #[inline]
    pub fn set_state(&mut self, i: usize, j: usize, state: State) {
        let k = self.linear_index(i, j);
        self.front[k] = state;
    }

    //======================================================================
    // Back framebuffer
    //======================================================================

    #[inline]
    pub fn next(&self, i: usize, j: usize) -> State {
        self.back[self.linear_index(i, j)]
    }

    #[inline]
    pub fn next_mut(&mut self, i: usize, j: usize) -> &mut State {
        let k = self.linear_index(i, j);
        &mut self.back[k]
    }

    #[inline]
    pub fn set_next_state(&mut self, i: usize, j: usize, state: State) {
        *self.next_mut(i, j) = state;
    }

    //======================================================================
    // Utilidades
    //======================================================================

    /// Cambia el estado de una celda de forma cíclica.
    pub fn next_state(&mut self, i: usize, j: usize, states: State) {
        let k = self.linear_index(i, j);

        self.front[k] = (self.front[k] + 1) % states;
    }

    /// Reinicia completamente el framebuffer trasero.
    pub fn clear_next(&mut self) {
        self.back.fill(0);
    }

    /// Intercambia ambos framebuffers.
    pub fn swap_buffers(&mut self) {
        mem::swap(&mut self.front, &mut self.back);
    }

    /// Finaliza una generación.
    ///
    /// Intercambia ambos framebuffers y limpia el nuevo buffer trasero.
    pub fn commit(&mut self) {
        self.swap_buffers();
        self.clear_next();
    }

    //======================================================================
    // Acceso directo a las imágenes
    //======================================================================

    #[inline]
    pub fn image(&self) -> &[State] {
        &self.front
    }

    #[inline]
    pub fn image_mut(&mut self) -> &mut [State] {
        &mut self.front
    }

    #[inline]
    pub fn next_image(&self) -> &[State] {
        &self.back
    }

    #[inline]
    pub fn next_image_mut(&mut self) -> &mut [State] {
        &mut self.back
    }
}

////////////////////////////////////////////////////////////////////////////////
// Index
////////////////////////////////////////////////////////////////////////////////

impl Index<(usize, usize)> for World {
    type Output = State;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j) = index;
        &self.front[self.linear_index(i, j)]
    }
}

impl IndexMut<(usize, usize)> for World {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (i, j) = index;
        let k = self.linear_index(i, j);

        &mut self.front[k]
    }
}
