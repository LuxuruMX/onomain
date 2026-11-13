//! Inventarios gráficos de los cuatro estilos creativos.

pub(crate) const GRAVES: [&str; 16] = [
    "k", "gr", "kr", "tr", "dr", "br", "cr", "g", "t", "r", "sk", "st", "ch", "vr", "kl", "gl",
];
pub(crate) const SUAVES: [&str; 16] = [
    "l", "v", "f", "n", "m", "j", "s", "h", "y", "w", "sh", "fl", "z", "r", "ll", "sl",
];
pub(crate) const FLUIDOS: [&str; 16] = [
    "m", "n", "l", "r", "w", "y", "p", "t", "k", "h", "s", "b", "d", "f", "v", "ch",
];
pub(crate) const CORTANTES: [&str; 16] = [
    "tz", "tl", "ch", "ts", "k", "t", "p", "sk", "st", "kr", "tr", "x", "n", "r", "g", "cl",
];
pub(crate) const VOCALES: [&str; 5] = ["a", "e", "i", "o", "u"];
pub(crate) const DIPTONGOS_GRAVES: [&str; 3] = ["au", "oi", "ai"];
pub(crate) const DIPTONGOS_SUAVES: [&str; 4] = ["au", "eu", "ia", "ei"];
pub(crate) const DIPTONGOS_FLUIDOS: [&str; 4] = ["ai", "oi", "ua", "ie"];
pub(crate) const DIPTONGOS_CORTANTES: [&str; 3] = ["ai", "ua", "au"];
pub(crate) const CODAS_GRAVES: [&str; 10] = ["r", "t", "k", "n", "s", "l", "rt", "sk", "ch", "tl"];
pub(crate) const CODAS_SUAVES: [&str; 7] = ["n", "l", "m", "s", "r", "h", "v"];
pub(crate) const CODAS_FLUIDAS: [&str; 7] = ["n", "m", "l", "r", "s", "ng", "h"];
pub(crate) const CODAS_CORTANTES: [&str; 10] =
    ["tl", "tz", "k", "t", "n", "s", "ch", "x", "r", "ts"];
