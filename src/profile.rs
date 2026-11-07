use crate::inventory::*;
use crate::rng::Rng;

/// Mezclas creativas de rasgos; no reproducen idiomas reales.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Estilo {
    Automatico = 0,
    Grave = 1,
    Suave = 2,
    Fluido = 3,
    Cortante = 4,
}

impl Estilo {
    pub fn desde_nombre(value: &str) -> Option<Self> {
        match value {
            "automatico" | "auto" => Some(Self::Automatico),
            "grave" => Some(Self::Grave),
            "suave" => Some(Self::Suave),
            "fluido" => Some(Self::Fluido),
            "cortante" => Some(Self::Cortante),
            _ => None,
        }
    }

    pub(crate) fn resolver(self, hash: u64) -> Self {
        if self != Self::Automatico {
            return self;
        }
        match hash % 4 {
            0 => Self::Grave,
            1 => Self::Suave,
            2 => Self::Fluido,
            _ => Self::Cortante,
        }
    }
}

pub(crate) struct Perfil {
    pub estilo: Estilo,
    pub ataques: Vec<&'static str>,
    pub vocales: Vec<&'static str>,
    pub diptongos: Vec<&'static str>,
    pub codas: Vec<&'static str>,
    pub motivo: &'static str,
    pub cierre: &'static str,
    pub min_silabas: usize,
    pub max_silabas: usize,
    pub coda_interna: usize,
    pub coda_final: usize,
    pub ataque_vacio: usize,
    pub prob_diptongo: usize,
}

impl Perfil {
    pub fn nuevo(hash: u64, estilo: Estilo) -> Self {
        let estilo = estilo.resolver(hash);
        let mut rng = Rng::new(hash ^ 0xd1b5_4a32_d192_ed03);
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

        let ataques = elegir(ataques, 9, &mut rng);
        let vocales = elegir(&VOCALES, 4, &mut rng);
        let diptongos = elegir(diptongos, 2, &mut rng);
        let codas = elegir(codas, 5, &mut rng);
        let motivo = ataques[rng.below(ataques.len())];
        let cierre = codas[rng.below(codas.len())];
        Self {
            estilo,
            ataques,
            vocales,
            diptongos,
            codas,
            motivo,
            cierre,
            min_silabas,
            max_silabas,
            coda_interna,
            coda_final,
            ataque_vacio,
            prob_diptongo,
        }
    }
}

fn elegir(source: &[&'static str], count: usize, rng: &mut Rng) -> Vec<&'static str> {
    let mut values = source.to_vec();
    for i in 0..count {
        let j = i + rng.below(values.len() - i);
        values.swap(i, j);
    }
    values.truncate(count);
    values
}
