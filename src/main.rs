use onomain::{Generador, PalabraDetallada};

fn ayuda() -> &'static str {
    "Uso: onomain NUMERO CANTIDAD [--semilla OTRO_NUMERO] [--perfil] [--detalle] [--json]\nEjemplo: onomain 12345 12 --perfil --detalle"
}

fn numero(valor: &str) -> Result<u64, String> {
    valor.parse::<u64>().map_err(|_| {
        format!(
            "La semilla debe ser un entero entre 0 y {}: {valor}",
            u64::MAX
        )
    })
}

fn imprimir_perfil(generador: &Generador) {
    let perfil = generador.perfil();
    let longitud = if perfil.silabas_min == perfil.silabas_max {
        format!("{} sílabas", perfil.silabas_min)
    } else {
        format!("{}-{} sílabas", perfil.silabas_min, perfil.silabas_max)
    };
    eprintln!(
        "Perfil: {} | {} | acento {} | coda final {}% | diptongo {}%",
        perfil.estilo.nombre(),
        longitud,
        perfil.acento.nombre(),
        perfil.prob_coda_final,
        perfil.prob_diptongo
    );
    eprintln!(
        "Sonidos: ataques [{}] | vocales [{}] | diptongos [{}] | codas [{}]",
        perfil.ataques.join(","),
        perfil.vocales.join(","),
        perfil.diptongos.join(","),
        perfil.codas.join(",")
    );
    eprintln!("Motivo: {} | cierre: {}", perfil.motivo, perfil.cierre);
}

fn imprimir_json(palabras: &[PalabraDetallada], detalle: bool) {
    if detalle {
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
        println!("[{}]", entradas.join(","));
    } else {
        let entradas: Vec<String> = palabras
            .iter()
            .map(|p| format!("\"{}\"", p.texto))
            .collect();
        println!("[{}]", entradas.join(","));
    }
}

fn ejecutar() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let Some(primera) = args.next() else {
        return Err(ayuda().into());
    };
    if primera == "--help" || primera == "-h" {
        println!("{}", ayuda());
        return Ok(());
    }
    let mut semillas = vec![numero(&primera)?];
    let cantidad: usize = args
        .next()
        .ok_or_else(|| ayuda().to_owned())?
        .parse()
        .map_err(|_| "CANTIDAD debe ser un entero no negativo".to_owned())?;
    let mut perfil = false;
    let mut detalle = false;
    let mut json = false;
    while let Some(opcion) = args.next() {
        match opcion.as_str() {
            "--semilla" => {
                semillas.push(numero(&args.next().ok_or("Falta el valor de --semilla")?)?)
            }
            "--perfil" => perfil = true,
            "--detalle" => detalle = true,
            "--json" => json = true,
            _ => return Err(format!("Opción desconocida: {opcion}\n{}", ayuda())),
        }
    }
    let generador = Generador::desde_numeros(&semillas).map_err(|e| e.to_string())?;
    if perfil {
        imprimir_perfil(&generador);
    }
    let palabras = generador
        .generar_detalladas(cantidad)
        .map_err(|e| e.to_string())?;
    if json {
        imprimir_json(&palabras, detalle);
    } else {
        for palabra in palabras {
            if detalle {
                println!("{}\t{}", palabra.texto, palabra.guia());
            } else {
                println!("{}", palabra.texto);
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = ejecutar() {
        eprintln!("{error}");
        std::process::exit(2);
    }
}
