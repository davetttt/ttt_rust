use super::io::*;

#[test]
fn can_inject_answer_to_prompt_into_test_io() {
    let question = "some question".to_string();
    let io = TestIo::new("answer".to_string());
    assert_eq!("answer", io.prompt(question).as_slice());
}

