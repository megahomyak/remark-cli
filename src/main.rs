use regex::Regex;
use remarklib::{ExecutionContext, ReplacementResult, Rule};
use rustyline::error::ReadlineError;

enum LineReadResult {
    UserExited,
    Success(String),
}

fn main() {
    let mut editor = rustyline::DefaultEditor::new().unwrap();
    let mut request_line = || match editor.readline(">>> ") {
        Ok(line) => {
            editor.add_history_entry(&line).expect("could not add a history entry");
            LineReadResult::Success(line)
        }
        Err(ReadlineError::Eof | ReadlineError::Interrupted) => LineReadResult::UserExited,
        Err(other_error) => panic!("{}", other_error),
    };
    let first_line = match request_line() {
        LineReadResult::Success(line) => line,
        LineReadResult::UserExited => return,
    };
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
        program: first_line,
    };
    loop {
        remarklib::execute(&mut execution_context);
        println!("{}", execution_context.program);
        execution_context.program = match request_line() {
            LineReadResult::Success(line) => line,
            LineReadResult::UserExited => break,
        }
    }
}
