/*
 * El concepto de prestamo `borrowing` en `Rust` nos permite referirnos a un valor sin tener que
 * tomar la propiedad de el.
 * Una referencia permite que multiples partes del código puedan acceder al recurso sin la necesidad
 * de tener la propiedad
 */
fn run() {
    let greeting = String::from("Hi!");
    let (g_simple, transform) = takes_and_back_ownwership(greeting); // -> El valor de `greeting` se mueve dentro de la función
    /*
     * Con el retorno de valores podemos tener nuevamente la propiedad del valor de `Hi!` asignada
     * a una nueva variable
     */
    println!("Say {}", g_simple);
    println!("Length: {}", transform);
    /*
     * En lugar de "mover" el valor, puedes prestar una referencia al mismo, permitiendo que sea
     * utilizado temporalmente sin perder la propiedad.
     * Como la sintaxis `&g_simple` nos permite crear una referencia que hace referencia al recurso de
     * `g_simple` pero no le pertenece, el recurso al que apunta no será eliminado cuando salga del
     * `scope`
     */
    as_params_ref(&g_simple);
    /*
     * Es decir que aquí sigue siendo una variable valida `g_simple` que apunta al recurso `Hi!`,
     * porque el propietario del recurso núnca cambio
     */
    println!("Say {}", g_simple);
    /*
     * Trasladamos la propiedad a una nueva variable y la hacemos mutable
     */
    let mut g_simple_mut = g_simple;
    /*
     * Creamos una variable de tipo `&mut String` que contendrá una referencia mutable al recurso `Hi!`
     */
    let greeting_ref: &mut String = &mut g_simple_mut;
    
    as_mut_param_ref(greeting_ref);
    //as_mut_param_ref(&mut g_simple_mut);
    println!("Ref: {}", greeting_ref);

    let hello = dangle();
    println!("Not danling val: {}", hello)

}

/*
 * Debido a como `Rust` trabaja con el concepto de transferencia de propiedad `move`, ahora la 
 * variable `val` es propietaria del recurso `Hi!`
 */
fn takes_and_back_ownwership(val: String) -> (String, usize) {
    let len = val.len();
    (val, len)
}


/*
 * Ahora la variable `s` tiene acceso al recurso (solo lectura) `Hi!`, pero sin tener la propiedad
 * del recurso. La variable `s` es una referencia a `s1` que apunta al recurso `Hi!`
 * 
 *                   STACK                                     HEAP
 *       s                           s1
 *  +---------+                  +---------+                  +-------+
 *  |   ptr   | ---------------> |   ptr   | ---------------> | 0 | H |
 *  +---------+                  +---------+                  | 1 | i |
 *                               | len | 3 |                  | 2 | ! |
 *                               +---------+                  +-------+
 *                               | cap | 3 |                  
 *                               +---------+                  
 * 
 */
fn as_params_ref(s: &String) {
    println!("Original greeting: {}", s);
    /*
     * La variable `s` sale del `scope`, pero no tiene la propiedad del recurso `Hi!` al que 
     * hace referencia, por lo tanto, no se libera el recurso, nada pasa
     */
}

/*
 * Además de tener una referencia de solo lectura, el segundo tipo que existe en `Rust` es la
 * referencia mutable, que permite cambiar el valor de un recurso si tener la propiedad
 */
fn as_mut_param_ref(val: &mut String) {
    val.push_str(" Yael");
}

/*
 * No es valido retornar un valor perstado `borrowed value` sin especificar un `lifetime`, se da
 * así porque se esta retornando una referencia a un recurso que ya no va a seguir existiendo
 * 
 *  * fn dangle() -> &String {
 */
fn dangle() -> String {
    let s = String::from("Value");
    /*
     * Por lo tanto se devuelve directamente el valor, ERROR: el tipo de retorno de esta función
     * contiene un valor prestado, pero no hay ningún valor del que tomarlo prestado
     * 
     *  * &s
     */
    s
    /*
     * `s` sale del scope, por lo tanto, es dropeado y la memoria se libera, ya no existe un valor
     * al cual se pueda apuntar y estamos tratando de devolver esa referencia, eso quiere decir
     * que `s` esta apuntando a un `String` invalido
     */
}
