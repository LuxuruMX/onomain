use crate::Error;
use crate::profile::{Estilo, Perfil};
use crate::profile_summary::PerfilResumen;
use crate::rng::{Rng, hash_numeros, hash_seeds};
use crate::similarity::Indice;
use crate::word::PalabraDetallada;

/// Una familia estable de palabras inventadas.
pub struct Generador {
    hash: u64,
    perfil: Perfil,
}

impl Generador {
    /// La semilla numérica fija todo el perfil de la familia.
    pub fn desde_numero(semilla: u64) -> Self {
        let hash = hash_numeros(&[semilla]);
        Self {
            hash,
            perfil: Perfil::nuevo(hash, Estilo::Automatico),
        }
    }

    /// Combina varias semillas numéricas en orden.
    pub fn desde_numeros(semillas: &[u64]) -> Result<Self, Error> {
        if semillas.is_empty() {
            return Err(Error::SinSemillas);
        }
        let hash = hash_numeros(semillas);
        Ok(Self {
            hash,
            perfil: Perfil::nuevo(hash, Estilo::Automatico),
        })
    }

    /// El orden y los bytes UTF-8 de las semillas forman parte de la identidad.
    pub fn nuevo(semillas: &[&str], estilo: Estilo) -> Result<Self, Error> {
        if semillas.is_empty() {
            return Err(Error::SinSemillas);
        }
        if semillas.iter().any(|s| s.trim().is_empty()) {
            return Err(Error::SemillaVacia);
        }
        let hash = hash_seeds(semillas);
        Ok(Self {
            hash,
            perfil: Perfil::nuevo(hash, estilo),
        })
    }

    pub fn estilo(&self) -> Estilo {
        self.perfil.estilo
    }

    pub fn perfil(&self) -> PerfilResumen {
        PerfilResumen::from(&self.perfil)
    }

    /// Devuelve exactamente `cantidad` palabras o un error explícito.
    /// Cada par del lote tiene distancia de Levenshtein de al menos 3.
    pub fn generar(&self, cantidad: usize) -> Result<Vec<String>, Error> {
        self.generar_detalladas(cantidad)
            .map(|palabras| palabras.into_iter().map(|p| p.texto).collect())
    }

    /// Incluye la silabificación y el acento que eligió la semilla.
    pub fn generar_detalladas(&self, cantidad: usize) -> Result<Vec<PalabraDetallada>, Error> {
        if cantidad > 10_000 {
            return Err(Error::CantidadExcesiva);
        }
        let mut rng = Rng::new(self.hash ^ (self.perfil.estilo as u64) ^ 0xa076_1d64_78bd_642f);
        let mut indice = Indice::default();
        let mut resultado = Vec::with_capacity(cantidad);
        let mut fallos_seguidos = 0_usize;
        let mut intentos = 0_usize;
        let max_intentos = 100_000 + cantidad * 2_000;

        while resultado.len() < cantidad {
            if intentos >= max_intentos {
                return Err(Error::EspacioAgotado);
            }
            intentos += 1;
            let silabas_extra = (fallos_seguidos / 500).min(3);
            let palabra = self.crear_palabra(&mut rng, silabas_extra);
            if !(3..=32).contains(&palabra.texto.len()) || !indice.acepta(palabra.texto.clone()) {
                fallos_seguidos += 1;
                continue;
            }
            fallos_seguidos = 0;
            resultado.push(palabra);
        }
        Ok(resultado)
    }

    fn crear_palabra(&self, rng: &mut Rng, extra: usize) -> PalabraDetallada {
        let perfil = &self.perfil;
        let total =
            perfil.min_silabas + rng.below(perfil.max_silabas - perfil.min_silabas + 1) + extra;
        let mut silabas = Vec::with_capacity(total);
        let mut coda_anterior = false;
        let mut ultima_vocal = "";

        for posicion in 0..total {
            let primero = posicion == 0;
            let ultimo = posicion + 1 == total;
            let ataque = if !primero && coda_anterior && rng.chance(perfil.ataque_vacio) {
                ""
            } else if rng.chance(if primero {
                perfil.motivo_inicio
            } else {
                perfil.motivo_interno
            }) {
                perfil.motivo
            } else {
                perfil.ataques[rng.below(perfil.ataques.len())]
            };
            let ataque = if coda_anterior && ataque.len() > 1 {
                perfil
                    .ataques
                    .iter()
                    .copied()
                    .find(|a| a.len() == 1)
                    .unwrap_or(ataque)
            } else {
                ataque
            };
            let mut vocal = if rng.chance(perfil.prob_diptongo) {
                perfil.diptongos[rng.below(perfil.diptongos.len())]
            } else {
                perfil.vocales[rng.below(perfil.vocales.len())]
            };
            if vocal == ultima_vocal && rng.chance(75) {
                vocal = perfil.vocales[rng.below(perfil.vocales.len())];
            }
            let mut silaba = String::with_capacity(ataque.len() + vocal.len() + 2);
            silaba.push_str(ataque);
            silaba.push_str(vocal);
            ultima_vocal = vocal;
            coda_anterior = rng.chance(if ultimo {
                perfil.coda_final
            } else {
                perfil.coda_interna
            });
            if coda_anterior {
                let coda = if ultimo && rng.chance(perfil.cierre_prob) {
                    perfil.cierre
                } else {
                    perfil.codas[rng.below(perfil.codas.len())]
                };
                silaba.push_str(coda);
            }
            silabas.push(silaba);
        }
        let texto = silabas.concat();
        PalabraDetallada {
            texto,
            silabas,
            tonica: perfil.acento.indice(total),
        }
    }
}

#[cfg(test)]
#[path = "generator_tests.rs"]
mod tests;
