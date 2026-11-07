# Base lingüística de Onomain

La unidad de trabajo es una **secuencia gráfica que representa un sonido o
un grupo de sonidos**, no siempre una letra aislada. `tl`, `ch`, `tz` y `sh`
pueden representar un fonema; `gr` representa normalmente dos y funciona
como un ataque completo en el generador.
La sílaba se aproxima como ataque + núcleo vocálico + coda. Las posiciones
importan: un grupo permitido al principio puede estar prohibido al final.
La gramática y la morfología afectan las familias de palabras, pero esta
primera versión solo genera formas léxicas, sin significados ni flexiones.

## Frecuencia y límites de las generalizaciones

[WALS 12A](https://wals.info/feature/12A) clasifica 486 lenguas de su muestra:
61 con estructura simple, 274 moderadamente compleja y 151 compleja. Son
**lenguas**, no porcentajes de sílabas en el habla. [El capítulo de WALS](https://wals.info/chapter/12)
señala que CV aparece en todas las lenguas estudiadas y que las estructuras
máximas de una lengua pueden ser infrecuentes en su vocabulario. La
frecuencia de una sílaba concreta depende del idioma, dialecto, corpus,
registro y de si se cuentan tipos de palabra o apariciones. No existe una
lista universal honesta de «las sílabas más usadas».

Sí hay datos locales útiles: en [náhuatl](https://mexico.sil.org/language_culture/aztec/syllables-stress-in-nahuatl)
CV es preferida, CVC también es abundante y V/VC son escasas. En
[ruso](https://www.cambridge.org/core/journals/applied-psycholinguistics/article/internal-structure-of-the-syllable-in-russian-and-in-hebrew-evidence-from-monolingual-kindergarteners/45B440E8E540DDE11005B975D1D3DBE9),
un estudio resume que CV y CCV superan dos tercios de los tipos silábicos
según las fuentes que cita. Estas observaciones no se trasladan como
probabilidades universales a Onomain; se usan como inspiración para que
las sílabas sencillas dominen y las complejas den carácter ocasional.

Un ejemplo con recuentos comparables: en un [corpus de habla leída en bengalí](https://www.isca-archive.org/sltu_2010/alam10_sltu.pdf),
CV representa 59,02 % de las sílabas, CVC 27,40 %, V 6,31 %, VC 3,10 % y
CCV 1,99 %. En ese mismo corpus, CCVCCC aparece 4 veces y CVVC una sola;
varias formas complejas proceden de préstamos ingleses. Son recuentos de
**ese corpus**, no de todo el bengalí. En un [estudio de habla infantil en
español](https://loquens.revistas.csic.es/index.php/loquens/article/view/103),
las 25 sílabas más frecuentes tienen como máximo dos fonemas y en su
mayoría son CV. En cambio, un [corpus dialectal alemán de monosílabos](https://www.cambridge.org/core/journals/journal-of-germanic-linguistics/article/syllable-structure-spatially-distributed-patterns-of-monosyllables-in-german-dialects/C7F7EF6975110F750AFA42EAC76960EE)
encuentra CVC como patrón más frecuente y agrupaciones de tres consonantes
o más como raras. La unidad de muestreo cambia la respuesta.

Para **sílabas concretas**, el [inventario C-ORAL-ROM de castellano hablado](https://aclanthology.org/L08-1523.pdf)
contó 558 982 apariciones. Sus cinco formas más frecuentes fueron `/a/`
(4,94 %), `/ke/` (3,77 %), `/de/` (3,51 %), `/es/` (2,45 %) e `/i/`
(2,34 %). En un corpus escrito comparable, las cinco primeras fueron
`/de/`, `/a/`, `/la/`, `/ta/` y `/ke/`. El artículo solo publica las 25
primeras, así que no permite afirmar cuáles son las sílabas concretas menos
frecuentes. En el corpus oral, las primeras 100 formas cubren 78,7 % de
todas las apariciones: las formas raras forman una cola extensa. Estas
frecuencias no son pesos aplicados por defecto a los idiomas inventados.

## Rasgos examinados

| Lengua o familia | Evidencia útil | Decisión de diseño |
| --- | --- | --- |
| [Náhuatl](https://mexico.sil.org/language_culture/aztec/syllables-stress-in-nahuatl) | Patrón (C)V(C), preferencia CV, acento generalmente penúltimo; `tl`, `tz` y `ch` pueden ser grafías de un fonema. | `tl` y `tz` entran como unidades en `cortante`, incluso al final. No se intenta copiar su acento ni su gramática. |
| [Alemán](https://www.austriaca.at/0xc1aa5572_0x003d0eda.pdf) | Agrupaciones consonánticas y codas complejas; parte de las agrupaciones se explica por límites de morfemas. | `grave` permite ataques `gr`, `kr`, `tr` y algunas codas. No se equiparan todas las consonantes dobles con un «golpe». |
| [Alemán, `ch` y glotis](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/D74393072AA2D66032B8A6E58BE15082/S0025100300004084a.pdf/div-class-title-german-div.pdf) | El cierre glotal puede preceder a raíces que empiezan en vocal; el grafema `ch` tiene realizaciones distintas según el entorno. | Un cierre glotal no se inventa mediante una pareja de consonantes. El ASCII actual no lo representa. |
| [Ruso](https://www.cambridge.org/core/journals/journal-of-the-international-phonetic-association/article/russian/55589EC639ADEF1764B5ECD0B76970FA) | La oposición entre consonantes palatalizadas y no palatalizadas es central. | Grupos como `vl` son una opción creativa, pero no definen por sí solos al ruso. La palatalización real queda pendiente. |
| [Georgiano](https://www.cambridge.org/core/journals/phonology/article/abs/georgian-harmonic-clusters-phonetic-cues-to-phonological-representation/E701A9EF18D3B6AD06A3EC017F9904DF) | Admite grupos extensos y agrupaciones llamadas armónicas. | Se toma la idea de agrupaciones fuertes, pero se limita su longitud para mantener legibilidad. |
| [Japonés](https://www.cambridge.org/core/journals/phonology/article/moraic-reversal-and-realisation-analysis-of-a-japanese-language-game/083B743CFC770D79886F97F39F2ED67E) | La mora (C)V y las moras nasales, geminadas o largas participan en el ritmo. | `fluido` favorece secuencias abiertas; no afirma modelar prosodia japonesa. |
| [Hawaiano](https://wals.info/values/12A-haw) | Estructura silábica simple; un [estudio fonético](https://www.cambridge.org/core/journals/journal-of-the-international-phonetic-association/article/hawaiian/9F2B300BCF6EE97EA7E4437C5FCECB57) documenta también contraste glotal y vocales largas. | Inspiración para pocas codas. ASCII simple pierde glotis y longitud. |
| [Yorùbá](https://www.cambridge.org/core/journals/canadian-journal-of-linguistics-revue-canadienne-de-linguistique/article/emerging-grammars-in-contemporary-yoruba-phonology/5FBC31A5B98304BD5F4F398569D75854) | En el estrato tradicional predominan V y CV sin coda; el tono distingue palabras. | `fluido` favorece sílabas abiertas. No se simula tono con letras sin marcas. |
| [Mandarín](https://www.cambridge.org/core/journals/journal-of-the-international-phonetic-association/article/tianjin-mandarin/6BF886FF2DF9DAC72D34116888623AE3) | Un análisis de la variedad de Tianjin usa (C)(G)V(C); el tono y las combinaciones de inicial/final importan. | La idea de restricciones de posición es útil; no se genera pinyin ni tonos. |
| [Turco](https://www.cambridge.org/core/journals/phonology/article/an-exceptionfiltering-approach-to-phonotactic-learning/58AECB02B7F59B91D4D4C974383F41B2) | Armonía anterior/posterior y de redondeamiento, con excepciones. | La armonía vocálica sería una regla opcional futura; mezclar vocales sin regla no equivale a turco. |
| [Finés](https://www.cambridge.org/core/journals/phonology/article/an-algorithm-for-learning-phonological-classes-from-distributional-similarity/F6F352349A2EBF7F8CF789D75F54576A) | Vocales anteriores y posteriores armonizan; `i` y `e` son transparentes en el análisis citado. | Propone una manera concreta de implementar armonía futura sin bloquear todas las vocales mixtas. |
| [Árabe](https://www.cambridge.org/core/journals/journal-of-child-language/article/distribution-of-nominal-wordpatterns-and-roots-in-palestinian-arabic-a-developmental-perspective-in-early-childhood/6DC0B3818B20779004A2D9E1C014A44D) | Raíces consonánticas y patrones vocálicos crean familias morfológicas. | Una futura relación «raíz + patrón» daría parentesco más fuerte que compartir un inventario de sonidos. |
| [Galés](https://www.cambridge.org/core/journals/journal-of-the-international-phonetic-association/article/northern-welsh/196DCA35257D55166D33EA66FF372DC1) | Mutaciones de consonante inicial dependen de contexto morfosintáctico. | No se agregan mutaciones al azar: se necesitaría generar frases o categorías gramaticales. |
| [Español](https://www.cambridge.org/core/journals/journal-of-child-language/article/singleton-consonant-onset-acquisition-in-monolingual-granada-spanishspeaking-preschoolers-with-typical-versus-protracted-phonological-development-impacts-of-word-structure-and-feature-constraints/279D543046B6141807B7282FC1132B6C) | Ataques con líquida, frecuente acento penúltimo y codas sujetas a variación. | Sirve de referencia de pronunciabilidad para lectores hispanohablantes, sin imponer sus reglas. |
| [Bengalí](https://www.isca-archive.org/sltu_2010/alam10_sltu.pdf) | En un corpus leído, CV y CVC suman 86,42 %; grupos largos son raros y algunos son préstamos. | Da un ejemplo cuantitativo de por qué conviene dosificar estructuras marcadas. |

## Fonética, fonotáctica y gramática

**Fonética** describe cómo se articulan y perciben los sonidos: lugar,
modo, sonoridad, vibración de las cuerdas vocales y duración. Una misma
grafía latina puede representar sonidos distintos. `ch` en alemán,
español y francés no designa la misma realización; ni `j` es universalmente
«suave». Las etiquetas de Onomain son impresiones estéticas para lectores
y requieren una futura capa de pronunciación explícita.

**Fonotáctica** decide qué sonidos se permiten juntos y dónde. Se modela
mediante ataques, núcleos y codas, además de reglas entre sílabas. Una
secuencia de tres letras como `tla` puede contener un ataque de un sonido
(`tl`) y una vocal; contar caracteres como consonantes llevaría a un análisis
erróneo del náhuatl. Una prohibición de grupos iniciales no implica una
prohibición de consonantes contiguas entre sílabas.

**Prosodia** reúne acento, tono, longitud y ritmo. El acento penúltimo del
náhuatl, la mora japonesa y el tono yorùbá son fenómenos distintos. El
alfabeto ASCII sin diacríticos no codifica esas oposiciones. **Morfología**
explica parentesco entre formas: las raíces y patrones árabes y las
mutaciones galesas dependen de funciones gramaticales. Generar variantes
ortográficas sin esas funciones sería una imitación superficial.

## Traducción a un generador inventado

El perfil de cada semilla fija un subconjunto de ataques, vocales, diptongos y
codas. Un motivo inicial y un cierre recurren con cierta probabilidad. Las
sílabas abiertas son comunes; los diptongos y codas se dosifican. `grave`
prioriza más sílabas y grupos consonánticos; `suave` permite palabras cortas;
`fluido` favorece CV; `cortante` admite `tl`, `tz` y `ch` en varias posiciones.
Las mezclas son estética inventada, no reconstrucción lingüística.

«Grave» aquí describe una **impresión** por grupos y duración gráfica. No
equivale a frecuencia fundamental baja, tono léxico ni acento prosódico.
Sin audio o marcas prosódicas, la biblioteca no puede garantizar que alguien
pronuncie una palabra con voz grave o suave.
