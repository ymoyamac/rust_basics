/*
 * Los `slices` son una referencia a una parte continua de un fragmento de una colección de datos,
 * sin tener la propiedad sobre los datos siendo todas referencias, es así para asegurar el acceso
 * seguro a los datos
 */
fn slice_type() {
    let hello: &str = "hello world";
    /*
     * Para crear un `slice` se crea con el operador de rango `range operator` `..` o `..=`
     * indicando el comienzo y el final, los valores son posiciones de la collección
     */
    let first_word: &str = &hello[0..5];
    println!("{}", first_word);
    iter_over_slice();
    let numbers: [i32; 5] = [5, 13, 48, 203, 9823];
    /*
     * Operador de rango exclusivo `..` incluye los valores `[13, 48, 203]`, pero excluye
     * el último que esta en la posición `4`
     */
    iter_over_param(&numbers[1..4]);
    let nums: [i32; 10] = [3, 812, 83, 7521, 342, 12, 1245, 341, 853, 7893];
    /*
     * Operador de rango inclusivo `..=` incluye los valores `[812, 83, 7521, 342, 12, 1245]`
     * incluye el último valor
     */
    iter_over_param(&nums[1..=6]);
    /*
     * Operador de rango completo, sin especificar límites, el `slice` toma toda la colleción
     */
    iter_over_param(&nums[..]);
    /*
     * Operador de rango exclusivo, sin especificar límite superior, continua indefinidamente
     * Este tipo de rango es útil cuando no se sabe el valor final o se desea iterar hasta
     * que se cumpla una condición externa.
     */
    iter_over_param(&nums[3..]);

}

fn iter_over_slice() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4]; // -> slice contiene [2, 3, 4]
    for (index, ele) in slice.iter().enumerate() {
        println!("Index: {}, Item: {}", index, ele);
    }
}

/*
 * Los `slice` permiten trabajar con una perte de datos sin depender del tamaño fijo de la colección
 */
fn iter_over_param(slice: &[i32]) {
    for (index, ele) in slice.iter().enumerate() {
        println!("Index: {}, Item: {}", index, ele);
    }
}
