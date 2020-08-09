extern crate clap;

use clap::{Arg, App, AppSettings, SubCommand};
use std::fs;
use whistle_core::lexer::*;
use std::time::{Instant};

fn main() {
    let intro = "          ▄▄▄▄▄▄▄▄▄           
       ▄████████████▀▀        
    ▄██████▀▀▀     ▄▄▄▄▄▄        
   ▄████▀  ▄▄███████████▀▀       
  ████▀ ▄████▀▀▀▀▀      ▄▄▄▄     
 ▄███▀ ▄██▀  ▄▄▄▄ ▀███▄ ▀███▄    ██╗    ██╗██╗  ██╗██╗███████╗████████╗██╗     ███████╗
 ████  ██▀ ▄██████  ███  ████    ██║    ██║██║  ██║██║██╔════╝╚══██╔══╝██║     ██╔════╝
 ████ ███  ████████ ███  ████    ██║ █╗ ██║███████║██║███████╗   ██║   ██║     █████╗  
 ████  ███  █████▀  ███  ████    ██║███╗██║██╔══██║██║╚════██║   ██║   ██║     ██╔══╝  
 ▀███▄ ▀███▄  ▀▀  ▄███  ████     ╚███╔███╔╝██║  ██║██║███████║   ██║   ███████╗███████╗
  ▀███▄  ▀██████████▀  ████▀      ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝╚══════╝   ╚═╝   ╚══════╝╚══════╝ 
   ▀████▄  ▀▀▀▀▀▀▀  ▄▄████▀      
     ▀██████▄▄▄▄▄▄██████▀        One hella programming language.
       ▀▀████████████▀▀          Made with <3 by the Whistle Team.
            ▀▀▀▀▀▀              ";
    
    let tokenize_option = SubCommand::with_name("tokenize")
        .about("tokenize [file]")
        .arg(Arg::with_name("pretty")
            .short("p")
            .help("Pretty print the tokens/program"))
        .arg(Arg::with_name("file")
            .help("Sets the input file to use")
            .required(true));

    let run_option = SubCommand::with_name("run")
        .about("run [file]")
        .arg(Arg::with_name("file")
            .help("Sets the input file to use")
            .required(true));

    let app = App::new(intro)
            .setting(AppSettings::ArgRequiredElseHelp)
            .version("v0.0.1")
            .subcommand(run_option)
            .subcommand(tokenize_option)
            .get_matches();

    if let Some(command) = &app.subcommand_name() {
        let text = readfile(&app, command);
        match command {
            &"run" => run(text),
            &"tokenize" => tokenize(text),
            _ => println!("Unreachable")
        };
    }
}

fn readfile(app: &clap::ArgMatches, command: &str) -> String{
    if let Some(matches) = app.subcommand_matches(command) {
        let file = matches.value_of("file")
            .expect("This argument can't be empty, we said it was required.");
        let text = fs::read_to_string(file)
            .expect("Something went wrong, we can't read this file.");
        return text
    }
    return String::from("")
}

fn run(text: String) {
    let lexer = Lexer::new(text);
    let now = Instant::now();
    for tok in lexer {
        println!("{:?}", tok);
        if tok.is_err() {
          break;
        }
    }
    println!("Operation complete! Took us about {} seconds.", now.elapsed().as_secs_f64());
}

fn tokenize(text: String) {
    let lexer = Lexer::new(text);
    let now = Instant::now();
    for tok in lexer {
        println!("{:?}", tok);
        if tok.is_err() {
          break;
        }
    }
    println!("Operation complete! Took us about {} seconds.", now.elapsed().as_secs_f64());
}