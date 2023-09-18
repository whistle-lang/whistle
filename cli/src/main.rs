use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::time::Instant;
use std::{fs, sync::Arc};
use tokio::sync::RwLock;

mod lsp;
mod util;


use lsp::WhistleBackend;

use tower_lsp::{LspService, Server};

#[derive(Debug, Parser)]
#[command(name = "whistle")]
#[command(author = "The Whistle Authors")]
#[command(about = "Next gen Whistle CLI", long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
  /// Lexes the file
  Lex {
    /// input
    #[arg(value_name = "INPUT")]
    path: String,
  },

  /// Parses the file
  Parse {
    /// input
    #[arg(value_name = "INPUT")]
    path: String,
  },

  /// compiles and runs the code
  #[command(arg_required_else_help = true)]
  Run {
    path: String,
  },

  /// compiles the file
  Compile {
    /// input
    #[arg(value_name = "INPUT")]
    path: String,
    /// output file
    #[arg(short = 'o', long = "output", value_name = "OUTPUT")]
    output: Option<String>,
  },

  /// launches the language Server
  Lsp,
}

#[tokio::main]
async fn main() {
  let args = Cli::parse();

  match args.command {
    Commands::Lex { path } => {
      let now = Instant::now();
      let text = fs::read_to_string(path).expect("Something went wrong, we can't read this file.");
      let (_, tokens) = util::preprocess(&text, false);
      println!("{:#?}", tokens);
      println!(
        "Operation complete! Took us about {} seconds.",
        now.elapsed().as_secs_f64()
      );
    }

    Commands::Parse { path } => {
      let now = Instant::now();
      let text = fs::read_to_string(path).expect("Something went wrong, we can't read this file.");
      let ast = util::parse(&text, false);
      println!("{:#?}", ast);
      println!(
        "Operation complete! Took us about {} seconds.",
        now.elapsed().as_secs_f64()
      );
    }

    Commands::Run { path } => {
      let text = fs::read_to_string(path).expect("Something went wrong, we can't read this file.");
      let bytes = util::compile(&text);
      let engine = wasmtime::Engine::default();
      let mut linker = wasmtime::Linker::new(&engine);
      wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();
      let wasi = wasmtime_wasi::WasiCtxBuilder::new()
          .inherit_stdio()
          .inherit_args().unwrap()
          .build();
      let mut store = wasmtime::Store::new(&engine, wasi);
  
      let module = wasmtime::Module::new(&engine, &bytes).unwrap();
      linker.module(&mut store, "", &module).unwrap();
      linker
          .get_default(&mut store, "").unwrap()
          .typed::<(), ()>(&store).unwrap()
          .call(&mut store, ()).unwrap();  
    }

    Commands::Compile { path, output } => {
      let now = Instant::now();
      let output = output.unwrap_or(path.replace(".whi", ".wasm"));
      let text = fs::read_to_string(path).expect("Something went wrong, we can't read this file.");
      let bytes = util::compile(&text);
      if output.ends_with(".wat") {
        let wasm_text = wasmprinter::print_bytes(&bytes).unwrap();
        fs::write(output, wasm_text.as_bytes())
          .expect("Something went wrong, we can't write this file.");
      } else {
        fs::write(output, bytes).expect("Something went wrong, we can't write this file.");
      }
      println!(
        "Operation complete! Took us about {} seconds.",
        now.elapsed().as_secs_f64()
      );
    }

    Commands::Lsp => {
      tracing_subscriber::fmt().init();
      let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
      let (service, socket) = LspService::new(|client| WhistleBackend {
        client,
        document_map: Arc::new(RwLock::new(HashMap::new())),
      });
      Server::new(stdin, stdout, socket).serve(service).await;
    }
  }
}
