/*
 * Una `enum` es un tipo de dato que lista un conjunto de valores que se puedan tomar, cada valor
 * se conoce como `variant`
 */

 #[derive(Debug)]
enum IpVersionAddress {
    /** 
     * Variante sin datos asociados
     */
    VersionFour,
    /*
     * Cada variante puede tener la definición de un tipo diferente, tener tipos asociados a cada
     * variante permite que cada variante use los tipos de datos correctos
     */
    V4(u8, u8, u8, u8),
    V6(String),
    /*
     * Variante con una estructura anónima
     */
    IpV4Mask { host: (u8, u8, u8, u8), mask: String }
}

/*
 * Como las estructuras, las `enum` pueden tener bloques de implementación `impl` esto permite
 * encapsular la lógica relacionada con cada variante dentro de la `enum`
 */
impl IpVersionAddress {
    /*
     * `&self` permite que `get_ip` use ip_address sin tomar la propiedad, por lo que podemos seguir
     * usando la instancia después de la llamada.
     */
    fn get_ip(&self) {
        println!("Ip version: {:?}", self);
    }
}

fn main() {
    /*
     * Instanciación de una de las variantes de la `enum`, se accede a la variante con el operador
     * `::` y el nombre de la variante
     */
    let ip = IpVersionAddress::VersionFour;
    println!("Ip version: {:?}", ip);
    let localhost = IpVersionAddress::V4(127, 0, 0, 1);
    println!("Localhost: {:?}", localhost);
    let ipv6 = IpVersionAddress::V6(String::from("2001:0db8:85a3:08d3:1319:8a2e:0370:7334"));
    println!("Ip version: {:?}", ipv6);
    let ip_close = IpVersionAddress::IpV4Mask {
        host: (127, 0, 0, 1),
        mask: String::from("255.255.255.0")
    };
    println!("Ip version: {:?}", ip_close);
    ip_close.get_ip();
    /*
     * Aquí `ip_close` sigue siendo valida
     */
    localhost.get_ip();
}
