//! ============================================================================
//! simulation.rs
//!
//! Define la interfaz común para todas las simulaciones del laboratorio.
//!
//! Una simulación representa una transformación:
//!
//!     World(t) ---> World(t+1)
//!
//! utilizando una determinada vecindad.
//!
//! ============================================================================

use crate::{
    Neighborhood::Neighborhood,
    world::World,
};

/// Interfaz común para todas las simulaciones.
///
/// Cada simulación conoce:
///
/// - su nombre;
/// - su vecindad;
/// - sus parámetros;
/// - cómo evolucionar el mundo.
///
/// La geometría NO pertenece a la simulación.
pub trait Simulation: Send {

    //----------------------------------------------------------------------
    // Información
    //----------------------------------------------------------------------

    /// Nombre de la simulación.
    fn name(&self) -> &'static str;

    /// Número de estados permitidos.
    ///
    /// Ejemplos:
    ///
    /// Conway        -> 2
    ///
    /// Brian Brain   -> 3
    ///
    /// Potts(q)      -> q
    fn states(&self) -> u8;

    /// Vecindad utilizada por la simulación.
    fn neighborhood(&self) -> &dyn Neighborhood;

    //----------------------------------------------------------------------
    // Simulación
    //----------------------------------------------------------------------

    /// Ejecuta exactamente una generación.
    ///
    /// La simulación debe:
    ///
    /// 1. Leer únicamente del framebuffer frontal.
    /// 2. Escribir únicamente sobre el framebuffer trasero.
    /// 3. Finalizar llamando a `world.commit()`.
    fn step(
        &mut self,
        world: &mut World,
    );

    //----------------------------------------------------------------------
    // Parámetros
    //----------------------------------------------------------------------

    /// Exporta la configuración en formato TOML.
    ///
    /// El texto generado será mostrado en el editor de configuración
    /// de la interfaz.
    fn export_parameters(&self) -> String;

    /// Importa una configuración escrita por el usuario.
    ///
    /// Debe devolver un error descriptivo si la configuración es inválida.
    fn import_parameters(
        &mut self,
        text: &str,
    ) -> Result<(), String>;

    /// Devuelve la configuración por defecto.
    ///
    /// Normalmente simplemente llamará a `export_parameters()`
    /// sobre una simulación recién creada.
    fn default_configuration(&self) -> String {
        self.export_parameters()
    }
}