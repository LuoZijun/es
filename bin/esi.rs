#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rustyline;

use rustyline::error::ReadlineError;


use std::env;
use std::fs;
use std::io::{self, Write};

const PS1: &str = ">> ";
const HISTORY: &str = ".history";




#[allow(dead_code)]
#[deprecated(note="please use `new_method` instead")]
fn test_deprecated(){ }


pub fn repl() {
    let mut rl = rustyline::Editor::<()>::new();
    match fs::metadata(HISTORY) {
        Ok(metadata) => {
            if metadata.is_file() {
                if rl.load_history(HISTORY).is_err() {
                    writeln!(io::stderr(), "Error: unable to load history on startup").unwrap();
                }
            }
        },
        Err(_e) => {
            fs::write(HISTORY, b"").unwrap();
            if rl.load_history(HISTORY).is_err() {
                writeln!(io::stderr(), "Error: unable to load history on startup").unwrap();
            }
        }
    }

    println!("ECMAScript 2017 (default, 06/14/2018, 06:41:53)");
    println!("[rustc 1.28.0-nightly (a805a2a5e 2018-06-10)] on darwin");
    println!(r#"Type "help", "copyright", "credits" or "license" for more information."#);
    
    loop {
        match rl.readline(PS1) {
            Ok(line) => {
                if line.len() > 0 {
                    println!("Input: {:?}", line);
                    rl.add_history_entry(line.as_ref());
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    
    if rl.save_history(HISTORY).is_err() {
        writeln!(io::stderr(), "Error: unable to save history on exit").unwrap();
    }
}


fn main() {
    env::set_var("RUST_LOG", "ecmascript=debug, esi=debug");
    env_logger::init();
    
    info!("ECMAScript interpreter init ...");
    repl();
}