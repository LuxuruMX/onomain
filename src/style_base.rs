use crate::inventory::*;
use crate::profile::Estilo;

/// Límites iniciales antes de que la semilla concrete el perfil.
pub(crate) struct BasePerfil {
    pub ataques: &'static [&'static str],
    pub diptongos: &'static [&'static str],
    pub codas: &'static [&'static str],
    pub min_silabas: usize,
    pub max_silabas: usize,
    pub coda_interna: usize,
    pub coda_final: usize,
    pub ataque_vacio: usize,
    pub prob_diptongo: usize,
}

impl BasePerfil {
    pub fn para(estilo: Estilo) -> Self {
        let (
            ataques,
            diptongos,
            codas,
            min_silabas,
            max_silabas,
            coda_interna,
            coda_final,
            ataque_vacio,
            prob_diptongo,
        ) = match estilo {
            Estilo::Grave => (
                &GRAVES[..],
                &DIPTONGOS_GRAVES[..],
                &CODAS_GRAVES[..],
                3,
                4,
                17,
                48,
                0,
                8,
            ),
            Estilo::Suave => (
                &SUAVES[..],
                &DIPTONGOS_SUAVES[..],
                &CODAS_SUAVES[..],
                2,
                3,
                2,
                8,
                8,
                18,
            ),
            Estilo::Fluido => (
                &FLUIDOS[..],
                &DIPTONGOS_FLUIDOS[..],
                &CODAS_FLUIDAS[..],
                2,
                4,
                6,
                18,
                6,
                14,
            ),
            Estilo::Cortante => (
                &CORTANTES[..],
                &DIPTONGOS_CORTANTES[..],
                &CODAS_CORTANTES[..],
                2,
                4,
                15,
                40,
                0,
                8,
            ),
            Estilo::Automatico => unreachable!(),
        };
        Self {
            ataques,
            diptongos,
            codas,
            min_silabas,
            max_silabas,
            coda_interna,
            coda_final,
            ataque_vacio,
            prob_diptongo,
        }
    }
}
