use std::io::BufferedReader;
use std::io::stdin;

pub trait Io {
    fn new() -> Self;
    fn prompt(&self, question: String) -> String;
}

pub struct ConsoleIo;

impl Io for ConsoleIo {
    fn new() -> ConsoleIo {
        ConsoleIo
    }

    fn prompt(&self, question: String) -> String {
        let mut stdin = BufferedReader::new(stdin());
        let answer: String;

        print!("{}", question);
        match stdin.read_line() {
            Ok(line) => answer = line,
            Err(e)   => answer = self.prompt(question)
        };
        answer
    }
}

pub struct TestIo;

impl Io for TestIo {
    fn new() -> TestIo {
        TestIo
    }

    fn prompt(&self, question: String) -> String {
        question.to_string()
    }
}

