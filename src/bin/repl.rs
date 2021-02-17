extern crate k;
use k::lexer::Lexer;

extern crate rustyline;

extern crate colored;
use colored::*;

static BINARY_VERSION: &'static str = env!("CARGO_PKG_VERSION");
static PROMPT: &'static str = ">>> ";

fn main() {
    println!("K Programming Language {} (written by Kosi Nwabueze)", BINARY_VERSION.blue());
    println!(r#"Type "{}" or "{}" for more information."#, "help".yellow(), "license".yellow());
    let mut rl = rustyline::Editor::<()>::new();

    loop {
        let readline = rl.readline(PROMPT);

        match readline {
            Ok(line) => {
                let lexer = Lexer::new(&line);
                for token in lexer {
                    println!("{:?}", token);
                }
            }
            Err(_) => break
        }
    }
}
