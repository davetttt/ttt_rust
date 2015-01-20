use std::io::BufferedReader;
use std::io::stdin;

pub trait Io {
    fn print(&self, statement: String);
    fn prompt(&self, question: String) -> String;
}

pub struct TestIo {
    answer: String
}

impl TestIo {
    pub fn new(answer: String) -> TestIo {
        TestIo { answer: answer }
    }
}

impl Io for TestIo {
    fn print(&self, statement: String) {}

    fn prompt(&self, question: String) -> String {
        self.answer.clone()
    }
}

#[derive(Clone)]
pub struct ConsoleIo;

impl ConsoleIo {
    pub fn new() -> ConsoleIo {
        ConsoleIo
    }
}

impl Io for ConsoleIo {
    fn print(&self, statement: String) {
        print!("{}", statement);
    }

    fn prompt(&self, question: String) -> String {
        let mut stdin = BufferedReader::new(stdin());
        let answer: String;
        print!("{}", question);
        match stdin.read_line() {
            Ok(line) => answer = line,
            Err(e)   => panic!(e)
        };
        answer
    }
}

