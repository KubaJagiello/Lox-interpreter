#![allow(dead_code)]

use crate::lox::Lox;
use std::env;
use std::io::Error;

mod lox;
mod scanner;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
    } else if args.len() == 2 {
        lox.run_file(&args[0])?;
    } else {
        lox.run_prompt()?;
    }
    Ok(())
}
