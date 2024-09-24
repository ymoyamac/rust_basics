use core::fmt::Debug;
/*
 * Un `trait` define una funcionalidad que puede ser compartida por otros tipos en una manera más
 * abstracta
 */
trait Summary {
    /*
     * Un trait define un conjunto de métodos que un tipo debe de implementar para poder cumplir
     * con ese trait
     */
    fn summarize(&self) -> String;
    /*
     * También se puede tener implementaciones por defecto, todos los tipos que implementen este `trait`
     * tendrán el mismo comportamiento si es que no se sobreescribe
     */
    fn def_summ(&self) -> String {
        String::from("(Read more...)")
    }
}

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u8
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
}

impl Summary for Person {
    fn summarize(&self) -> String {
        format!("My name is {} and my age is {} years old", self.name, self.age)
    }
    /*
     * Se puede sobrescribir la función por defecto definida en el trait
     */
    fn def_summ(&self) -> String {
        format!("Hello!")
    }
}

#[derive(Debug)]
struct Tweet {
    pub username: String,
    pub content: String,
}

/*
 * El mismo `trait` puede ser implementado para distintos tipos
 */
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let person = Person::new("Yael Moya".to_string(), 24);
    let summary = person.summarize();
    println!("How are you? {summary}");
    println!("{} I am a person", person.def_summ());
    let tweet = Tweet {
        username: "@ymoyamac".to_string(),
        content: "Hello from rust!".to_string(),
    };
    let t = tweet.summarize();
    println!("What are you thinking? {t}");
    println!("Def {:?}", tweet.def_summ());
    print_summary(&person);
    print_summary(&tweet);
    print_summary_bounds(&person)
}

/*
 * Los `traits` como parámetro, solo se aceptan tipos que implementen el `trait` `Summary`, esta
 * sintaxis permite tener mayor flexibilidad en la especificación de los traits, combinar multiples
 * `traits` y realizar combinaciones más complejas
 */
fn print_summary<T: Summary /*+ Display */>(item: &T) {
    println!("{}", item.summarize())
}

/*
 * Esta es otra forma de escribir las restricciones de `traits` de una forma más clara
 */
fn print_summary_bounds<T>(item: &T)
where
    T: Summary + Debug + Clone
{
    println!("{}", item.summarize())
}