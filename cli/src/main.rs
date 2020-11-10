extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};
use std::fs;
use std::time::Instant;
use whistle_core::lexer::*;
use whistle_core::parser::*;
use whistle_core::version;

fn main() {
  let intro = "
            ▄▄▄▄▄▄▄▄▄           
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

  let lex_option = SubCommand::with_name("lex")
    .about("lex [file]")
    .arg(
      Arg::with_name("pretty")
        .short("p")
        .help("Pretty print the tokens/program"),
    )
    .arg(
      Arg::with_name("output")
        .takes_value(true)
        .short("o")
        .help("Output the result to a file"),
    )
    .arg(
      Arg::with_name("file")
        .help("Sets the input file to use")
        .required(true),
    );

  let parse_option = SubCommand::with_name("parse")
    .about("parse [file]")
    .arg(
      Arg::with_name("pretty")
        .short("p")
        .help("Pretty print the tokens/program"),
    )
    .arg(
      Arg::with_name("output")
        .takes_value(true)
        .short("o")
        .help("Output the result to a file"),
    )
    .arg(
      Arg::with_name("file")
        .help("Sets the input file to use")
        .required(true),
    );

  let run_option = SubCommand::with_name("run").about("run [file]").arg(
    Arg::with_name("file")
      .help("Sets the input file to use")
      .required(true),
  );

  let app = App::new(intro)
    .setting(AppSettings::ArgRequiredElseHelp)
    .version(&*format!(
      "cli {}, core {}",
      env!("CARGO_PKG_VERSION"),
      version()
    ))
    .subcommand(run_option)
    .subcommand(lex_option)
    .subcommand(parse_option)
    .get_matches();

  if let Some(command) = app.subcommand_name() {
    if let Some(matches) = app.subcommand_matches(command) {
      let file = matches
        .value_of("file")
        .expect("This argument can't be empty, we said it was required.");
      let text = fs::read_to_string(file).expect("Something went wrong, we can't read this file.");
      let output = matches.value_of("output");

      match command {
        "lex" => lex(&text, output),
        "parse" => parse(&text, output),
        _ => println!("Unreachable"),
      };
    }
  }
}

fn lex(text: &str, output: Option<&str>) {
  let lexer = Lexer::new(text);
  let now = Instant::now();
  let mut toks = Vec::new();
  for tok in lexer {
    toks.push(tok.clone());

    if tok.is_err() {
      break;
    }
  }

  if let Some(file) = output {
    fs::write(file, format!("{:#?}", toks))
      .expect("Something went wrong, we can't write this file.");
  }

  println!(
    "Operation complete! Took us about {} seconds.",
    now.elapsed().as_secs_f64()
  );
}

fn parse(text: &str, output: Option<&str>) {
  let lexer = Lexer::new(text);
  let mut parser = Parser::from(lexer);
  let now = Instant::now();
  let res = parse_grammar(&mut parser);

  if let Some(file) = output {
    fs::write(file, format!("{:#?}", res))
      .expect("Something went wrong, we can't write this file.");
  }

  println!(
    "Operation complete! Took us about {} seconds.",
    now.elapsed().as_secs_f64()
  );
}
