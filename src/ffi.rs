use std::cell::Cell;
use std::ffi::{CStr, CString, c_char};

use crate::{Error, Estilo, Generador};

thread_local! {
    static ULTIMO_ERROR: Cell<u32> = const { Cell::new(0) };
}

fn error(codigo: u32) -> *mut c_char {
    ULTIMO_ERROR.set(codigo);
    std::ptr::null_mut()
}

fn estilo_desde_numero(numero: u32) -> Option<Estilo> {
    match numero {
        0 => Some(Estilo::Automatico),
        1 => Some(Estilo::Grave),
        2 => Some(Estilo::Suave),
        3 => Some(Estilo::Fluido),
        4 => Some(Estilo::Cortante),
        _ => None,
    }
}

/// Devuelve JSON UTF-8 asignado en Rust o NULL. Consultar `onomain_ultimo_error`.
///
/// # Safety
/// `semilla` debe apuntar a una cadena C válida y terminada en NUL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn onomain_generar(
    semilla: *const c_char,
    cantidad: u32,
    estilo: u32,
) -> *mut c_char {
    // SAFETY: se pasa la dirección de un puntero local, válido durante la llamada.
    unsafe { onomain_generar_varias(&semilla, 1, cantidad, estilo) }
}

/// Combina semillas en orden. `cantidad` puede ser cero.
///
/// # Safety
/// `semillas` debe apuntar a `numero_semillas` punteros C válidos, cada uno
/// terminado en NUL. La memoria debe seguir viva durante toda la llamada.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn onomain_generar_varias(
    semillas: *const *const c_char,
    numero_semillas: usize,
    cantidad: u32,
    estilo: u32,
) -> *mut c_char {
    if semillas.is_null() || numero_semillas == 0 || numero_semillas > 64 {
        return error(1);
    }
    let Some(estilo) = estilo_desde_numero(estilo) else {
        return error(4);
    };
    if cantidad > 10_000 {
        return error(5);
    }
    // SAFETY: el contrato exige el arreglo y las cadenas válidas.
    let punteros = unsafe { std::slice::from_raw_parts(semillas, numero_semillas) };
    let mut textos = Vec::with_capacity(numero_semillas);
    for &puntero in punteros {
        if puntero.is_null() {
            return error(1);
        }
        // SAFETY: el contrato exige una cadena C terminada en NUL.
        let Ok(texto) = unsafe { CStr::from_ptr(puntero) }.to_str() else {
            return error(2);
        };
        textos.push(texto);
    }
    let generador = match Generador::nuevo(&textos, estilo) {
        Ok(valor) => valor,
        Err(_) => return error(3),
    };
    let palabras = match generador.generar(cantidad as usize) {
        Ok(valor) => valor,
        Err(Error::CantidadExcesiva) => return error(5),
        Err(_) => return error(6),
    };
    let json = format!("[\"{}\"]", palabras.join("\",\""));
    let json = if palabras.is_empty() {
        "[]".to_owned()
    } else {
        json
    };
    ULTIMO_ERROR.set(0);
    CString::new(json)
        .expect("solo se generan letras ASCII")
        .into_raw()
}

/// Código del último error en este hilo: 0=ninguno, 1=puntero/cantidad de
/// semillas, 2=UTF-8, 3=semilla vacía, 4=estilo, 5=cantidad, 6=generación.
#[unsafe(no_mangle)]
pub extern "C" fn onomain_ultimo_error() -> u32 {
    ULTIMO_ERROR.get()
}

/// Libera el JSON devuelto por cualquiera de las funciones `onomain_generar*`.
///
/// # Safety
/// `texto` debe ser NULL o un puntero aún no liberado devuelto por esta biblioteca.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn onomain_liberar(texto: *mut c_char) {
    if !texto.is_null() {
        // SAFETY: el contrato exige un puntero de CString::into_raw.
        drop(unsafe { CString::from_raw(texto) });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cadena_ffi_se_puede_liberar() {
        let semilla = CString::new("noche").unwrap();
        let puntero = unsafe { onomain_generar(semilla.as_ptr(), 5, 1) };
        assert!(!puntero.is_null());
        let texto = unsafe { CStr::from_ptr(puntero) }.to_str().unwrap();
        assert_eq!(texto.matches('"').count(), 10);
        assert_eq!(onomain_ultimo_error(), 0);
        unsafe { onomain_liberar(puntero) };
    }
}
