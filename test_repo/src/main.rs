pub fn greeting() -> String {
    "Hello World!".to_string()
}

fn main() {
    let message = greeting();
    println!("{}", message);
}
