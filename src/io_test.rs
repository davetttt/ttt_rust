use super::io::*;

#[test]
fn can_call_prompt_on_test_io() {
    let io: TestIo = Io::new();
    let string = "text".to_string();
    assert_eq!("text", io.prompt(string).as_slice());
}
