
# Simulador de caché

En este trabajo práctico se implementará un **simulador de caché parametrizado**. Serán configurables las distintas características de la caché simulada (tamaño, número de sets y asociatividad), y el programa principal leerá y simulará una secuencia de operaciones desde un archivo de trazas.

### Índice

- [Simulador de caché](#simulador-de-caché)
    - [Índice](#índice)
  - [Requisitos formales](#requisitos-formales)
  - [Software](#software)
    - [Lenguaje](#lenguaje)
    - [Makefile](#makefile)
    - [Entrega](#entrega)
  - [Especificaciones](#especificaciones)
    - [Archivos de traza](#archivos-de-traza)
      - [Aclaraciones](#aclaraciones)
    - [Línea de comandos](#línea-de-comandos)
      - [Condiciones de error](#condiciones-de-error)
      - [Precondiciones](#precondiciones)
    - [Parámetros fijos](#parámetros-fijos)
    - [Salida del programa](#salida-del-programa)
    - [Modo verboso](#modo-verboso)
    - [Resumen y estadísticas](#resumen-y-estadísticas)
  - [Operación](#operación)
    - [Métricas](#métricas)
    - [Detalle de casos](#detalle-de-casos)
    - [Ejemplo](#ejemplo)

### Tests
```console
pip install pytest pytest-testdox
```

Ya instalado eso, pueden ejecutar:

```console
./runTester.sh
```

Para chequear memory leaks:

```console
./runValgrind.sh
```

## Especificaciones

En esta sección se especifica en detalle el formato de los archivos de traza que manipula el simulador, así como la interfaz de línea de comandos que el programa debe seguir. Se indica, asimismo, el formato **exacto** de salida que se espera del programa.


### Archivos de traza

Todas las simulaciones se realizarán sobre archivos de trazas de accesos a memoria, en el formato que se especifica a continuación. Cada uno de esos archivos enumera una serie de operaciones e indica, para cada una de ellas, tanto el tipo de la operación (lectura o escritura) como la direccion de memoria sobre la que se realiza.

Como dato interesante, los archivos de traza proporcionados corresponden a ejecuciones reales de varios programas, con lo que al simularlos bajo distintas configuraciones, los resultados son indicativos de cómo afecta la configuración de la caché a programas existentes.

**Cada archivo de trazas contiene un número de líneas _N,_ cada una de las cuales representa un acceso a memoria.** Un ejemplo de archivo de traza sería:

```
0xb7fc7489: W 0xbff20468 4 0xb7fc748e
0xb7fc748e: R 0xbff20468 4 0xb7fc748e
0xb7fc7495: W 0xbff20478 4 0xbff204b0
0xb7fc749e: R 0xb7fd9ff4 4 0x15f24
```

**Los cinco campos de cada línea representan:**

1.  El primer número en hexadecimal es el _instruction pointer_, esto es, la ubicación en memoria de la instrucción que está siendo ejecutada.

2.  Un caracter ASCII indicando si la operación es de lectura: _‘R’;_ o de escritura: _‘W’_.

3.  El siguiente valor en hexadecimal es la dirección de memoria a la que se realiza el acceso.

4.  Un número entero positivo (por ejemplo, 4 u 8) que indica la cantidad de bytes que la instrucción lee, o escribe.

5.  El último valor en hexadecimal corresponde a los datos que se leyeron o escribieron.

El programa simulará estos accesos con una caché del tipo indicado, y reportará las estadísticas correspondientes. Los detalles de la simulación (cálculo de tiempos, etc.) se explican en la sección [Operación](#operación).


#### Aclaraciones

  - Las direcciones de memoria son siempre de 32 bits.

En realidad las trazas usadas corresponden a programas de _benchmarking_ de la colección [MiBench]. Por tanto, si bien son programas “reales”, son del tipo normalmente empleado para medir el rendimiento de una computadora (CPU, memoria y caché), o de su software (compilador y sistema operativo).

[MiBench]: https://ieeexplore.ieee.org/document/990739


### Línea de comandos

La interfaz del programa en la línea de comandos es:

```
$ ./cachesim tracefile.xex C E S [ -v n m ]
```

Los cuatro primeros argumentos son:

1.  el archivo de traza a simular
2.  el tamaño de la caché _C_, en bytes
3.  la asociatividad de la caché, _E_
4.  el número de sets de la caché, _S_

Ejemplo de invocación:

```
$ ./cachesim blowfish.xex 2048 4 8
```

El parámetro `-v` es opcional pero, de estar presente, siempre aparece en quinto lugar, seguido de dos números enteros. Su presencia activa un “modo verboso” en que se imprime información detallada para un subconjunto de las operaciones, tal y como se explica en la sección [Salida del programa](#salida-del-programa).

#### Condiciones de error

El programa debe imprimir un mensaje de error por stderr, y terminar con estado distinto de cero, en cada uno de los casos siguientes:

  - si el número de argumentos no es 4 o 7;
  - si el archivo de trazas especificado no existe;
  - si alguno de los parámetros _C_, _E_ o _S_ no son potencia de dos;
  - si alguna combinación de parámetros de C, E y S es inválida;
  - si los argumentos _n_ y _m_ del modo verboso no son números enteros que cumplan _0 ≤ n ≤ m_

#### Precondiciones

El programa puede asumir que, si el archivo especificado existe, entonces es un archivo de trazas válido, y todas sus líneas respetan el formato especificado anteriormente.


### Parámetros fijos

Hay dos cosas que no se parametrizan en el simulador, que son:

  - la política que desalojo, que es siempre _least-recently used_ (LRU)
  - la _penalty_ por accesos a memoria en el cómputo de tiempos, que es siempre 100 ciclos (ver sección [Detalle de casos](#detalle-de-casos)).


### Salida del programa

La salida del programa sigue siempre el mismo formato y se estructura en dos partes (de estas dos partes, la primera solo se muestra en modo verboso):

1.  **Si se especificó la opción `-v`, se habilita el modo verboso para el subconjunto de operaciones en el rango _[n, m]_** (rango inclusivo). Así, para cada una de ellas se imprimirá una línea con información detallada sobre su resultado, en el formato especificado en la sección [Modo verboso](#modo-verboso).

    Debe considerarse que, en el archivo, las operaciones se enumeran comenzando desde 0. Por tanto, si se especificase por ejemplo `./cachesim … -v 0 9`, se mostraría información detallada para los primeros diez accesos a memoria (que corresponderían a las diez primeras líneas del archivo).

2.  En segundo lugar, **al finalizar la simulación se imprime siempre un resumen de la ejecución y una serie de estadísticas recolectadas**. Esta información se debe imprimir en el orden y formato especificado en la sección [Resumen y estadísticas](#resumen-y-estadísticas).


### Modo verboso

Si se especifica un rango _[n, m]_ para el que mostrar información detallada, para cada operación en el rango se debe imprimir una línea con la siguiente información:

1.  el **índice de la operación** (este campo será _n_ en la primera línea que se imprima, y _m_ en la última)
2.  el **identificador de caso** según lo explicado en la sección [Detalle de casos](#detalle-de-casos), que será uno de los siguientes valores:
    - ‘1’ para _cache hit_
    - ‘2a’ para _clean cache miss_
    - ‘2b’ para _dirty cache miss_
3.  **cache index:** el índice (en hexadecimal) del set correspondiente a la dirección, que será un valor en el rango _[0, S)_.
4.  **cache tag:** el valor (en hexadecimal) del _tag_ correspondiente a la dirección de la operación
5.  **cache line:** número de la línea leída o escrita en el set, que será un valor decimal en el rango _[0, E)_.
6.  **line tag:** el _tag_ presente anteriormente en la línea (mostrar -1 si no había datos válidos)
7.  **valid bit:** 1 o 0 según la línea de caché elegida tuviera previamente datos válidos, o no.
8.  **dirty bit:** 0 o 1 según el bloque estuviera previamente sincronizado con memoria principal, o no.
9.  **last used:** solo para cachés con asociatividad _E > 1_, el índice de la
    operación que usó este bloque por última vez.


### Resumen y estadísticas

Durante la simulación, se deben recolectar ciertas métricas, que serán  mostradas al final de la ejecución en el formato exacto que se muestra abajo. Las métricas necesarias son:

  - número de lecturas _(loads)_
  - número de escrituras _(stores)_
  - número total de accesos _(loads_ + _stores)_
  - número de _misses_ de lectura _(rmiss)_
  - número de _mises_ de escritura _(wmiss)_
  - número total de _misses_ _(rmiss_ + _wmiss)_
  - número de “dirty read misses” y “dirty write misses”
  - cantidad de bytes leídos de memoria _(bytes read)_
  - cantidad de bytes escritos en memoria _(bytes written)_
  - tiempo de acceso acumulado (en ciclos) para todas las lecturas
  - tiempo de acceso acumulado (en ciclos) para todas las escrituras
  - _miss rate_ total

Para la definición exacta de estas métricas, consultar la sección [Métricas](#métricas). Una vez finalice la simulación, se las deberá imprimir en el siguiente formato:

```
2-way, 64 sets, size = 4KB
loads 65672 stores 34328 total 100000
rmiss 679 wmiss 419 total 1098
dirty rmiss 197 dirty wmiss 390
bytes read 17568 bytes written 9392
read time 153272 write time 115228
miss rate 0.010980
```

donde la primera línea es una mera descripción de los parámetros de la caché.


## Operación

En esta sección se explica:

  - para cada métrica, la definición exacta de qué debe medir
  - para cada acceso, los distintos casos que pueden ocurrir, y la penalización
    (en tiempo) de cada uno


### Métricas

Las dos primeras métricas, _loads_ y _stores_, corresponden simplemente al número de operaciones de cada tipo (`R` y `W`), y su suma debe corresponder al total de líneas en el archivo de trazas.

Un “miss de lectura” ocurre ante cualquier operación de tipo `R` que resulte en un acceso a memoria. Un “miss de escritura” es el caso equivalente, pero para operaciones de tipo `W`. **En ambos casos se incrementará el valor de las métricas _bytes read_ y _bytes written_ según corresponda** (¡teniendo en cuenta el tamaño del bloque!).

Las métricas _dirty rmiss_ y _dirty wmiss_ son el subconjunto de _misses_ en que se escribe en memoria, esto es: se remplaza un bloque de la caché, y ese bloque tenía datos que no habían sido enviados aún a memoria principal. (En otras palabras, el _dirty bit_ de la línea remplazada estaba a 1.)

Finalmente, el _miss rate_ total es la división del número total de _misses_
por el número total de accesos. (La contabilización de tiempos de lectura y escritura se describen en la siguiente sección.)


### Detalle de casos

Cada operación en la cache resultará en uno de estos tres casos:

1.  _hit;_
2.  _miss;_ que puede ser:
    1. _clean_
    2. _dirty_

Sea un acceso a la dirección _M_, cuyo set _(cache index)_ resulta ser _i;_ dicho set tiene contiene _E_ líneas, que numeramos de 0 a _E-1_. Entonces:

1.  si hay un _hit_, significa que hay la línea número _k_, con _0 ≤ k < E_, tiene una coincidencia con _M_ en su tag; en ese caso:
    - el tiempo que toma la operación es 1 ciclo (ver sección [Métricas](#métricas))
    - el campo _last-used_ de la línea _k_ se actualiza con el índice de la operación actual (para el mecanismo LRU)
    - si el acceso es de escritura, se pone a 1 el _dirty bit_ del bloque

2.  si se produce un _miss_, se debe elegir una línea _k_ donde alojar el bloque; ésta siempre será: bien la línea no válida de menor índice, bien la línea con menor valor de _last-used_. Entonces puede suceder:
    1.  la línea a desalojar no tiene datos válidos, o bien los tiene pero el _dirty bit_ es 0: _**clean cache miss**_. Se lee el bloque _M_ de memoria y:
        - el tiempo de acceso en ciclos es 1 + _penalty_
        - se actualiza el campo _last-used_
        - si el acceso es de escritura, se pone a 1 el _dirty bit_ del bloque
    2.  la línea a desalojar tiene su _dirty bit_ a 1: _**dirty cache miss**_. Se escribe en memoria el bloque existente y:
        - se lee el bloque _M_ de memoria
        - el tiempo de acceso en ciclos es 1 + 2 × _penalty_
        - se actualiza el campo _last-used_
        - si el acceso es de escritura, se pone a 1 el _dirty bit_ del bloque

### Ejemplo

Dado el archivo de traza [adpcm.xex], se ofrecen muestras de la salida en las
siguientes configuraciones:

  - 2KiB, 2-way, 64 sets:

    ```
    $ ./cachesim adpcm.xex 2048 2 64 -v 0 15000
    ```

    Salida esperada: [adpcm_2048-2-64.txt].

  - 4KiB, direct-mapped, 256 sets:

    ```
    $ ./cachesim adpcm.xex 4096 1 256 -v 0 10000
    ```

    Salida esperada: [adpcm_4096-1-256.txt].

[adpcm.xex]: https://orgacomp.github.io/static/cachesim/trazas/adpcm.xex
[adpcm_2048-2-64.txt]: https://orgacomp.github.io/static/cachesim/output/adpcm_2048-2-64.txt
[adpcm_4096-1-256.txt]: https://orgacomp.github.io/static/cachesim/output/adpcm_4096-1-256.txt
