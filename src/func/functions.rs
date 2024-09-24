/*
 * Las funciones son bloques de código con un conjunto de instrucciones que pueden ser ejecutadas.
 * Para `Rust` no es importante en donde se definan las funciones, siempre y cuando esten
 * definidas en algun lugar
 */
fn add(first: i32, second: i32) -> i32 {
    first + second
}

/*
 * La palabra reservada para declarar una nueva función es `fn` segido del nombre de la función
 * `basic_function` con parentesis `()` y las llaves `curly brackets` que le indican al compilador
 * donde comienza el cuerpo de la función y donde termina
 */
fn basic_function() {
    /*
     * El tipo `unit` es un tipo especial que representa la ausencia de un valor significativo y se
     * representa como `()` y no es necesario especificarlo
     */
    ()
}

/*
 * Las funciones pueden declarar parámetros `params` que son variables especiales que forman parte
 * de la "firma" `signature` de la función.
 * `Rust` cuando defines una función, debes especificar los tipos de los parámetros así el compilador
 * sabe exactamente que tipos se manejan
 */
fn is_bigger_than(compare: i32, number: i32) -> bool {
    compare < number
}

/*
 * Las funciones pueden devolver valores después de ser invocadas, el valor de retorno no se "nombra"
 * solo se indica depués de la flecha `->`
 */
fn concat_name(name: &str) -> String {
    if name.len() == 0 {
        /*
         * En `Rust` no necesariamente la última línea del cuerpo de la función es la setencia
         * del valor de retorno, se puede devolver un valor antes usando `return`
         */
        return String::from("Hello");
    }
    let mut g = "Hello ".to_owned();
    g.push_str(name);
    /*
     * En `Rust` se puede indicar la sentencia de retorno omitiendo la palabra `return` y el punto y
     * coma `;`
     */
    g
}

fn run() {
    let res = add(10, 14);
    println!("Result: {res}");
    let compare = 100;
    let number = 101;
    /*
     * El valor de retorno de una función puede ser asignada a una variable
     */
    let is_bigger = is_bigger_than(compare, number);
    println!("Number: {number} is bigger than compare {compare}? R={}", is_bigger);

    basic_function(); // -> ejecución de una función vacia

    /*
     * Los argumentos son valores concretos que son pasados a las funciones en el momento de
     * su llamada, es decir cuando se invoca
     */
    let greeting: String = concat_name("Yael");
    println!("{}", greeting);

}


