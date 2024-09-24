/*
 * Los `String slices` son un tipo de `String` que en lugar de tener una referencia a todo el
 * `String` solo hacen referencia a una porción de el
 */
fn run() {
    /*
     * Los `Strings literals` son un `slice` y el hecho de que sean muy eficientes es porque `Rust`
     * los `harcodea` en el `binary`. Es un `slice` que apunta a esa parte específica del `binary`
     */
    let s: &str = "Hello, world!";
    get_first(s);
    /*
     * Se pueden crear `String slices` usando el `range operator` con corchetes
     * `[starting_index..ending_index]` donde `starting_index` es la primera posición del `slice`
     * y en `ending_index` es la última posición más uno
     */
    let phrase = String::from("Hello, world! From Rust lang");
    get_first(&phrase);
    /*
     * Internamente, el `slice` guarda la posición de inicio y su longitud `length` que corresponde
     * a `ending_index-starting_index`
     */
    let slice = &phrase[0..5];
    println!("Portion of String: {}", slice);
    let word_found = get_word(&phrase, 3);
    println!("Word: {}", word_found);

}

/*
 * La notación `&str` de la firma de la función `get_first()` permite recibir como parámetros tipos
 * `&String` y `&str`
 */
fn get_first(phrase: &str) -> &str {
    let chars = phrase.trim().as_bytes();
    for (index, &char) in chars.iter().enumerate() {
        if char == b' ' {
            return &phrase[0..index];
        }
    }
    &phrase[..]
}

fn get_word(phrase: &String, word: i32) -> &str {
    let chars = phrase.trim().as_bytes();
    let mut total_spaces_pos = Vec::<i32>::new();
    for (index, &char) in chars.iter().enumerate() {
        if char == b' ' {
            total_spaces_pos.push(i32::try_from(index).unwrap());
        }
    }
    if total_spaces_pos.len() > 0 {
        for _ in chars {
            let starting_index = if word == 0 {
                0
            } else {
                total_spaces_pos
                    .get((word as usize) - 1)
                    .unwrap()
                    .clone() as usize
            };
            let ending_index = total_spaces_pos
                .get(word as usize)
                .unwrap()
                .clone() as usize;
            return &phrase[starting_index..ending_index];
        }
    }

    &phrase[..].trim()
}

fn are_same() {
    /*
     * La variable `left` apunta a un `literal string` que es almacenado en la parte de la memoria
     * estática
     */
    let left: &str = "Hello, chatgpt";
    /*
     * La variable `rigth` apunta a un `String` que es almacenado en el `heap`
     */
    let rigth = String::from("Hello, chatgpt");
    /*
     * `Rust` implementa la comparación `==` para tipos como `&str` y `String` de manera que compara
     * el contenido de las cadenas y no sus ubicaciones en memoria. Si ambos contienen los mismo 
     * caracteres en el mismo orden, la comparación devolverá `true`
     */
    if left == rigth { // -> `true`
        println!("Yes");
    }
}