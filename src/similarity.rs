/// Índice BK para detectar semejanza global sin comparar cada par del lote.
#[derive(Default)]
pub(crate) struct Indice {
    raiz: Option<Box<Nodo>>,
}

struct Nodo {
    palabra: String,
    hijos: Vec<(usize, Box<Nodo>)>,
}

impl Indice {
    pub fn acepta(&mut self, palabra: String) -> bool {
        match &mut self.raiz {
            None => {
                self.raiz = Some(Box::new(Nodo {
                    palabra,
                    hijos: Vec::new(),
                }));
                true
            }
            Some(raiz) if raiz.cercano(&palabra, 2) => false,
            Some(raiz) => {
                raiz.insertar(palabra);
                true
            }
        }
    }
}

impl Nodo {
    fn cercano(&self, palabra: &str, umbral: usize) -> bool {
        let distancia = distancia(&self.palabra, palabra);
        if distancia <= umbral {
            return true;
        }
        self.hijos.iter().any(|(arista, hijo)| {
            arista.abs_diff(distancia) <= umbral && hijo.cercano(palabra, umbral)
        })
    }

    fn insertar(&mut self, palabra: String) {
        let distancia = distancia(&self.palabra, &palabra);
        for (arista, hijo) in &mut self.hijos {
            if *arista == distancia {
                hijo.insertar(palabra);
                return;
            }
        }
        self.hijos.push((
            distancia,
            Box::new(Self {
                palabra,
                hijos: Vec::new(),
            }),
        ));
    }
}

/// Distancia de Levenshtein sobre bytes; la salida se limita a ASCII.
pub(crate) fn distancia(a: &str, b: &str) -> usize {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let mut anterior: Vec<usize> = (0..=b.len()).collect();
    let mut actual = vec![0; b.len() + 1];
    for (i, &x) in a.iter().enumerate() {
        actual[0] = i + 1;
        for (j, &y) in b.iter().enumerate() {
            actual[j + 1] = (anterior[j + 1] + 1)
                .min(actual[j] + 1)
                .min(anterior[j] + usize::from(x != y));
        }
        std::mem::swap(&mut anterior, &mut actual);
    }
    anterior[b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rechaza_cambios_menores() {
        let mut indice = Indice::default();
        assert!(indice.acepta("krotirret".into()));
        assert!(!indice.acepta("krotiret".into()));
        assert!(!indice.acepta("krotarret".into()));
        assert!(indice.acepta("grastronta".into()));
    }
}
