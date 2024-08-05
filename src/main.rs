pub mod lexer;
pub mod logger;
pub mod source;
pub mod utils;

use lexer::lexer::Lexer;
use logger::Logger;
use source::Source;
use std::{env, path::PathBuf};

fn set_up_logger() {
    let pwd: PathBuf = env::current_dir().unwrap_or_else(|e| {
        panic!("Error getting current directory: {}", e);
    });
    let logger_file_path = pwd.join(".log").join("debug.log");
    let _logger = Logger::new(logger_file_path);
}

fn main() {
    set_up_logger();

    let usage_message: &str = "Usage: \n\
                               funs <file.fs>";
    let args: &[String] = &env::args().collect::<Vec<String>>()[1..];
    if args.len() != 1 {
        println!("{}", usage_message);
        return;
    }

    let file_path: &str = &args[0];
    let source = Source::new(file_path);
    let lexer = Lexer::new(&source);
    let tokens = lexer.collect::<Vec<_>>();
    println!("{:?}", tokens);
}
