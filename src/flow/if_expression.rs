/*
 * Los `Control flow` son estructuras que determinan que código se ejecutara dependiendo de si la
 * condición es `true` y correr código  repetidamente mientras (while) la condición sea `true`
 */
fn main() {
    /*
     * Una expresión `if` permite bifurcar el código dependiendo de la condición que evalua, 
     * "Si esta condicion se cumple ejecuta este código, si no se cumple ejecuta este otro código"
     */
    const MAX_NUMBER: i32 = 100;
    let number_to = 99;
    if MAX_NUMBER > number_to { // -> `if expression` evalua una operación buleana
        println!("Number to evaluate is less than limit {MAX_NUMBER}");
        println!("The number is {number_to}");
    } else {
        println!("Greater than!");
    }
    /*
     * Si el bloque `if` no tiene el bloque `else if` o `else` y la condición es evaluada como `false`
     * simplemente el programa se saltará el bloque `if`
     */
    if number_to < 0 {
        println!("This part of the code is never show");
    }
    /*
     * Si el bloque `if` contiene más de un caso a evaluar `Rust` va a ejecutar el bloque para la
     * primera condición donde se evalue como `true`
     */
    if number_to % 2 == 0 && number_to < MAX_NUMBER {
        println!("Is odd and less than {}", MAX_NUMBER);
    } else if number_to % 1 == 0 && number_to < MAX_NUMBER {
        println!("Is even and less than {}", MAX_NUMBER);
    } else if number_to < MAX_NUMBER {
        println!("Is zero");
    } else {
        println!("Number out of limit")
    }
    
    /*
     * Como `if` es una `expression` se puede asignar a un `statement` siempre y cuando la última
     * línea sea una `expresión implícita de retorno`
     */
    let number: &str = if number_to > MAX_NUMBER {
        "Is greater than"
    } else {
        /*
         * Cada `arm` o bifurcación del bloque `if` debe de tener una `expresión implícita de retorno`
         * del mismo tipo
         */
        "Is less than"
    };

    println!("{}", number);
}