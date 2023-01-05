extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};
use std::fs;
use std::time::Instant;

mod util;

const INTRO: &str = "  ▄███▀▀▀  
 ██▀ ▄▄█▀▀ ▄    █ ▄ █ █ █ █ █▀ ▀█▀ █  █▀▀
██ █▀ ▄▄ ▄ ██   █ █ █ █▀█ █ ▀█  █  █  █▀
██ █▄ ▀ ▄█ ██    ▀▀▀  ▀ ▀ ▀ ▀▀  ▀  ▀▀ ▀▀▀
 ██▄ ▀▀▀ ▄██  One hella programming language.
   ▀█████▀   Made with <3 a̶n̶d̶ ̶c̶o̶d̶e by the Whistle Team.
             ";

fn main() {
  let lex_option = SubCommand::with_name("lex")
    .about("lex [file]")
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

  let check_option = SubCommand::with_name("check")
    .about("check [file]")
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

  let compile_option = SubCommand::with_name("compile")
    .about("compile [file]")
    .arg(
      Arg::with_name("file")
        .help("Sets the input file to use")
        .required(true),
    )
    .arg(
      Arg::with_name("output")
        .takes_value(true)
        .short("o")
        .help("Output the result to a file")
        .required(true),
    );

  let app = App::new(INTRO)
    .setting(AppSettings::ArgRequiredElseHelp)
    .version(&*format!("cli {}", env!("CARGO_PKG_VERSION")))
    .subcommand(compile_option)
    .subcommand(lex_option)
    .subcommand(parse_option)
    .subcommand(check_option)
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
        "check" => check(&text, output),
        "compile" => compile(&text, output.unwrap()),
        _ => unreachable!(),
      };
    }
  }
}

fn lex(text: &str, output: Option<&str>) {
  let now = Instant::now();
  let (_, tokens) = util::preprocess(text, output.is_none());

  if let Some(file) = output {
    fs::write(file, format!("{:#?}", tokens))
      .expect("Something went wrong, we can't write this file.");
  } else {
    println!("{:#?}", tokens);
  }

  println!(
    "Operation complete! Took us about {} seconds.",
    now.elapsed().as_secs_f64()
  );
}

fn parse(text: &str, output: Option<&str>) {
  let now = Instant::now();
  let (_, grammar) = util::parse(text, false);

  if let Some(file) = output {
    fs::write(file, format!("{:#?}", grammar))
      .expect("Something went wrong, we can't write this file.");
  } else {
    println!("{:#?}", grammar);
  }

  println!(
    "Operation complete! Took us about {} seconds.",
    now.elapsed().as_secs_f64()
  );
}

fn check(text: &str, _output: Option<&str>) {
  let now = Instant::now();
  util::check(text);

  println!(
    "Operation complete! Took us about {} seconds.",
    now.elapsed().as_secs_f64()
  );
}

fn compile(text: &str, output: &str) {
  let now = Instant::now();
  let bytes = util::compile(text);

  fs::write(output, &bytes[..]).expect("Something went wrong, we can't write this file.");

  println!(
    "Operation complete! Took us about {} seconds.",
    now.elapsed().as_secs_f64()
  );
}
