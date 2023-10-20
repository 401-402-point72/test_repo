use src::greeting;

#[test]
fn test_greeting() {
    let result = greeting();
    assert_eq!(result, "Hello World!");
}