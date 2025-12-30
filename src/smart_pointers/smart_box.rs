
/*
 * Todos los 'smart pointers' implementan el 'trait' 'Deref' que le indica a
 * 'rust' como se tiene que desreferenciar y te permite trabajar con la instancia de
 * la 'struct' del puntero inteligente como si fuera una referencia o un sp. Y también
 * implementan el trait 'Drop' que te permite ejecutar código antes de que la instancia
 * salga del ambito.
 *
*/ 
pub fn smart_box() {
    /*
    * El smart pointer de 'Box' es el sp más sencillo de todos, 'Box' permite
    * allocar valores en el 'HEAP' o valores que su tamaño es dinámico y no se conoce
    * en tiempo de compilación p.e. un vector, inicialmente puede ser de 10 posiciones,
    * pero más adelante en el código puede crecer a 20.
    * 
    * A diferencia de las referencias que toman prestados los datos a los que apuntan
    * 'Box' es el único dueño del valor alojado en memoria.
    *
    */
    let boxed = Box::new(32);
    println!("Boxed: {:?}", *boxed);
    println!("Boxed: {:?}", boxed); // Es lo mismo usar el operador de desreferencia '*'

    op_add_box(boxed);

    // println!("Boxed: {:?}", boxed); // No se puede usar de nuevo 'boxed' porque 'Box'
    // sigue las reglas de 'borrowing'

}

pub fn op_add_box(b: Box<i32>) -> i32 {
    b.wrapping_add(200)
}

#[derive(Debug)]
pub struct BigData {
    buffer: Vec<u8> // String
}

pub fn main() {

    let big_data = BigData {
        buffer: vec![42; 10_000_000]
    };

    // process_data(big_data); // Se mueve 'big_data' al ambito de la fn

    /*
     * Se reserva memoria para 'big_data' y se alloca en el 'HEAP'
     */
    let bd_boxed = Box::new(big_data);

    /*
     * El contenido de 'bd_boxed' sigue estando en el 'HEAP', solo se mueve el
     * puntero al ambito de la función
     */
    process_data_in(bd_boxed);

    // println!("Boxed data {:?}", bd_boxed.buffer.len()); // Se transfiere
    // la propriedad y 'bd_boxed' deja de ser accesible
}

/**
 * Mover 'big_data' al ambito de la función, pero estó puede ser muy costoso,
 * porque los datos se copean en el 'STACK', mover una variable de 10MB puede
 * tomar mucho tiempo
 * 
 */
pub fn process_data(bd: BigData) { // Aquí se está creando una nueva variable
    // en el ambito de la función 'process_data'
    println!("Data: {:?}", bd.buffer)
}

/**
 * Si se envuelve en un 'Box' solo se copean los 8 bytes que ocupa el puntero,
 * porque los datos ya están en el 'HEAP'
 * 
 *     STACK                          HEAP
 * 
 *  +------------+                  +-------+
 *  | ptr | 0x.  | ---------------> | 0 | * |
 *  |------------|                  | 1 | * |
 *  | len | 10MB |                  | 2 | * |
 *  |------------|                  | 3 | * |
 *  | cap | 10MB |                  | n | * |
 *  +------------+                  +-------+
 * 
 * 
 */
pub fn process_data_in(bd: Box<BigData>) {
    println!("Data: {:?}", bd.buffer)
}