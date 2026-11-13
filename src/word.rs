/// Palabra con separación silábica y acento sugerido.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PalabraDetallada {
    pub texto: String,
    pub silabas: Vec<String>,
    /// Índice de la sílaba tónica, empezando en cero.
    pub tonica: usize,
}

impl PalabraDetallada {
    /// Muestra el acento con mayúsculas: `gra-TO-na`.
    pub fn guia(&self) -> String {
        self.silabas
            .iter()
            .enumerate()
            .map(|(i, silaba)| {
                if i == self.tonica {
                    silaba.to_ascii_uppercase()
                } else {
                    silaba.clone()
                }
            })
            .collect::<Vec<_>>()
            .join("-")
    }
}
