/*
 * El motivo de que `Rust` trabaje de esta forma es porque no tiene un `garbage collector`, entonces
 * es si no tiene un `GC` entonces es nuestra responsabilidad liberar la memoria cuando se necesite,
 * pero hacer esto manualmente puede causar que 1) nos olvidemos de ella, entonces desperdiciariamos
 * memoria, 2) liberarla antes de tiempo, por lo que tendriamos variables invalidas, 3) o hacerlo dos
 * veces.
 * `Rust` toma una ruta diferente, regresar la memoria cuando el propietario salga de su `scope`
 * una vez fuera `Rust` llama automáticamente la función `drop()`
 */
fn string_type() {
    /*
     * En `Rust` el tipo `String` es un tipo que representa a las cadenas de caracteres mutables
     * y que se almacenan en el `heap` pueden crecer, modificarse y gestionan su propia memoria
     * de manera dinámica.
     */
    let mut s = String::from("Hello");
    /*
     * El método `.push_str()` agrega una cadena al final del `String` modificando su contenido
     * original
     */
    s.push_str(", world!");
    println!("{}", s);  // Imprime: Hello, world!
    /*
     * El tipo `String` sigue las reglas de `ownership`, por lo que el valor `Hello, world!` solo
     * puede tener un propietario y se debe ceder o clonar la propiedad para compartir el valor
     * de la cadena
     */
    let mut str = s;
    /*
     * El método `.push()` agrega solo un caracter al final del `String` modificando su contenido
     * original
     */
    str.push('!');
    println!("Str value: {}", str);
    let p_bear = String::from("Panda bear");
    let eat = String::from("can eat bambu!");
    /*
     * El operador `+` hace una llamda interna al método `.add(mut self, other: &str) -> String`
     * este método toma la propiedad de `p_bear` que es un `String` como primer argumento y el segundo
     * es un `&str`, posteriormente regresa un nuevo `String`, para el segundo operador `+` ya tendrá el
     * resultado de `"Panda bear" " "`
     */
    let phrase = p_bear + " " + &eat;
    /*
     * La variable `p_bear` ya no es accesible
     */
    //println!("What bear is? {}", p_bear);
    println!("{phrase}");

    /*
     * El macro `format!` concatena varias cadenas de manera más flexible
     */
    let s = format!("{} {}", "Hello", "world");
    println!("{s}");
}

fn move_concept() {
    /*
    * El lado izquierdo es un puntero que apunta al contenido del `string`, tiene una longitud
    * `length` medida en bytes que es cuanta memoria, y una capacidad `capacity` que es la cantidad
    * total de memoria
    */
    let hello = String::from("Hello");
    /*
     * El concepto de `move` es la forma en la que `Rust` transfiere la propiedad de una variable
     * a otra, la variable original deja de ser accesible y la nueva variable se convierte en la
     * nueva propietaria del recurso
     */
    let s1 = String::from("Hello");
    /*
     * Ahora la variable `s1` es inaccesible y `s2` es la propietaria del recurso que se almacena en
     * el `heap`. `Rust` realiza un `shallow copy` una copia superficial del puntero que se encuentra
     * en el `stack`, pero no del recurso
     * 
     *     STACK                       HEAP
     *  +---------+                  +-------+
     *  | ptr | 0x| ---------------> | 0 | h |
     *  |---------|                  | 1 | e |
     *  | len | 5 |                  | 2 | l |
     *  |---------|                  | 3 | l |
     *  | cap | 5 |                  | 4 | 0 |
     *  +---------+                  +-------+
     * 
     * 
     */
    let s2 = s1;
    println("the unic owner is s2 with value: {}", s2);
}

fn copy_clone_concepts() {
    let x = 10;
    /*
     * Aquí `Rust` no hace un `move` no cambia de propietario, porque los valores `scalar` o primitivos
     * son de tamaño fijo, conocidos en tiempo de compilación y ya estan almacenados en el `stack`, por
     * lo tanto, se puede hacer una copia y da lo mismo si se hace un `deep` o un `shallow` `copy`
     */
    let y = x;
    /*
     * Para los tipos primitivos `scalar` se puede usar el `trait` `Copy`, pero no se puede usar junto
     * con el `trait` `Drop` si es que el tipo ya lo tiene o alguno lo tiene
     */
    let x = x;
    /*
     * Cualquier grupo de tipos `scalar` puede implementar el `trait` `Copy`
     * Esto es valido
     */
    let tup: (i32, bool) = (10, true);
    /*
     * Esto no es valido, aunque la tupla sea un tipo de datos `compund`, el tipo `string` no lo es 
     */
    let invalid_tup: (i32, String) = (10, String::from("hello"));
    println!("X: {}, Y:{}", x, y);

}

fn main() {
    let s = String::from("hello"); // -> La variable `s` esta en el `scope` de la función `main`
    /*
     * El valor de la variable `s` se mueve `move` dentro de la función `takes_ownership` después
     * ya no será valida
     */
    takes_ownership(s);
    let x = 5; // -> La variable `x` esta en el `scope` de la función `main`
    /*
     * La variable `x` se debería mover `move` dentro de la función, pero como es un tipo `scalar`
     * se crea una copia, entonces `x` seguirá siendo accesible
     */
    makes_copy(x);

    /*
    * La función `gives_ownership()` mueve `moves` su valor de retorno dentro de la variable `s1`
    */
    let s1 = gives_ownership();

    let s2 = String::from("Go back!"); // -> La variable `s2` esta en el `scope`
    /*
     * La variable `s2` se mueve dentro de `takes_and_gives_back()` y se mueve el valor de retorno
     * dentro de la variable `s3`
     */
    let s3 = takes_and_gives_back(s2);
    /*
     * Por último se transfiere la propiedad a la variable `s3` y `s2` ya no es accesible
     */
    println!("values: {}, {}", s1, s3);
    /*
     * Cuando sale del `scope` `s3` y `s1` se dropea, cuando sale `s2` nada pasa porque se movio
     * dentro de la función `takes_and_gives_back()`
     */
}

/*
 * Ahora la variable `str` es propietaria del valor `hello`
 */
fn takes_ownership(str: String) {
    println!("Now the owner is str with value {}", str);
}// -> Cuando `str` sale del `scope` la función `drop` es llamada y se devuelve la memoria

/*
 * Ahora la variable `val` tiene una copia del valor de `5`
 */
fn makes_copy(val: i32) {
    println!("Makes a copy of x: {}", val);
    /*
     * En este caso, cuando `val` sale del scope, no se liberan recursos, porque los tipos como `i32`
     * no poseen recursos que necesiten ser liberados. Solo se elimina su valor de la `stack`, lo
     * cual es un proceso trivial en términos de gestión de memoria. 
     */
}

fn gives_ownership() -> String {
    let val = String::from("Again!"); // -> La variable `val` esta en el `scope`
    /*
     * `val` es retornada y se mueve fuera de la llamada de la función
     */
    val
}

fn takes_and_gives_back(val: String) -> String {
    /*
     * Aquí el argumento `val` ahora es propietario del recurso `Go back!` se transfiere la propiedad
     * del valor, el valor es retornado y se mueve fuera de la llamada de la funcion
     */
    val
}
