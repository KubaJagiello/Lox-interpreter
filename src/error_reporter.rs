pub struct ErrorReporter {
    had_error: bool,
}

impl ErrorReporter {
    pub fn new() -> ErrorReporter {
        ErrorReporter { had_error: false }
    }

    pub fn error(&mut self, line: u32, message: String) {
        eprintln!("[line {}] Error: {}", line, message);
        self.set_had_error(true);
        //self.report(line, "".into(), message);
    }

    pub fn report(&mut self, line: u32, where_in_code: String, message: String) {
        eprintln!("[line {}] Error {}: {}", line, where_in_code, message);
        self.set_had_error(true);
        // self.had_error = false;
        // self.error_reporter.borrow_mut().set_had_error(false);
    }

    pub fn had_error(&self) -> bool {
        self.had_error
    }

    pub fn set_had_error(&mut self, had_error: bool) {
        self.had_error = had_error;
    }
}
