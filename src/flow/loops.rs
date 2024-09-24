/*
 * Una estructura de control repetitivas son necesarias cuando se quiere ejecutar un bloque de código
 * varias veces un número finito de veces. Se caracterizan por tener un punto inicial, y una sentencia
 * que cuando no se cumpla indica que debe de terminar
 */
fn loop_flow() {
    /*
     * En `Rust` existe una estructura de control `loop` que repite un bloque de código indefinidamente
     * hasta que se evalue una instrucción que lo detenga explicitamente
     */
    let mut counter = 0;
    loop {
        counter += 1;
        println!("again");
        if counter == 10 {
            break;
        }
    }
    /*
     * `loop` también es una `expression`, por lo tanto, puede devolver un valor y ser asignada
     * a una variable
     */
    counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            /*
             * El uso de `break` para devolver un valor es exclusivo del ciclo `loop`
             */
            break counter + 12;
        }
    }; // -> Los bloques de una `expression` siempre termina en un `;`
    println!("Result: {result}");
    
}

fn while_flow() {
    /*
     * El flujo de control `while` es una estructura repetitiva que siempre se va a ejecutar
     * simpre y cuando la condición sea `true` y terminar cunado sea `false`
     */
    let mut counter = 3;
    /*
     * El bloque `while` evalua la condición antes de ejecutar el bloque de código, por lo tanto,
     * cuando no se cumple no entra al bloque
     */
    println!("Start");
    /*
     * while es una estructura de control y no devuelve un valor. Un bucle while es considerado un
     * `statement` (una declaración) y no una `expression` (una expresión) que devuelva algo.
     */
    while counter != 0 {
        println!("Counting... {}", counter);
        counter -= 1;
    }
    println!("LIFTOFF!!");

}

fn for_flow() {
    let a = [10, 20, 30, 40, 50];
    /*
     * Esta forma de recorrer una colección es valida, pero puede ocasionar una serie de errores
     * como que se acceda a un `index` incorrecto en el `array` causando un `panic` y deteniendo
     * el programa
     */
    let mut index = 0;
    /*
     * También se ralentiza, el compilador añade código de tiempo de ejecución para realizar la
     * comprobación condicional de si el índice está dentro de los límites de la matriz en cada
     * iteración a través del bucle.
     */
    while index < 5 {
        println!("Element: {}", a[index]);
        index += 1;
    }

    /*
     * El bucle `for` en `Rust` se usa principalmente para iterar sobre elementos de una colección
     * (como un array o un vector) o un rango de valores
     */
    for e in a {
        println!("Element: {}", e);
    }

    let numbers = [10, 20, 30];

    for (index, value) in numbers.iter().enumerate() {
        println!("The value at index {} is {}", index, value);
    }

}