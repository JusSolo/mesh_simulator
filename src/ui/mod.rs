//! ============================================================================
//! ui
//!
//! Interfaz gráfica del laboratorio de autómatas celulares.
//!
//! Este módulo contiene únicamente la presentación.
//!
//! No implementa:
//!
//! - reglas de simulación
//! - vecindades
//! - geometrías
//! - almacenamiento del mundo
//!
//! Toda la lógica matemática vive en los demás módulos.
//!
//! ============================================================================

//---------------------------------------------------------------------
// Módulos
//---------------------------------------------------------------------

pub mod app;

pub mod layout;

pub mod menu; 

pub mod left_panel;

pub mod right_panel;

pub mod viewport;

pub mod bottom_panel;

pub mod theme;


//---------------------------------------------------------------------
// Reexportaciones
//---------------------------------------------------------------------

pub use app::AutomataLab;