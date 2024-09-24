/*
 * `Rust` es un lenguaje `expression-based language`, es decir basado en expresiones
 */
fn statements_expressions() {
    /*
     * Un `statement` es una instrucción que realiza una acción pero no devuelve un valor.
     * La mayoría de las líneas que declaran variables o llaman funciones son `statements`
     * 
     *  * let x = y = 5;
     * 
     * No es valido, la asignación de `y = 5` no es una declaración de la variable `y` con `let`
     * 
     *  * let x = (let y = 5);
     * 
     * y la asignación no devuelve el valor de `5`
     */
    let x = 5; // -> `Statement`
    /*
     * La llamada a la función `fn_statement()` es una acción, pero no devuelve ningún valor
     * directamente en el lugar donde se llama
     */
    fn_statement(); // -> `Statement`

    /*
     * Las `expressions` son una instrucción que evalua y devuelve un valor, las `expressions`
     * pueden ser parte de un `statement`
     * 
     *  * 5 + 6 // -> es una expresión que evalua el valor de `11` y lo devuelve
     */
    let res = 5 + 6;
    println!("Result of the expression 5 + 6 {}", res);
    /*
     * Los bloques para crear un nuevo `scope` con `{}` son una `expression`
     */
    let y = {
        let a = 10;
        let x = a + 5;
        x + 1 // -> es el valor devuelto por el bloque
    };
    println!("Value of X: {x} and value of y: {y}");

}

fn fn_statement() {}