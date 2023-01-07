#![allow(dead_code)]

use crate::lox::Lox;
use std::env;
use std::io::Error;

mod error_reporter;
mod lox;
mod scanner;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();
    match args.len() {
        l if l > 2 => {
            println!("Usage: jlox [script]");
        }
        2 => {
            lox.run_file(&args[0])?;
        }
        _ => {
            lox.run_prompt()?;
        }
    }
    Ok(())
}
