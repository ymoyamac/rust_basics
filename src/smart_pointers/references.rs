pub fn references() {
    /*
     * Una referencia como '&integer' es un puntero inteligente en sentido que podemos seguir la dirección
     * y encontrar el valor almacenado, es el más común de todos los punteros inteligentes.
     * 
     */

    let s1 = String::from("hello");
    let s = &s1;

    println!("Reference: {:?}", *s); // 'Rust' des-referencia la referencia automaticamente
    println!("Reference: {:?}", s);

     /* 
     * Una referencia en 'rust' garantiza que apunta a un valor valido durante toda la vida útil del
     * programa.
     * 
     * Esté tipo de puntero inteligente sigue las reglas del 'borrowing', el sistema de prestamos 
     * de 'rust', es decir, que solo toma prestados los datos.
     * 
     * 1.- Pueden existir múltiples referencias inmutables a un mismo valor.
     * 2.- Solo puede existir una referencia mutable al mismo tiempo a un valor.
     */
    let integer: i32 = 32;
    println!("Integer: {:032b}", integer);

    let ptr_integer = &integer as *const i32; // casteo explícito a un 'raw pointer'
    println!("Ptr: {:?}", ptr_integer);
    println!("Ptr: {:?}", ptr_integer.wrapping_add(1));

    let long = 100u64;
    let ref_long = &long;
    let long_boxed = Box::new(long);
    println!("P: {:p}", long_boxed); // el formato {:p} es para imprimir direcciones de memoria 'P: 0x140606030'

    println!("Sumando con ref {}", ref_long + 100);
    println!("Sumando con box {}", *long_boxed + long);
}