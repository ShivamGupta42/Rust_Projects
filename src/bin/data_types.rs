#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

//#[allow(dead_code)]
fn main() {
    let name = String::from("Shivam");
    let age = 27;
    let person = Person { name, age };
    println!("{:?}",person)
}