//! ============================================================================
//! neighborhood.rs
//!
//! Define la interfaz común para todas las vecindades del simulador.
//!
//! Una vecindad determina únicamente qué celdas son consideradas vecinas.
//!
//! Es completamente independiente de:
//!
//! - la geometría
//! - el mundo
//! - la simulación
//!
//! ============================================================================

/// Índice de una celda dentro del mundo.
///
/// (fila, columna)
pub type CellIndex = (usize, usize);

/// Elimina las coordenadas que se encuentran fuera del dominio.
///
/// Actualmente esta función implementa la topología de un plano finito.
///
/// ---------------------------------------------------------------------------
/// FUTURO
/// ---------------------------------------------------------------------------
///
/// Esta función probablemente evolucione para soportar distintas topologías
/// del espacio simulado:
///
/// - Plano.
/// - Cilindro.
/// - Toro.
/// - Esfera.
/// - Cinta de Möbius.
/// - Botella de Klein.
///
/// Es posible que en el futuro reciba un parámetro adicional que describa la
/// topología utilizada. **No implementar esto todavía.**
pub fn filter_neighbors(candidates: &[(isize, isize)], L: usize, C: usize) -> Vec<CellIndex> {
    let mut neighbors = Vec::new();

    for &(i, j) in candidates {
        if i >= 0 && i < L as isize && j >= 0 && j < C as isize {
            neighbors.push((i as usize, j as usize));
        }
    }

    neighbors
}

/// Interfaz común para todas las vecindades.
///
/// La notación utilizada sigue la convención matemática:
///
/// - `i` : índice de fila.
/// - `j` : índice de columna.
/// - `L` : número de filas.
/// - `C` : número de columnas.
pub trait Neighborhood: Send + Sync {
    /// Nombre de la vecindad.
    fn name(&self) -> &'static str;

    /// Devuelve los vecinos de la celda `(i,j)`.
    ///
    /// Los vecinos siempre pertenecen al dominio
    ///
    /// ```text
    /// 0 <= i < L
    /// 0 <= j < C
    /// ```
    fn neighbors(&self, i: usize, j: usize, L: usize, C: usize) -> Vec<CellIndex>;
}
