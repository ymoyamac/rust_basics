use std::ops::Deref;
pub struct MyBox<T>(T);

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

fn main() {
    let mut number = 5;
    println!("Number: {}", &number);
    /*
     * Una cosa es usar una referencia para imprimirla con el macro 'println!' y otra es operar
     * con referencias
     */
    println!("Number: {}", &number);
    println!("Number: {}", &number);
    println!("Number: {}", &number);
    println!("Number: {}", &number);

    let ref_number = &mut number;

    println!("Ref: {}", ref_number);

    /*
     * Aquí estamos indicando que enviaremos una referencia mutable para modificar el valor al que
     * 'ref_number' está apuntando
     */
    add_two(ref_number);

    println!("Number: {}", &number);

}

pub fn add_two(number: &mut i32) {
    /*
     * No es valido escribir 
     * ´´´rust
     * number = &(*number + 2);
     * ´´´
     * porque estamos diciendo que se va a asignar la referencia de un valor del resultado de
     * la expresión de '*number + 2', lo que nos está diciendo 'rust' es que ese valor no va
     * a vivir lo suficiente para que 'ref_number' apunte a él y se volvería una referencia
     * colgante.
     */
    *number = *number + 2;
    println!("Number: {number}");
    
}