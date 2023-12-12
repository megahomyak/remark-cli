use regex::Regex;
use remarklib::{ExecutionContext, ReplacementResult, Rule};
use rustyline::error::ReadlineError;

fn builtin(
    pattern: &str,
    replacer: impl Fn(&regex::Captures) -> ReplacementResult + 'static,
) -> Rule {
    Rule::Builtin {
        pattern: Regex::new(pattern).unwrap(),
        replacer: Box::new(replacer),
    }
}

fn get<'a>(captures: &'a regex::Captures, index: usize) -> &'a str {
    captures.get(index).unwrap().as_str()
}

fn main() {
    let mut editor = rustyline::DefaultEditor::new().unwrap();
    let mut execution_context = ExecutionContext {
        rules: [
            builtin(r"\(define (.+);(.+)\)", |captures| {
                let pattern = get(captures, 1);
                let replacement = get(captures, 2);
                match Regex::new(pattern) {
                    Ok(pattern) => ReplacementResult {
                        substitution: "".into(),
                        new_rule: Some(Rule::Regex {
                            pattern,
                            replacement: replacement.into(),
                        }),
                    },
                    Err(error) => ReplacementResult {
                        substitution: format!("{{REGEX ERROR: {}}}", error),
                        new_rule: None,
                    },
                }
            }),
            builtin(r"\((\d+) \- (\d+)\)", |captures| {
                let minuend: isize = get(captures, 1).parse().unwrap();
                let subtrahend: isize = get(captures, 2).parse().unwrap();
                ReplacementResult {
                    substitution: format!("{}", minuend - subtrahend),
                    new_rule: None,
                }
            }),
        ]
        .into(),
    };
    let mut program;
    loop {
        program = match editor.readline(">>> ") {
            Ok(line) => {
                editor
                    .add_history_entry(&line)
                    .expect("could not add a history entry");
                line
            }
            Err(ReadlineError::Eof | ReadlineError::Interrupted) => break,
            Err(other_error) => panic!("{}", other_error),
        };
        program = remarklib::execute(&mut execution_context, program);
        if !program.is_empty() {
            println!("{}", program);
        }
    }
}
