/*
 * Toda referencia en `Rust` tiene un `lifetime`, cuando queremos que una referencia viva más
 * del `scope` donde se declaro agregamos un `lifetime`
 */
#[derive(Debug)]
struct Person<'a> {
    /*
     * Cuando el `owner` (propietario) sale del `scope` su valor es eliminado y la memoria es liberada
     * un `lifetime` le permite a `Rust` asegurarse que la referencia a un objeto siga siendo válida
     * durante el tiempo que sea necesario
     */
    _name: &'a str,
    _age: i32
}

impl<'a> Person<'a> {
    fn new(name: &'a str, age: i32) -> Self {
        Self { _name: name, _age: age }
    }

    fn name(&self) -> &'a str {
        self._name
    }
}

fn main() {
    /*
     * Aquí `Rust` se asegura que la referencia que apunta al `slice` `John Doe` sea una referencia
     * valida hasta que el objeto salga del `scope` y sea liberado
     */
    let police = Person::new("John Doe", 35);
    println!("Hi! {:?}", police.name());
    scoping();
    let phrase = String::from("Hi! from Rust");
    let greeting = "Hi!";
    let largest = largest_slice(&phrase, &greeting);
    println!("The largest String is {}: from the two options 1) {phrase} and 2) {greeting}", largest);
}

fn scoping<'a>() { // -> 'a
    let r: &i32 = &6;
    { // -> 'b
        let x = 5;
        /*
         *  * r = &x; 
         */
        println!("x: {x}");
    }
    /*
     * La asignación no es valida, porque la referencia a la variable `x` "no vive lo sificiente"
     * para se utilizada después del `scope` donde es declarada, esto se debe a que después de salir
     * del `scope` con un `lifetime` `'b` la variable `x` es liberada, por lo tanto, la referencia
     * apunta a un valor no valido, esto provoca una referencia colgante `dealing references`
     */
    println!("r: {}", r);
}

/*
 * Esta función toma referencias de `String` porque no queremos tomar la propiedad `ownership` de las
 * cadenas, y devolvemos la referencia de la cadena
 */
fn largest_slice<'a>(var: &'a str, val: &'a str) -> &'a str {
    if var.len() > val.len() {
        var
    } else {
        val
    }
}