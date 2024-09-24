/*
 * Un `HashMap` es una estructura de tipo clave-valor
 */
use std::collections::HashMap;


fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    let team = String::from("Blue team");
    let slice = &team[..4];
    println!("{slice}");
    let mut json: HashMap<&str, i32> = HashMap::new();
    json.insert("Blue", 32);
    let score = json.get(slice);
    println!("Score: {:?}", score);

    for (key, value) in scores {
        println!("{key}: {value}");
    }
}