//! CLI binary for parsing Rust expressions and outputting structured JSON.
//!
//! Usage:
//!   syn-expr-json "expression"
//!
//! Example:
//!   syn-expr-json "1 + 2 * 3"

use std::env;
use std::process;

use promql_parser_js::expr_to_json;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <expression>", args[0]);
        eprintln!();
        eprintln!("Parse a Rust expression and output structured JSON.");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  {} \"1 + 2 * 3\"", args[0]);
        eprintln!("  {} \"foo.bar(baz)\"", args[0]);
        eprintln!("  {} \"if x > 0 {{ x }} else {{ -x }}\"", args[0]);
        process::exit(1);
    }

    let input = &args[1];

    match syn::parse_str::<syn::Expr>(input) {
        Ok(expr) => {
            let json = expr_to_json(&expr);
            match serde_json::to_string_pretty(&json) {
                Ok(output) => println!("{}", output),
                Err(e) => {
                    eprintln!("Error serializing JSON: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
            process::exit(1);
        }
    }
}
