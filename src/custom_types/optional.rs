/*
 * En `Rust` no existe el concepto de `null`, (null es un valor que representa la ausencia de uno)
 * sin embrago, `Rust` si provee el concepto de un valor presente y de uno ausente con la `enum`
 * `Option<T>`
 */
enum ExampleOption<T> {
    Some(T),
    /*
     * La variante `None` representa el concepto de `null` un null es un valor que actualmente no
     * es válido o está ausente por alguna razón.
     */
    None
}

fn main() {
    /*
     * La `enum` `Option<T>` sigue siendo una enumeración normal, su variante con parte genérica
     * `<T>` significa que la variante `Some` del puede contener un dato de cualquier tipo
     */
    let some_number: Option<i32> = Option::Some(5);
    /*
     * Se puede acceder a las variantes de `Option<T>` sin el prefijo `Option::`, y todo es accesible
     * porque se encuentra en el `prelude`
     */
    println!("Some value: {:?}", some_number);
    /*
     * Cuando tenemos un valor `Some`, sabemos que un valor está presente y que el valor se mantiene
     * dentro del `Some`.
     */
    let some_number: Option<i32> = Some(5);
    println!("Some value: {:?}", some_number);
    /*
     * Para la variante `None` es necesario indicar el tipo para el compilador. Cuando tenemos un
     * valor `None`, en cierto sentido significa lo mismo que `null`, no tenemos un valor válido
     */
    let absent_number: Option<i32> = None;
    println!("None value: {:?}", absent_number);
    let value = get_some(10);
    println!("Value: {:?}", value);
}

/*
 * La `enum` `Option<T>` elimina el riesgo de asumir incorrectamente que un valor no-nulo si sea
 * nulo
 */
fn get_some(index: usize) -> Option<u32> {
    let numbers: [u32; 5] = [1, 2, 3, 4, 5];
    if index > numbers.len() {
        None
    } else {
        Some(numbers[index])
    }
}