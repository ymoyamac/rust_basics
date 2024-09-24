/*
 * En otras palabras, la definición `struct` es como una plantilla general para el tipo, y las instancias
 * rellenan esa plantilla con datos particulares para crear valores del tipo.
 */
#[derive(Debug)]
struct User {
    username: String, // -> `fields`
    /*
     * Las `struct` estan compuestas por `fields` (campos) que describen las características, cada
     * `field` es un par `key: value`, donde las `keys` son el nombre del `field` y el `value` son
     * el dato que queremos almacenar en el `field`
     */
    email: String, // -> key: value pairs
    sign_in_count: u64,
    is_active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        /*
         * Si los parámetros y los campos tienen el mismo nombre, se puede usar la sintaxis abreviada
         * para la inicialización del campo, se "setea" el valor del parámetro en el campo
         * 
         *  * username: username, // -> antes
         */
        username, // -> después
        email,
        is_active: true,
        sign_in_count: 2003
    }
}

fn run() {
    /*
     * Las instancias son inmutables por defecto
     */
    let main_user = User {
        username: String::from("Yael Moya"),
        email: String::from("yael.moya@email.com"),
        is_active: true,
        sign_in_count: 1990
    };
    println!("User: {:?}", main_user);
    /*
     * Se puede acceder a los valores mediante el operador `.` con la variable punto y el nombre 
     * del `field`
     */
    println!("Username: {}, email: {}, is active: {}, id: {}", main_user.username, main_user.email, main_user.is_active, main_user.sign_in_count);

    let second_user = build_user(
        String::from("Alejandro"),
        String::from("ale_taboa@email.com")
    );

    println!("Other user: {:?}", second_user);

    let admin_user = User {
        sign_in_count: 2999,
        /*
         * La sintaxis .. especifica que los campos restantes no establecidos explícitamente deben
         * tener el mismo valor que los campos de la instancia dada. Se reutilizan los valores
         */
        ..main_user
    };
    println!("ADMIN user: {:?}", admin_user);


}

/*
 * Las `tuple structs` son variantes de las `structs` tradicionales, pero en lugar de tener campos con
 * nombres, los campos se indexan por posición
 */
#[derive(Debug)]
struct Reference(String, i32, i32);

fn tuple_struct() {
    /*
     * Aunque dos `tuple structs` tengan los mismo tipos de datos internamente, definen tipos diferentes
     */
    let val = Reference(String::from("Pointing"), 5, 5);
    println!("Variable: {:?}", val);
    let var = Reference(String::from("Pointing"), 5, 5);
    /*
     * También se puede usar el `pattern matching to destructure` para declarar y obtener los valores
     * individuales y asignarlos a una nueva variable
     */
    let Reference(ptr, len, cap) = var;
    println!("Pointing to: {}, len: {} and cpacity {}", ptr, len, cap);
}


/*
 * Se define una `struct` `Rectangle` con dos `fields` (campos), `width` y `height` ambos de tipo `u32`
 */
struct Rectangle {
    width: u32,
    height: u32,
}

/*
 * Los métodos están definidos en el contexto de una `struct`, se definen dentro de un bloque 
 * `implementation` `impl`
 * 
 */
impl Rectangle {
    /*
     * Los métodos asociados no dependen de una instancia y no tienen el parámetro `self`, estan
     * asociados directamente al tipo
     */
    fn new(width: u32, height: u32) -> Self {
        /*
         * El tipo `Self` hace referencia al nombre de la estructura, tiene la flexibilidad de que
         * si cambia el nombre de la estructura no es necesario refactorizar el código
         */
        Self {
            width,
            height
        }
    }

    /*
     * Los métodos de instancia toman como parámetro el argumento `self` que hace referencia a la
     * instancia, para acceder a los `fields` de la misma `struct`. Se usa como referencia `&`
     * porque los métodos pueden tomar la propiedad de la instancia
     */
    fn calc_area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rec: &Rectangle) -> bool {
        self.width < rec.width && self.height < rec.height
    }
}

/*
 * Una estructura puede tener multiples `impl blocks`
 */
impl std::fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "Rectangle {{ width: {}, height: {} }} \n, area: {} ", self.width, self.height, self.calc_area())
        }
    }
}

fn run_struct() {
    let width = 30;
    let height = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );
    let reactangle_dimensions = (70, 80);
    let res = area_tup(reactangle_dimensions);
    println!(
        "The area of the rectangle is {} square pixels.",
        res
    );
    run();
    /*
     * El método asociado `new` permite crear una instancia de `Rectangule`, se invoca son el 
     * nombre de la estructura y el operador `::` que se usa para acceder a elementos asociados
     */
    let rectangle_two = Rectangle::new(10, 20);
    let result = rectangle_two.calc_area();
    println!(
        "The area of the rectangle is {} square pixels.",
        result
    );
    let rec = Rectangle::new(70, 100);
    if rectangle_two.can_hold(&rec) {
        println!("The rectangle is bigger than the second");
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

/*
 * Trabajar con tuplas nos permite construir una estructura más clara que solo variables, solo se pasa
 * un argumento en la firma de la función
 */
fn area_tup(dimensions: (u32, u32)) -> u32 {
    /*
     * Pero esta es poco clara, tenemos que tener encuenta que la posición cero `0` de la tupla es el ancho
     * `width` y el uno `1` es el `height`, pero ¿Qué pasa si alguien manda los valores invertidos?
     */
    dimensions.0 * dimensions.1
}

fn run() {
    /*
     * Se crea una instancia de `Rectangle` con los valores `230` y `10`
     */
    let rectangle_one = Rectangle {
        width: 230,
        height: 10
    };
    let res = area_struct(&rectangle_one);
    println!(
        "The area of the rectangle is {} square pixels.",
        res
    );
    /*
     * La notación `:?` le indica a `Rust` que vamos a usar el formato `Debug` para formatear la salida
     * y mostart la estructura
     */
    println!("The rectangle is: {:?}", rectangle_one);
}

/*
 * La definición ahora es más clara, ahora la función que calcula el área recibe una instancia inmutable
 * de una referencia/prestamo `borrow`
 */
fn area_struct(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}
