//! ============================================================================
//! conway.rs
//!
//! Juego de la Vida de Conway.
//!
//! Esta implementación representa cualquier autómata tipo Life:
//!
//!     B... / S...
//!
//! Conway es simplemente:
//!
//!     B3/S23
//!
//! ============================================================================

use crate::{
    Neighborhood::{Neighborhood, VecinosMoore},
    world::World,
};

use super::simulation::Simulation;

/// Juego de la Vida de Conway.
pub struct Conway {
    /// Vecindad utilizada.
    neighborhood: Box<dyn Neighborhood>,

    /// Número de estados.
    ///
    /// Conway utiliza únicamente dos.
    states: u8,

    /// Reglas de nacimiento.
    birth: Vec<u8>,

    /// Reglas de supervivencia.
    survival: Vec<u8>,
}

impl Default for Conway {
    fn default() -> Self {
        Self {
            neighborhood: Box::new(VecinosMoore),

            states: 2,

            birth: vec![3],

            survival: vec![2, 3],
        }
    }
}

impl Simulation for Conway {
    //----------------------------------------------------------------------
    // Información
    //----------------------------------------------------------------------

    fn name(&self) -> &'static str {
        "Conway"
    }

    fn states(&self) -> u8 {
        self.states
    }

    fn neighborhood(&self) -> &dyn Neighborhood {
        self.neighborhood.as_ref()
    }

    //----------------------------------------------------------------------
    // Simulación
    //----------------------------------------------------------------------

    fn step(&mut self, _world: &mut World) {
        todo!(
            "Se implementará cuando terminemos \
             el registro de vecindades."
        );
    }

    //----------------------------------------------------------------------
    // Configuración
    //----------------------------------------------------------------------

    fn export_parameters(&self) -> String {
        format!(
            r#"#==========================================================
# Conway Game of Life
#==========================================================

simulation = "{}"

states = {}

neighborhood = "{}"

birth = {:?}

survival = {:?}
"#,
            self.name(),
            self.states,
            self.neighborhood.name(),
            self.birth,
            self.survival,
        )
    }

    fn import_parameters(&mut self, _text: &str) -> Result<(), String> {
        todo!("Parser TOML pendiente.");
    }
}
