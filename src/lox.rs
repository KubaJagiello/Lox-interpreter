use crate::scanner::Scanner;
use std::fs::File;
use std::io::{stdin, stdout, Error, Read, Write};

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox { had_error: false }
    }

    pub fn run_file(&mut self, path: &str) -> Result<(), Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        self.run(&contents);
        if self.had_error {
            eprintln!("failed to execute the given file");
            // exit here
        }
        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<(), Error> {
        loop {
            print!("> ");
            stdout().flush().unwrap();
            let mut line = String::new();
            stdin().read_line(&mut line)?;
            self.run(&line);
            self.had_error = false;
        }
    }

    fn run(&mut self, source: &str) {
        let mut scanner = Scanner::new(source.into());
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
        }
    }

    pub fn error(line: u32, message: String) {
        eprintln!("[line {}] Error: {}", line, message);
        //self.report(line, "".into(), message);
    }

    pub fn report(&mut self, line: u32, where_in_code: String, message: String) {
        eprintln!("[line {}] Error {}: {}", line, where_in_code, message);
        self.had_error = true;
    }
}
