//! ============================================================================
//! app.rs
//!
//! Aplicación principal del laboratorio de autómatas celulares.
//!
//! Esta estructura coordina todos los módulos del programa.
//! La interfaz gráfica se organiza en `layout.rs`.
//!
//! ============================================================================

use super::{layout, theme};
use eframe::{
    egui,
    App,
    Frame,
};


/// Aplicación principal.
pub struct AutomataLab {
    /// ¿La simulación está ejecutándose?
    running: bool,

    /// Velocidad de simulación (pasos por segundo).
    speed: f32,

    /// Retículo seleccionado.
    ///
    /// 0 -> Ortogonal
    /// 1 -> Hexagonal
    /// 2 -> Triangular
    pub selected_lattice: usize,

    /// Configuración del experimento.
    ///
    /// Este texto podrá editarse únicamente cuando la simulación
    /// esté detenida.
    pub configuration_text: String,
}

impl Default for AutomataLab {
    fn default() -> Self {
        Self {
            running: false,

            speed: 10.0,

            selected_lattice: 0,

            configuration_text: String::from(
                r#"#==========================================================
# Configuración del experimento
#==========================================================

# Mundo
width = 100
height = 100

# Geometría
geometry = "Orthogonal"

# Vecindad
neighborhood = "Moore"

# Simulación
simulation = "Conway"

# Parámetros
states = 2
birth = [3]
survival = [2,3]

# Condiciones de frontera
boundary = "Toroidal"

# Semilla
seed = 42
"#,
            ),
        }
    }
}

impl AutomataLab {
    //----------------------------------------------------------------------
    // Running
    //----------------------------------------------------------------------

    #[inline]
    pub fn running(&self) -> bool {
        self.running
    }

    #[inline]
    pub fn set_running(&mut self, value: bool) {
        self.running = value;
    }

    //----------------------------------------------------------------------
    // Velocidad
    //----------------------------------------------------------------------

    #[inline]
    pub fn speed(&self) -> f32 {
        self.speed
    }

    #[inline]
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.max(0.1);
    }
}

impl App for AutomataLab {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        theme::apply(ctx);

        layout::show(ctx, self);
    }
}
