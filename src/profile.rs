use crate::inventory::*;
use crate::prosody::Acento;
use crate::rng::Rng;
use crate::style_base::BasePerfil;

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
    pub fn nombre(self) -> &'static str {
        match self {
            Self::Automatico => "automatico",
            Self::Grave => "grave",
            Self::Suave => "suave",
            Self::Fluido => "fluido",
            Self::Cortante => "cortante",
        }
    }

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
    pub motivo_inicio: usize,
    pub motivo_interno: usize,
    pub cierre_prob: usize,
    pub acento: Acento,
}

impl Perfil {
    pub fn nuevo(hash: u64, estilo: Estilo) -> Self {
        let estilo = estilo.resolver(hash);
        let mut rng = Rng::new(hash ^ 0xd1b5_4a32_d192_ed03);
        let base = BasePerfil::para(estilo);

        let longitud = rng.below(3);
        let min_silabas = (base.min_silabas - usize::from(longitud == 0)
            + usize::from(longitud == 2))
        .max(match estilo {
            Estilo::Suave => 1,
            Estilo::Grave => 3,
            _ => 2,
        });
        let max_silabas =
            base.max_silabas - usize::from(longitud == 0) + usize::from(longitud == 2);
        let coda_interna = variar(base.coda_interna, 10, &mut rng);
        let coda_final = variar(base.coda_final, 15, &mut rng);
        let prob_diptongo = variar(base.prob_diptongo, 8, &mut rng);
        let ataques = elegir(base.ataques, 7 + rng.below(5), &mut rng);
        let vocales = elegir(&VOCALES, 3 + rng.below(3), &mut rng);
        let diptongos = elegir(base.diptongos, 2, &mut rng);
        let codas = elegir(base.codas, 3 + rng.below(5), &mut rng);
        let motivo = if estilo == Estilo::Suave {
            let simples: Vec<_> = ataques.iter().copied().filter(|a| a.len() == 1).collect();
            simples[rng.below(simples.len())]
        } else {
            ataques[rng.below(ataques.len())]
        };
        let cierre = codas[rng.below(codas.len())];
        let motivo_inicio = 20 + rng.below(31);
        let motivo_interno = 7 + rng.below(20);
        let cierre_prob = 20 + rng.below(41);
        let acento = match rng.below(3) {
            0 => Acento::Inicial,
            1 => Acento::Penultimo,
            _ => Acento::Final,
        };
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
            ataque_vacio: base.ataque_vacio,
            prob_diptongo,
            motivo_inicio,
            motivo_interno,
            cierre_prob,
            acento,
        }
    }
}

fn variar(base: usize, amplitud: usize, rng: &mut Rng) -> usize {
    (base as isize + rng.below(amplitud * 2 + 1) as isize - amplitud as isize).clamp(0, 80) as usize
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
