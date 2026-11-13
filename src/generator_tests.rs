use super::*;
use crate::similarity::distancia;

#[test]
fn es_reproducible_y_extensible() {
    let generador = Generador::nuevo(&["bosque", "luna"], Estilo::Suave).unwrap();
    let diez = generador.generar(10).unwrap();
    let veinte = generador.generar(20).unwrap();
    assert_eq!(diez, veinte[..10]);
    assert_ne!(
        diez,
        Generador::nuevo(&["luna", "bosque"], Estilo::Suave)
            .unwrap()
            .generar(10)
            .unwrap()
    );
}

#[test]
fn lote_sin_repeticiones_cercanas_y_solo_ascii() {
    for estilo in [
        Estilo::Grave,
        Estilo::Suave,
        Estilo::Fluido,
        Estilo::Cortante,
    ] {
        let palabras = Generador::nuevo(&["árbol"], estilo)
            .unwrap()
            .generar(100)
            .unwrap();
        assert_eq!(palabras.len(), 100);
        for (i, a) in palabras.iter().enumerate() {
            assert!(a.bytes().all(|b| b.is_ascii_lowercase()));
            for b in &palabras[..i] {
                assert!(distancia(a, b) >= 3, "{a} y {b}");
            }
        }
    }
}

#[test]
fn entradas_invalidas_y_cero() {
    assert!(matches!(
        Generador::nuevo(&[], Estilo::Automatico),
        Err(Error::SinSemillas)
    ));
    assert!(matches!(
        Generador::nuevo(&["  "], Estilo::Suave),
        Err(Error::SemillaVacia)
    ));
    let generador = Generador::nuevo(&["x"], Estilo::Automatico).unwrap();
    assert!(generador.generar(0).unwrap().is_empty());
    assert_eq!(
        generador.generar(10_001).unwrap_err(),
        Error::CantidadExcesiva
    );
}

#[test]
fn numero_define_perfil_y_acento() {
    let uno = Generador::desde_numero(12345);
    let otra_vez = Generador::desde_numeros(&[12345]).unwrap();
    let distinto = Generador::desde_numero(12346);
    assert_eq!(uno.perfil(), otra_vez.perfil());
    assert_ne!(uno.perfil(), distinto.perfil());
    let palabras = uno.generar_detalladas(30).unwrap();
    assert_eq!(
        palabras.iter().map(|p| p.texto.clone()).collect::<Vec<_>>(),
        uno.generar(30).unwrap()
    );
    for palabra in palabras {
        assert_eq!(palabra.texto, palabra.silabas.concat());
        assert!(palabra.tonica < palabra.silabas.len());
        assert_eq!(
            palabra.guia().replace('-', "").to_ascii_lowercase(),
            palabra.texto
        );
    }
}
