use std::ffi::c_char;

use crate::ffi::{error, json_palabras, resultado};
use crate::{Error, Generador};

fn codigo_error(error: Error) -> u32 {
    match error {
        Error::SinSemillas => 1,
        Error::CantidadExcesiva => 5,
        _ => 6,
    }
}

/// Genera palabras usando exclusivamente una semilla numérica.
#[unsafe(no_mangle)]
pub extern "C" fn onomain_generar_numero(semilla: u64, cantidad: u32) -> *mut c_char {
    let generador = Generador::desde_numero(semilla);
    match generador.generar(cantidad as usize) {
        Ok(palabras) => resultado(json_palabras(&palabras)),
        Err(e) => error(codigo_error(e)),
    }
}

/// Combina varias semillas numéricas en orden.
///
/// # Safety
/// `semillas` debe apuntar a `numero_semillas` elementos `uint64_t` válidos.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn onomain_generar_numeros(
    semillas: *const u64,
    numero_semillas: usize,
    cantidad: u32,
) -> *mut c_char {
    if semillas.is_null() || numero_semillas == 0 || numero_semillas > 64 {
        return error(1);
    }
    // SAFETY: el contrato garantiza el arreglo de entrada durante la llamada.
    let semillas = unsafe { std::slice::from_raw_parts(semillas, numero_semillas) };
    let generador = match Generador::desde_numeros(semillas) {
        Ok(g) => g,
        Err(e) => return error(codigo_error(e)),
    };
    match generador.generar(cantidad as usize) {
        Ok(palabras) => resultado(json_palabras(&palabras)),
        Err(e) => error(codigo_error(e)),
    }
}

/// Devuelve el perfil JSON derivado de una semilla numérica.
#[unsafe(no_mangle)]
pub extern "C" fn onomain_describir_numero(semilla: u64) -> *mut c_char {
    let perfil = Generador::desde_numero(semilla).perfil();
    resultado(format!(
        "{{\"estilo\":\"{}\",\"silabas_min\":{},\"silabas_max\":{},\"acento\":\"{}\",\"ataques\":[\"{}\"],\"vocales\":[\"{}\"],\"diptongos\":[\"{}\"],\"codas\":[\"{}\"],\"motivo\":\"{}\",\"cierre\":\"{}\",\"prob_coda_final\":{},\"prob_diptongo\":{}}}",
        perfil.estilo.nombre(),
        perfil.silabas_min,
        perfil.silabas_max,
        perfil.acento.nombre(),
        perfil.ataques.join("\",\""),
        perfil.vocales.join("\",\""),
        perfil.diptongos.join("\",\""),
        perfil.codas.join("\",\""),
        perfil.motivo,
        perfil.cierre,
        perfil.prob_coda_final,
        perfil.prob_diptongo
    ))
}

/// Devuelve palabras con sílabas y posición tónica en JSON.
#[unsafe(no_mangle)]
pub extern "C" fn onomain_generar_numero_detalle(semilla: u64, cantidad: u32) -> *mut c_char {
    let palabras = match Generador::desde_numero(semilla).generar_detalladas(cantidad as usize) {
        Ok(palabras) => palabras,
        Err(e) => return error(codigo_error(e)),
    };
    let entradas: Vec<String> = palabras
        .iter()
        .map(|p| {
            format!(
                "{{\"texto\":\"{}\",\"silabas\":[\"{}\"],\"tonica\":{}}}",
                p.texto,
                p.silabas.join("\",\""),
                p.tonica
            )
        })
        .collect();
    resultado(format!("[{}]", entradas.join(",")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ffi::onomain_liberar;
    use std::ffi::CStr;

    #[test]
    fn abi_numerico_entrega_json_y_error() {
        let ptr = onomain_generar_numero_detalle(12345, 3);
        assert!(!ptr.is_null());
        let json = unsafe { CStr::from_ptr(ptr) }.to_str().unwrap();
        assert!(json.contains("\"tonica\":"));
        unsafe { onomain_liberar(ptr) };
        assert!(onomain_generar_numero(12345, 10_001).is_null());
        assert_eq!(crate::ffi::onomain_ultimo_error(), 5);
    }
}
