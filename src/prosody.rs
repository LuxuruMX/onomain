/// Posición de la sílaba tónica sugerida por el perfil.
/// La escritura ASCII de la palabra no marca esta diferencia.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Acento {
    Inicial,
    Penultimo,
    Final,
}

impl Acento {
    pub fn nombre(self) -> &'static str {
        match self {
            Self::Inicial => "inicial",
            Self::Penultimo => "penultimo",
            Self::Final => "final",
        }
    }

    pub(crate) fn indice(self, total: usize) -> usize {
        match self {
            Self::Inicial => 0,
            Self::Penultimo => total.saturating_sub(2),
            Self::Final => total.saturating_sub(1),
        }
    }
}
