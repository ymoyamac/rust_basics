/*
 * Los tipos de datos `scalar` representan un solo valor y son indivisibles sin subestructuras
 * internas. `Rust` maneja dos subsets de tipos los `scalar` y los `compound`
 */
fn run() {
    /*
     * Los tipos `Integer` son números enteros sin parte decimal, y pueden ser con signo
     * (positivos o negativos) o sin signo (solo positivos). Los tamaños varían en función de
     * los bits que ocupan en la memoria.
     * Con signo (signed): i8, i16, i32, i64, i128, isize
     * Sin signo (unsigned): u8, u16, u32, u64, u128, usize
     */
    let byte: u8 = 255; // -> 0 .. 255
    let min_byte = i8::MIN;
    let byte_signed: i8 = 127; // -> -128 .. 127
    println!("Byte: Unsigned {byte}, signed {min_byte} to {byte_signed}");

    let short: u16 = 65_535; // -> 0 .. 65,535
    let min_short = i16::MIN;
    let short_sg: i16 = 32_767; // -> -32,768 .. 32,767
    println!("Short: Unsigned {short}, signed {min_short} to {short_sg}");
    
    let int: u32 = 4_294_967_295; // -> 0 .. 4,294,967,295
    let min_int = i32::MIN;
    let int_sg: i32 = 2_147_483_647; // -> -2,147,483,648 .. 2,147,483,647
    println!("Integer: Unsigned {int}, signed {min_int} to {int_sg}");
    
    let long: u64 = 18_446_744_073_709_551_615; // -> 0 .. 18,446,744,073,709,551,615
    let min_long = i64::MIN;
    let long_sg: i64 = 9_223_372_036_854_775_807; // -> -9,223,372,036,854,775,808 .. 9,223,372,036,854,775,807
    println!("Long: Unsigned {long}, signed {min_long} to {long_sg}");
    
    let biglong: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455; // -> 0 .. 340,282,366,920,938,463,463,374,607,431,768,211,455
    let biglong = i128::MIN;
    let biglong_sg: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727; // -> -170,141,183,460,469,231,731,687,303,715,884,105,728 .. 170,141,183,460,469,231,731,687,303,715,884,105,727
    println!("BigLong: Unsigned {biglong}, signed {biglong} to {biglong_sg}");

    /*
     * Los `scalar types` también tienen representación en formato decimal, hexadecimal, octal y binario.
     * Los tipos literales numéricos por defecto son de tipo `i32` a excepción de la representación de `byte`
     */
    let decimal = 30;
    let hex = 0xff; // -> prefijo `0x` va de 0 .. 9 y A .. F
    let octal = 0o377; // -> prefijo `0o` va de 0 .. 8
    let binary = 0b1111_1111; // -> prefijo `0b` va de 0 .. 1
    let byte = b'A'; // -> prefijo `b` representa el valor ASCII del caracteres en número
    /*
     * La notación sufijo de tipo `type suffix` se usa para indicar que un literal numérico de valor `100`
     * debe de ser interpretado como un tipo de `u16` y no como un `i32`
     */
    let type_suffix = 100u16;
    println!("Deacimal: {decimal}");
    println!("Hexadecimal: {hex}"); // Número hexadecimal de base 16
    println!("Octal: {octal}"); // Número hexadecimal de base 8
    println!("Binary: {binary}"); // Número hexadecimal de base 2
    println!("Byte: {byte}");
    println!("Type suffix: {type_suffix}");

    /*
     * `Rust` permite realizar operaciones entre tipos escalares (como enteros) de diferentes tipos
     * siempre y cuando hagas una conversión explícita entre ellos.
     * Está diseñado para prevenir errores relacionados con la manipulación de valores de diferentes
     * tamaños y tipos sin una conversión adecuada.
     */
    let res = (short as i64) + (short_sg as i64);
    println!("Operation: {}", res);

    let x: u8 = 100;
    let y: u32 = 200;
    /*
     * El operador `as` `type casting operator` es utilizado para hacer una conversión explícita
     * entre tipos de datos
     */
    let sum: i64 = (y + (x as u32)) as i64;
    println!("The sum is: {}", sum);
}
