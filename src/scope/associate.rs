
#[derive(Debug)]
pub struct Box {
    pub integer: i32, // -> atributo/campos
    pub double: f64
}

// Esto funciona como un espacio de nombres para asociar elementos a nuestra estructura
impl Box { // -> using namespace Box {}

    /*
     * La función 'new' conceptualmente hablando es un "constructor", porque la función nos regresa
     * una "instancia" de nuestra 'struct'.
     * 
     * La función 'new' es el equivalente en 'java' de tener un método estático
     */
    pub fn new(integer: i32, double: f64) -> Self { // -> Constructor, pero los constructores no existe en 'rust'
        Self { integer, double }
    }

    pub fn unboxing(&self) -> (i32, f64) { // -> métodos
        (self.integer, self.double)
    }
}

pub fn create_box() {
    let boxed = Box::new(1, 1.1);
    println!("Boxed: {boxed:?}")
}
