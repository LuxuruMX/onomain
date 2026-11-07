#ifndef ONOMAIN_H
#define ONOMAIN_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* 0=automatico, 1=grave, 2=suave, 3=fluido, 4=cortante. */
/* Devuelven JSON UTF-8 (por ejemplo ["lave","fano"]) o NULL. */
/* Las cadenas de entrada deben estar terminadas en NUL y ser UTF-8. */
char *onomain_generar(const char *semilla, uint32_t cantidad, uint32_t estilo);
char *onomain_generar_varias(const char *const *semillas, size_t numero_semillas,
                            uint32_t cantidad, uint32_t estilo);

/* 0=éxito, 1=puntero/cantidad de semillas, 2=UTF-8, 3=semilla vacía,
   4=estilo, 5=cantidad mayor que 10000, 6=espacio de generación agotado.
   El código pertenece al hilo que hizo la llamada. */
uint32_t onomain_ultimo_error(void);

/* Liberar exactamente una vez cada resultado no nulo. Acepta NULL. */
void onomain_liberar(char *texto);

#ifdef __cplusplus
}
#endif

#endif
