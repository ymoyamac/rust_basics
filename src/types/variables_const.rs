/*
 * Las variables son espacios de memoria que permiten almacenar datos con un nombre asociado.
 */
pub fn run() {
    /*
     * Por defecto las variables son `inmutables` para garantizar la seguridad de memoria y
     * la concurrencia sin datos compartidos, para evitar errores comunes como `data races`
     * (condiciones de carrera), problemas de sincronización y estados inconsistentes.
     */
    let immutable_variable = 10;
    println!("Immutable variable has the value: {}", immutable_variable);
    /*
     * No se puede reasignar el valor nuevamente a la variable después de su primera asignación,
     * a menos que se indique con la palabra `mut`, en algunas ocaciones será más conveniente
     * utilizar la mutabilidad
     */
    let mut mutable_variable = 9;
    println!("First value: {}", mutable_variable);
    mutable_variable = mutable_variable + 1;
    println!("Second value: {}", mutable_variable);
    /*
     * Las constantes en `Rust` siempre serán `inmutables` y su valor se debe de conocer en
     * tiempo de compilación, se declaran con la palabra clave `const` especificar el tipo de dato,
     * en mayúsculas y usando `snacke_case`
     */
    const DIMENTIONS: u8 = 3;
    /*
     * Tienen un ciclo de vida más amplio que las variables, ya que se almacenan en la memoria
     * de solo lectura.
     */
    println!("The number of dimentions is {} and Max points are {} units", DIMENTIONS, MAX_POINTS);
}

/*
 * Las constantes pueden ser declaradas en cualquier `scope`
 */
const MAX_POINTS: u32 = 100_000;