use onomain::{Estilo, Generador};

fn ayuda() -> &'static str {
    "Uso: onomain SEMILLA CANTIDAD [--semilla OTRA] [--estilo auto|grave|suave|fluido|cortante] [--json]\nEjemplo: onomain bosque 12 --semilla luna --estilo suave"
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
    let cantidad: usize = args
        .next()
        .ok_or_else(|| ayuda().to_owned())?
        .parse()
        .map_err(|_| "CANTIDAD debe ser un entero no negativo".to_owned())?;
    let mut semillas = vec![primera];
    let mut estilo = Estilo::Automatico;
    let mut json = false;
    while let Some(opcion) = args.next() {
        match opcion.as_str() {
            "--semilla" => semillas.push(args.next().ok_or("Falta el valor de --semilla")?),
            "--estilo" => {
                let nombre = args.next().ok_or("Falta el valor de --estilo")?;
                estilo = Estilo::desde_nombre(&nombre).ok_or("Estilo desconocido")?;
            }
            "--json" => json = true,
            _ => return Err(format!("Opción desconocida: {opcion}\n{}", ayuda())),
        }
    }
    let referencias: Vec<&str> = semillas.iter().map(String::as_str).collect();
    let palabras = Generador::nuevo(&referencias, estilo)
        .and_then(|g| g.generar(cantidad))
        .map_err(|e| e.to_string())?;
    if json {
        if palabras.is_empty() {
            println!("[]");
        } else {
            println!("[\"{}\"]", palabras.join("\",\""));
        }
    } else {
        for palabra in palabras {
            println!("{palabra}");
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
