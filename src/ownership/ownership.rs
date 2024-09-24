/*
 * En `Rust` se utiliza dos áreas de memoria para almacenar datos, uno es el `stack` que es una
 * zona de memoria donde los datos son almacenados de forma contigua, para datos donde su tamaño
 * es conocido en tiempo de complilación
 *  +---------+
 *  |---------|
 *  |  STACK  |
 *  |---------|
 *  +---------+
 * 
 * La otra área de memoria es el `heap` donde se almacenan datos que su valor no es conocido y/o
 * dinámico, que puede cambiar en tiempo de ejecución. `Rust` busca un espacio en la memoria que
 * sea lo suficientemente grande para guardar el dato y puede ser en cualquier parte
 * 
 *      oooo
 *   oo      oo
 *  o   HEAP   o
 *   oo      oo
 *      oooo
 */

 fn ownership() {
    /*
     * Las variables en `Rust` son dueñas de los recursos que manejan. El `ownership` es la forma
     * en la que `Rust` gestiona su memoria de forma automática sin la necesidad de un `garbage collector`,
     * esta ligada al `scope`, cuando una varibale sale del ambito `Rust` libera el recurso 
     * 
     * 1) Cada valor en `Rust` tiene una variable llamada `owner`
     * 2) Un valor solo puede tener un `owner` a la vez
     * 3) Cuando un `owner` sale del `scope`, el valor es eliminado
     */
    let s1 = String::from("Hello");
    /*
     * La variable `s1` transfirio su valor a `s2` ahora `s2` es la variable dueña del valor `Hello`
     * y la variable original dejo de ser accesible
     */
    let s2 = s1;
    /*
     * Aquí `s1` ya no es accesible
     */
    println!("Owner value: {}, s2 value: {}", 1, s2);

    /*
     * Para los tipos primitivos `Rust` clona/copia los valores lo que permite que las variables
     * sigan siendo accesibles
     */
    let choice = false;
    let x = 10;
    let y = x;
    println!("X: {}, and choice {}", x, choice);
    println!("Y: {}, and choice {}", y, !choice);
    scope_out();
}

fn scope_out() {
    /*
     * Un ámbito es el rango dentro de un programa para el que un elemento es válido. La variable
     * `var` solo es accesible en el `scope` de la función `scope_out()`
     */
    let var = 10;
    println!("{}", var);
}