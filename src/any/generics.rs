/*
 * Los tipos genéricos estan basados `types to-be-specified-later` especificar después los tipos,
 * es decir, escribimos un `placeholder` en lugar de especificar el tipo, lo que nos da la flexibilidad
 * de aceptar cualquier tipo
 */
#[derive(Debug)]
struct Point<T> {
    /*
     * Los tipos `placeholders` son especificados en tiempo de compilación, y son manejados como
     * parámetros
     */
    x: T,
    y: T
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct OtherPoint<T, U> {
    x: T,
    y: U
}

/*
 * Las definiciones de métodos o bloques de implementación `impl` también pueden tener genéricos,
 * y deben de definir por lo menos el mismo número de la `struct`
 */
impl<T, U> OtherPoint<T, U> {
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }
}

fn run() {
    /*
     * El tipo de `T` debe de ser el mismo para la declaración
     */
    let integer = Point { x: 5, y: 10 };
    /*
     * Se pueden declarar multiples tipos de genéricos, de esa forma se puede tomar distintos tipos
     */
    let float = OtherPoint { x: 10, y: 4.0 };
    println!("Integer point: {:?}, Floating point {:?}", integer, float);
}

/*
 * En este caso, `T` es un genérico que representa cualquier tipo que implemente el `trait` `PartialOrd`
 * (para que los elementos se puedan comparar).
 */
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for i in list.iter() {
        if i < largest {
            largest = i;
        }
    }
    largest
}