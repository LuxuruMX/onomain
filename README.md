# Onomain

Biblioteca Rust para crear palabras inventadas reproducibles a partir de una
o varias semillas. Puede llamarse desde Rust, C#, C u otros lenguajes que
puedan cargar una biblioteca C. La [investigación lingüística](docs/INVESTIGACION.md)
explica las fuentes y las simplificaciones.

## Probar en la terminal

```sh
cargo run -- bosque 12 --estilo grave
cargo run -- bosque 12 --estilo suave
cargo run -- bosque 12 --semilla luna --estilo auto --json
```

Los estilos son `auto`, `grave`, `suave`, `fluido` y `cortante`. `auto` elige
uno de los cuatro a partir del hash estable de las semillas. La semilla es
una identidad, no una descripción semántica: `bosque` no implica un estilo
concreto. Las semillas se combinan en orden; `bosque, luna` y `luna, bosque`
son familias distintas.

## API Rust

```rust
use onomain::{Estilo, Generador};

let generador = Generador::nuevo(&["bosque", "luna"], Estilo::Suave)?;
let palabras = generador.generar(12)?;
assert_eq!(palabras.len(), 12);
# Ok::<(), onomain::Error>(())
```

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
[include/onomain.h](include/onomain.h) documenta el ABI C. Las funciones
`onomain_generar` y `onomain_generar_varias` devuelven un arreglo JSON UTF-8
o `NULL`; el llamador consulta `onomain_ultimo_error` en el mismo hilo y
libera cada resultado con `onomain_liberar`. El ejemplo C# muestra el manejo
de memoria y la deserialización.

## Diseño y límites

`src/inventory.rs` contiene los inventarios; `profile.rs` define la
distribución por familia; `generator.rs` combina sílabas; `similarity.rs`
mantiene un índice de distancia; `rng.rs` fija la reproducibilidad;
`ffi.rs` expone el ABI C; `main.rs` permite probar desde la terminal.
No hay dependencias externas.

El parecido de una familia se basa hoy en sonidos, longitud y motivos
recurrentes. No hay significados, raíces morfológicas ni sintaxis. Algunas
palabras pueden coincidir con vocablos existentes o resultar incómodas en
algún idioma: no se consultan diccionarios ni listas de exclusión. `j`,
`ch`, `x` y otros grafemas pueden leerse de formas distintas entre lectores.
El alfabeto ASCII sacrifica tono, acento, longitud y ciertos sonidos. Una
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
