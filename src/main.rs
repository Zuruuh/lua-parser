use clap::Parser;
use std::{fs, path::PathBuf, process};

mod lexer;
use crate::lexer::lexer::Lexer;

#[derive(Clone, Parser, Debug)]
#[command(author, version)]
struct Args {
    #[arg()]
    file: PathBuf,
}

fn main() {
    let args = Args::parse();

    if !args.file.exists() {
        println!("File \"{}\" does not exists!", args.file.to_str().unwrap());

        process::exit(1);
    }

    let content = fs::read_to_string(args.file).unwrap();
    let mut lexer = Lexer::new();
    let tokens = lexer.lex(content);

    println!("{:?}", tokens);
}
