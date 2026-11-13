//! Generador reproducible de palabras inventadas por familias sonoras.

mod ffi;
mod ffi_numeric;
mod generator;
mod inventory;
mod profile;
mod profile_summary;
mod prosody;
mod rng;
mod similarity;
mod style_base;
mod word;

pub use generator::Generador;
pub use profile::Estilo;
pub use profile_summary::PerfilResumen;
pub use prosody::Acento;
pub use word::PalabraDetallada;

use std::fmt;

/// Errores que pueden aparecer antes o durante la generación.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    SinSemillas,
    SemillaVacia,
    CantidadExcesiva,
    EspacioAgotado,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SinSemillas => write!(f, "se requiere al menos una semilla"),
            Self::SemillaVacia => write!(f, "las semillas no pueden estar vacías"),
            Self::CantidadExcesiva => write!(f, "el máximo es de 10000 palabras por llamada"),
            Self::EspacioAgotado => write!(f, "no se encontraron suficientes palabras distintas"),
        }
    }
}

impl std::error::Error for Error {}
