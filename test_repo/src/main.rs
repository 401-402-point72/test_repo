pub mod my_module {
    pub fn greeting() -> String {
        "Hello World!".to_string()
    }
}

fn main() {


    
    let message = my_module::greeting();
    println!("{}", message);
}

#[test]
fn test_greeting() {
    let result = my_module::greeting();
    assert_eq!(result, "Hello World!");
}
