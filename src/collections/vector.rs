/*
 * Una colección dinámica se almacena en el `heap`, su tamaño no se conoce en tiempo de compilación y
 * puede crecer o disminuir mientras el programa se ejecuta
 */
fn main() {
    /*
     * Un `vector` es una estructura de datos que te permite almacenar más de un elemento de forma
     * contigua, uno después de otro en memoria, y solo pueden almacenar datos del mismo tipo. Una
     * forma de crear un `vector` vacio es con `new()`
     */
    let mut numbers = Vec::<i32>::new();
    numbers.push(1);
    let mut nums = mapping(numbers);
    println!("Vector numbers: {:?}", nums);
    nums.push(3);
    nums.push(5);
    /*
     * Se puede acceder a los elementos de un vector mediante `index`, aquí se devuelve una copia
     * del elemento en esa posición (siempre y cuando implemente el `trait` `Copy`)
     */
    let third = nums[2];
    println!("Third position: {}", third);
    /*
     * El tipo `String` no implementa el `trait` `Copy`, por lo tanto, se tiene que usar una referencia
     * para acceder a ese elemento
     */
    let names = vec!["Yael".to_string(), "Luis".to_string(), "Alejandro".to_string()];
    let name = &names[0];
    println!("My name is: {name}");
    /*
     * La segunda forma de crear vectores es con el `macro` `vec!` que permite inicializar el vector
     * con elementos
     */
    let colors = vec!["blue", "red", "green"];
    let red = colors[1];
    println!("One color: {red}");
    alloc_vec();

    let ele = get_ele(nums, 1);
    println!("Element: {ele}");
    multi_refs();
    let mut integers = vec![1, 2, 3, 4, 4];
    alter_content_vec(&mut integers);
    println!("Out - Vec: {:?}", integers);
    alter_elements_in_vec(&mut integers);

} // -> Cuando los `vectors` salen del `scope` se liberan junto con su contenido

fn mapping(vec: Vec<i32>) -> Vec<i32> {
    vec.iter().map(|x| x * 2).collect()
}

fn alloc_vec() {
    /*
     * Esta es la reprecentación de un `vector` que contiene el caracter `a` y el `b`, con capacidad
     * de `4`, la parte del puntero también es almacenada en el `heap` en un bloque de memoria contigua
     * y donde `uninit` representa memoria que no ha sido inicializada
     * 
     *              ptr     len    capacity
     *          +--------+--------+--------+
     *          | 0x0123 |      2 |      4 |
     *          +--------+--------+--------+
     *             |
     *             v
     *   Heap   +--------+--------+--------+--------+
     *          |    'a' |    'b' | uninit | uninit |
     *          +--------+--------+--------+--------+
     */
    let mut chars = Vec::<char>::with_capacity(4);
    chars.push('a');
    chars.push('b');
    println!("Capacity: {} and lenght: {}", chars.capacity(), chars.len());
}

fn get_ele(vec: Vec<i32>, index: usize) -> i32 {
    /*
     * El método `.get(index)` devuelve un `Option<&I>`, donde si existe un espacio y un elemento
     * en ese `index` devolvera su variante `Some` con una referencia de ese elemento
     */
    match vec.get(index) {
        Some(ele) => {
            println!("The element in the index: {index} is {ele}");
            ele.clone()
        },
        /*
         * Si intentamos acceder a un `index` que este fuera del rango de `vector`, el método `.get(index)`
         * devolvera la variante `None` sin lanzar un `panic`
         */
        None => {
            println!("There is no element here");
            -1
        }
    }
}

fn multi_refs() {
    /*
     * ¿Por qué debería importarle a una referencia al primer elemento lo que cambia al final del
     * vector? Este error se debe a la forma en que funcionan los vectores: añadir un nuevo elemento
     * al final del vector puede requerir asignar nueva memoria y copiar los elementos antiguos al
     * nuevo espacio, si no hay suficiente espacio para poner todos los elementos uno al lado del
     * otro donde está actualmente el vector. En ese caso, la referencia al primer elemento estaría
     * apuntando a memoria desasignada. Las reglas de préstamo evitan que los programas acaben en
     * esa situación
     */
    let names = vec![1, 2, 3];
    /*
     * Mientras esta referencia exista `Rust` se asegura que `names` no pueda se modificado para
     * evitar inconsistencias, debido a que cada vez que se modifica el `vector` `Rust` copia su
     * contenido si sobre pasa la capacidad del `vector`, por lo tanto, se invalida la referencia y
     * esta referencia apuntaría a un elemento no valido, porque el vector podría moverse a una
     * nueva ubicación en la memoria
     */
    let first = &names[0];
    /*
     * 
     *                   STACK                                     HEAP
     *     first                       names
     *  +---------+                  +---------+                  +-------+
     *  |   ptr   | ---------------> |   ptr   | --------------->(| 0 | 1 |)
     *  +---------+                  +---------+                  | 1 | 2 |
     *                               | len | 3 |                  | 2 | 3 |
     *                               +---------+                  +-------+
     *                               | cap | 3 |                  
     *                               +---------+                  
     *  * names.push(4);
     * 
     *  +---------+                  +---------+                   +-------+
     *  |   ptr   | ------x--------> |   ptr   | -------+         (| 0 | 1 |)
     *  +---------+                  +---------+        |          | 1 | 2 |
     *                               | len | 3 |        |          | 2 | 3 |
     *                               +---------+        |          +-------+
     *                               | cap | 3 |        |          
     *                               +---------+        |
     *                                                  |
     *                                                  |          +-------+
     *                                                  +--------> | 0 | 1 |
     *                                                             | 1 | 2 |
     *                                                             | 2 | 3 |
     *                                                             | 3 | 4 |
     *                                                             +-------+
     */
    println!("First element: {first}");
}

fn alter_content_vec(vec: &mut Vec<i32>) {
    /*
     * Se puede modificar el contenido del vector con una referencia mutable, ahora la variable `vec`
     * puede acceder a modificar el `vector` si tener la propiedad
     */
    vec.push(130);
    println!("Vec: {:?}", vec);
}

fn alter_elements_in_vec(vex: &mut Vec<i32>) {
    for x in vex.iter_mut() {
        *x *= 2;
        println!("Element: {x}");
    }
}