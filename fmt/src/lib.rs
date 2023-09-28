#![feature(array_chunks)]
mod config;
mod gen;
mod utils;
use anyhow::Result;
use config::Configuration;
use dprint_core::configuration::resolve_new_line_kind;
use dprint_core::formatting::*;
use gen::generate;
use std::path::Path;
use whistle_ast::Grammar;
use whistle_parser::Parser;

pub struct ParsedSource {
  pub parser: Parser,
  pub grammar: Grammar,
  pub text: String,
}

pub fn format_text(
  file_path: &Path,
  file_text: &str,
  config: &Configuration,
) -> Result<Option<String>> {
  if utils::file_text_has_ignore_comment(file_text, "// whistle-ignore-file") {
    Ok(None)
  } else {
    let parsed_source = utils::parse(file_path, file_text, false);
    let parsed_source = ParsedSource {
      parser: parsed_source.0,
      grammar: parsed_source.1,
      text: file_text.to_string(),
    };
    inner_format(&parsed_source, config)
  }
}

/// Formats an already parsed source. This is useful as a performance optimization.
pub fn format_parsed_source(
  source: ParsedSource,
  config: &Configuration,
) -> Result<Option<String>> {
  if utils::file_text_has_ignore_comment(&source.text, "// whistle-ignore-file") {
    Ok(None)
  } else {
    // ensure_no_specific_syntax_errors(source)?;
    inner_format(&source, config)
  }
}

fn inner_format(parsed_source: &ParsedSource, config: &Configuration) -> Result<Option<String>> {
  let result = dprint_core::formatting::format(
    || {
      #[allow(clippy::let_and_return)]
      let print_items = generate(parsed_source, config);
      println!("{}", print_items.get_as_text());
      print_items
    },
    config_to_print_options(parsed_source.text.as_str(), config),
  );
  if result == parsed_source.text.as_str() {
    Ok(None)
  } else {
    Ok(Some(result))
  }
}

fn config_to_print_options(file_text: &str, config: &Configuration) -> PrintOptions {
  PrintOptions {
    indent_width: config.indent_width,
    max_width: config.line_width,
    use_tabs: config.use_tabs,
    new_line_text: resolve_new_line_kind(file_text, config.new_line_kind),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::path::PathBuf;

  #[test]
  fn test_format_text() {
    let file_path = PathBuf::from("test.whistle");
    let file_text = r#"
      // whistle-ignore-file
      export fn hi(): i32 {
        return 5
      }
    "#;
    let config = Configuration {
      indent_width: 2,
      line_width: 80,
      use_tabs: false,
      new_line_kind: dprint_core::configuration::NewLineKind::LineFeed,
    };
    let result = format_text(&file_path, file_text, &config).unwrap();
    assert_eq!(result, None);
  }

  #[test]
  fn test_format_text2() {
    let file_path = PathBuf::from("test.whistle");
    let file_text = r#"
            export fn hi(): i32 {
            return 5
            }
        "#;
    let config = Configuration {
      indent_width: 2,
      line_width: 80,
      use_tabs: false,
      new_line_kind: dprint_core::configuration::NewLineKind::LineFeed,
    };
    let result = format_text(&file_path, file_text, &config).unwrap();
    assert_eq!(
      result,
      Some("export fn hi(): i32 {\n  return 5\n}\n".to_string())
    );
  }
}
