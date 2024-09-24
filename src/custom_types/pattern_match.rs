/*
 * En `Rust` existe un operador de flujo de control llamado `match`, que permite comparar un valor
 * con una serie de patrones
 */
#[derive(Debug)]
enum FiftyCent {
    Glod,
    Silver
}

#[derive(Debug)]
enum Coin {
    FiftyCentPesos(FiftyCent),
    OnePeso,
    TwoPesos,
    FivePesos,
    TenPesos
}

fn main() {
    let five_pesos = Coin::TenPesos;
    println!("{:?}", five_pesos);
    let pesos = get_value_pesos(five_pesos);
    /*
     * Proporciona una forma concisa y expresiva de manejar diferentes casos y opciones, y es
     * particularmente útil en situaciones donde hay múltiples posibilidades.
     */
    match pesos {
        1 => println!("Get one million more like this and you can have anyting, {:?}", pesos),
        2 => println!("Gum, {:?}", pesos),
        5 => println!("You can have a metro ticket, {:?}", pesos),
        10 => println!("You can have a two metro tickets, {:?}", pesos),
        /*
         * `Rust` tiene un patron para cuando no queremos listar todos los posibles valores es
         * `The _ Placeholder` se usa como caso por defecto, cuando no coincide con ninguno de los
         * anteriores se ejecuta este `arm`
         */
        _ => println!("What?")
    }

    let cent = Coin::FiftyCentPesos(FiftyCent::Glod);
    let _ = get_value_pesos(cent);

    let val = Some(5);
    let res = patter_optional(val);
    println!("Resul of addition: {:?}", res);
    if_let_pattern(Coin::OnePeso);

}

fn get_value_pesos(peso: Coin) -> u8 {
    /*
     * El patron `match` se utiliza cuando nos importan comparar multiples patrones, es decir que
     * podamos tener "multiples opciones". Se puede evaluar cualquier expresión de `Rust`
     */
    match peso {
        /*
         * La expresión "Patterns That Bind to Values" en Rust se refiere a patrones que no solo
         * verifican la estructura o tipo de un valor, sino que también extraen (o "unen") partes
         * del valor a variables para usarlas en el bloque de código correspondiente.
         */
        Coin::FiftyCentPesos(cent) => { // `cent` es un patrón que "bindea" el valor almacenado en `FiftyCentPesos` a la variable `cent`.
            println!("In 2010 a gum, now noting. Is a {:?} coin", cent);
            50
        },
        /*
         * Cada uno de los casos se conoce como `pattern` (patron), si el valor coincide se ejecutara
         * el código después del operador `=>`
         */
        Coin::OnePeso => 1,
        Coin::TwoPesos => 2,
        /*
         * Los patrones pueden ser más extensos y tener bloques de ejecución
         */
        Coin::FivePesos => {
            println!("Five pesos!");
            5
        },
        Coin::TenPesos => 10
    }
}

fn patter_optional(val: Option<i32>) -> Option<i32> {
    match val {
        /*
         * La variable `x` es el valor que contendra la variante `Some` (bind), entonces `x` toma
         * el valor de `5` y se ejecuta el código en el `arm` que coincida, se suma uno y se devuelve
         * un nuevo valor de `Some` con el valor de `6` adentro
         */
        Some(x) => Some(x + 1),
        /*
         * Si se envia un `None` el programa se detiene
         */
        None => None
    }
}

/*
 * El patron `if let` es una forma más concisa y menos "verbosa" de manejar solo un `pettern` e
 * ignorar el resto
 */
fn if_let_pattern(coin: Coin) {
    match coin {
        Coin::OnePeso => println!("One pesos"),
        _ => {
            println!("Its not a One peso");
            ()
        }
    }
    /*
     * La alternativa a esto es la sintaxis `if let`, es `syntax sugar` del patron `match` para
     * evaluar solo un `pattern`
     */
    if let Coin::OnePeso = coin {
        println!("Processing...");
        println!("One pesos {:?}", coin)
    } else {
        println!("Its not a One peso")
    }
    /*
     * La variable `peso` no existe fuera del `scope` del `if let`
     */
}
