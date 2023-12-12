use std::io::Write;

use regex::Regex;
use remarklib::{ExecutionContext, ReplacementResult, Rule};

fn request_line() -> String {
    let mut input = String::new();
    print!(">>> ");
    std::io::stdout().flush().expect("could not flush stdout");
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read a line from the input");
    match input.strip_suffix("\n") {
        None => input,
        Some(stripped_input) => stripped_input.into(),
    }
}

fn main() {
    let mut execution_context = ExecutionContext {
        rules: [Rule::Builtin {
            pattern: Regex::new(r"\(define (.+);(.+)\)").unwrap(),
            replacer: Box::new(|captures: &regex::Captures| {
                let pattern = captures.get(1).unwrap().as_str();
                let replacement = captures.get(2).unwrap().as_str();
                ReplacementResult {
                    substitution: "".into(),
                    new_rule: Some(Rule::Regex {
                        pattern: Regex::new(pattern).unwrap(),
                        replacement: replacement.into(),
                    }),
                }
            }),
        }]
        .into(),
        program: request_line(),
    };
    loop {
        remarklib::execute(&mut execution_context);
        println!("{}", execution_context.program);
        execution_context.program = request_line();
    }
}
