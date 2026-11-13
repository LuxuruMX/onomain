use crate::profile::{Estilo, Perfil};
use crate::prosody::Acento;

/// Rasgos fijados por una semilla; porcentajes son probabilidades por sílaba.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerfilResumen {
    pub estilo: Estilo,
    pub silabas_min: usize,
    pub silabas_max: usize,
    pub acento: Acento,
    pub ataques: Vec<&'static str>,
    pub vocales: Vec<&'static str>,
    pub diptongos: Vec<&'static str>,
    pub codas: Vec<&'static str>,
    pub motivo: &'static str,
    pub cierre: &'static str,
    pub prob_coda_final: usize,
    pub prob_diptongo: usize,
}

impl From<&Perfil> for PerfilResumen {
    fn from(value: &Perfil) -> Self {
        Self {
            estilo: value.estilo,
            silabas_min: value.min_silabas,
            silabas_max: value.max_silabas,
            acento: value.acento,
            ataques: value.ataques.clone(),
            vocales: value.vocales.clone(),
            diptongos: value.diptongos.clone(),
            codas: value.codas.clone(),
            motivo: value.motivo,
            cierre: value.cierre,
            prob_coda_final: value.coda_final,
            prob_diptongo: value.prob_diptongo,
        }
    }
}
