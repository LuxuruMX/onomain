# Onomain

Biblioteca Rust para crear palabras inventadas reproducibles a partir de una
o varias semillas. Puede llamarse desde Rust, C#, C u otros lenguajes que
puedan cargar una biblioteca C. La [investigación lingüística](docs/INVESTIGACION.md)
explica las fuentes y las simplificaciones.

## Probar en la terminal

```sh
cargo run -- 12345 12
cargo run -- 12345 12 --perfil --detalle
cargo run -- 12345 12 --semilla 67890 --json
```

`12345` es un número de 64 bits sin signo: funciona como **identificador
reproducible** de una familia. No representa la palabra «bosque» ni tiene
un significado oculto. El número decide estilo (`grave`, `suave`, `fluido`
o `cortante`), longitud habitual, inventario de sonidos, frecuencia de
codas y diptongos, motivos recurrentes y acento sugerido. `--perfil`
muestra esas decisiones; `--detalle` separa las sílabas y pone en mayúsculas
la tónica, por ejemplo `gra-TO-na`. Con `--json --detalle` se devuelven
objetos con `texto`, `silabas` y el índice `tonica` desde cero. Varias
semillas se combinan en orden y producen otra familia.

Semillas numéricamente próximas no producen familias necesariamente
parecidas: `12345` y `12346` son dos identidades independientes.

## API Rust

```rust
use onomain::Generador;

let generador = Generador::desde_numero(12345);
let perfil = generador.perfil();
let palabras = generador.generar(12)?;
let detalladas = generador.generar_detalladas(12)?;
assert_eq!(palabras.len(), 12);
assert_eq!(palabras[0], detalladas[0].texto);
# Ok::<(), onomain::Error>(())
```

Para varias semillas use `Generador::desde_numeros(&[12345, 67890])`.
La API previa de texto (`Generador::nuevo`) sigue disponible para proyectos
que ya la usaban, pero la terminal ahora exige semillas numéricas.

La salida usa únicamente `a`–`z`. Se garantiza que en un lote ninguna
pareja está a una o dos inserciones, sustituciones o eliminaciones de
distancia de Levenshtein. La misma configuración produce el mismo prefijo
de resultados en esta versión. Los lotes aceptan de 0 a 10000 palabras;
si el espacio de búsqueda se agota, se devuelve un error explícito. Los
lotes de varios miles pueden tardar bastante por la comprobación global
de semejanza.

## C y C#

```sh
cargo build --release
LD_LIBRARY_PATH="$PWD/target/release" dotnet run --project examples/csharp/OnomainEjemplo.csproj
```

En Linux, el archivo compilado es `target/release/libonomain.so`. En macOS
sería `libonomain.dylib` y en Windows `onomain.dll`. La cabecera
[include/onomain.h](include/onomain.h) documenta el ABI C. La función
principal `onomain_generar_numero(semilla, cantidad)` devuelve un arreglo
JSON UTF-8; `onomain_describir_numero` expone el perfil y
`onomain_generar_numero_detalle` incluye el acento. Ante `NULL`, el llamador
consulta `onomain_ultimo_error` en el mismo hilo y
libera cada resultado con `onomain_liberar`. El ejemplo C# muestra el manejo
de memoria y la deserialización.

## Diseño y límites

`src/inventory.rs` contiene los inventarios; `style_base.rs` agrupa los
límites de cada estilo; `profile.rs` concreta el perfil desde la semilla;
`generator.rs` combina sílabas; `similarity.rs` mantiene un índice de
distancia; `rng.rs` fija la reproducibilidad; `ffi.rs` y `ffi_numeric.rs`
exponen el ABI C; `main.rs` permite probar desde la terminal.
No hay dependencias externas.

El parecido de una familia se basa hoy en sonidos, longitud y motivos
recurrentes. No hay significados, raíces morfológicas ni sintaxis. Algunas
palabras pueden coincidir con vocablos existentes o resultar incómodas en
algún idioma: no se consultan diccionarios ni listas de exclusión. `j`,
`ch`, `x` y otros grafemas pueden leerse de formas distintas entre lectores.
El alfabeto ASCII de la palabra simple sacrifica tono, longitud y ciertos
sonidos. La salida detallada comunica **acento silábico sugerido**, no
entonación real ni altura de voz. Una
versión nueva del algoritmo podría cambiar la secuencia de nombres de una
semilla: guarde las palabras aprobadas si necesita persistencia.

## Próximos cambios que más valor aportarían

1. Perfiles configurables con inventario, reglas de posición, longitud y
   pesos propios; hoy los cuatro estilos son fijos y el control creativo es
   limitado.
2. Familias morfológicas opcionales con raíces y patrones o afijos. Eso
   crearía parentesco reconocible sin recurrir a diferencias mínimas.
3. Frecuencias medidas sobre corpus por lengua y un evaluador humano de
   pronunciabilidad. Los pesos actuales son heurísticos, no estadísticas.
4. Filtro configurable de palabras existentes, marcas, insultos y términos
   no deseados para el público objetivo.
5. Una capa fonémica interna con romanización configurable. La escritura
   ASCII actual no define una pronunciación única.
