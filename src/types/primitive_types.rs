
fn run() {
    /*
     * Los tipos numéricos con parte decimal son `Floating-Point Types` que son los tipos
     * `f32` y `f64`, por defecto los literales numéricos decimales son de tipo `f64` ya que
     * los procesadores modernos son tienen la misma velocidad con tamaños de 32 y 64 bits 
     */

    let double_min = f64::MIN;
    let double_max = f64::MAX;
    println!("Double {double_min} to {double_max}");

    let float_min = f32::MIN;
    let float_max = f32::MAX;
    println!("Float {float_min} to {float_max}");

    /*
     * La precisión `double-precision` es de 64 bits
     * 
     *  * 1 bit para el signo
     *  * 11 bits para el exponente
     *  * 52 bits para la parte decimal
     */
    let double = 1808.918230981092213123; // -> double-presicion
    println!("Double: {}", double);
    /*
     * La precisión `single-precision` es de 32 bits
     * 
     *  * 1 bit para el signo
     *  * 8 bits para el exponente
     *  * 23 bits para la parte decimal
     */
    let float_literal: f32 = 5.333333; // -> single-precision, 32 bits
    println!("Float: {float_literal}");

    /*
     * Los tipos `Boolean` solo tienen dos valores posibles `true` y `false` son del tamaño de
     * 1 byte
     */
    let is_active: bool = true;
    let is_paid = false;
    println!("Is active {is_active} and is paid {is_paid}");

    /*
     * `Rust` también soporta el tipo primitivo `char` que son la representación de letras, las
     * literales de tipo `char` son definidas con comillas simples y son de tamaño de 4 bytes
     * por lo que pueden representar más caracteres qe los de ASCII
     */
    let c = 'c'; // 32 bits UTF-32
    let a: char = '\u{0000}';
    let letter: char = 'A';      // U+0041
    let accent: char = 'é';      // U+00E9
    let emoji: char = '😊';      // U+1F60A

    println!("letter: {}, {}", letter, a);
    println!("accent: {}", accent);
    println!("emoji: {}", emoji);
    /*
     * El tipo char en Rust puede ser convertido a su valor Unicode usando el método `.escape_unicode()
     * o como un número `u32`
     */
    println!("Unicode scalar value of 'é': U+{:X}", c as u32);
}

/*
 * Los tipos `compound types` son tipos de datos que pueden agrupar multiples valores en una
 * sola entidad. `Rust` tiene dos tipos primitivos las `tuples` y los `arrays`
 */
fn compound_tuple() {
    /*
     * Una `tuple` es una estructura que agrupa multiples valores del mismo o diferentes tipos en
     * una sola entidad. Las tuplas tienen una longitud fija, una vez definida no pueden cambiar
     */
    let tuple: (u8, &str, bool, f64) = (255, "Slice string", false, 18.01);
    /*
     * Para declarar la `tuple` se escribe dentro de los parentesis separados por comas y para
     * acceder a un valor en específico se hace por medio de operador `.` y el `index` comenzando
     * desde el número cero. Cada posición puede tener un tipo diferente
     */
    println!("First element: {}", tuple.0);
    println!("Second element: {}", tuple.1);
    println!("Third element: {}", tuple.2);
    /*
     * La variable `tuple` esta asociada a la tupla entera, porque una tupla es considerada como
     * un solo elemento compuesto. Para obtener cada valor individual de la tupla se puede usar
     * el `pattern matching to destructure` los valores de la tupla
     */
    let (first, _, _, fourth) = tuple; // -> El `_` ignora a los elementos
    /*
     * El `pattern matching to destructure` o destructuración es la forma de obtener los valores
     * individuales de una estructura como `tuples`, `arrays` y `structures` y asignarlos a una
     * variable(s) separada(s) sin tener que utilizar índices
     */
    println!("Now first is a variable with tuple value associated: {first}");
    println!("Now fourth is a variable with tuple value associated: {fourth}");
}

/*
 * Los `arrays` en `Rust` son almacenados en el `stack`, dado que son estructuras de datos de
 * tamaño fijo `Rust` puede reservar espacio en memoria contigua en tiempo de compilación
 */
fn compound_arrays() {
    /*
     * Un `array` es una estructura que agrupa distintos valores del mismo tipo en una sola
     * entidad, son de tamaño fijo e inmutables por defecto y se almacenan en el `stack`
     */
    let numbers: [u8; 5] = [1, 2, 3, 4, 5];
    /*
     * El lado izquierdo de la definición del tipo del `array` indica el tipo de cada elemento
     * que se encuentra dentro, seprado por un `;` con el número de elementos en el
     */
    println!("Length: {}", numbers.len());
    /*
     * Una forma de inicializar en `array` con el mismo valor de elementos es escribiendo el valor
     * que cada elemento va a tener junto con la cantidad
     */
    let phrases = ["Chaining"; 3];
    /*
     * Para acceder a un elemento en particular se puede utilizar `[]` junto con el `index` del
     * elemento, comienza desde cero
     */
    println!("First phrase is {}", phrases[0]);
    for n in numbers {
        println!("Number: {n}");
    }

    /*
     * El método `.is_empty()` devuelve `true` si el arreglo no contiene ningún elemento
     */
    let array: [i32; 0] = [];
    println!("Is empty: {}", array.is_empty()); // Output: Is empty: true

    /*
     * Retorna una referencia opcional `Option<&T>` al elemento en el índice especificado.
     * Si el índice está fuera del rango, devuelve `None`.
     */
    let array = [1, 2, 3];
    if let Some(value) = array.get(1) {
        println!("Value at index 1: {}", value); // Output: Value at index 1: 2
    }

    /*
     * El método `.first()` devuelve una referencia opcional al primer elemento del array o slice.
     * El método `.last()` devuelve una referencia opcional al último elemento.
     */
    let array = [1, 2, 3];
    println!("First element: {:?}", array.first()); // Output: First element: Some(1)
    println!("Last element: {:?}", array.last());   // Output: Last element: Some(3)

    /*
     * El método `.iter()` crea un iterador sobre los elementos del array o slice.
     */
    let array = [1, 2, 3];
    for element in array.iter() {
        println!("{}", element);
    }

    /*
     * El método `.contains()` verifica si el array contiene un valor específico. Retorna true
     * si lo contiene, false de lo contrario.
     */

    let array = [1, 2, 3];
    println!("Contains 2: {}", array.contains(&2)); // Output: Contains 2: true

    /*
     * El método `.reverse()` invierte el orden de los elementos del array o slice in-place
     * (modifica el array original).
     */
    let mut array = [1, 2, 3];
    array.reverse();
    println!("{:?}", array); // Output: [3, 2, 1]


}