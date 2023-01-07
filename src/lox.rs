use crate::error_reporter::ErrorReporter;
use crate::scanner::Scanner;
use std::cell::RefCell;
use std::fs::File;
use std::io::{stdin, stdout, Error, Read, Write};
use std::rc::Rc;

pub struct Lox {
    error_reporter: Rc<RefCell<ErrorReporter>>,
}

impl Lox {
    pub fn new() -> Lox {
        Lox {
            error_reporter: Rc::new(RefCell::new(ErrorReporter::new())),
        }
    }

    pub fn run_file(&mut self, path: &str) -> Result<(), Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        self.run(&contents);

        if self.error_reporter.borrow().had_error() {
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
            self.error_reporter.borrow_mut().set_had_error(false);
        }
    }

    fn run(&mut self, source: &str) {
        let mut scanner = Scanner::new(source.into(), self.error_reporter.clone());
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
        }
    }
}
