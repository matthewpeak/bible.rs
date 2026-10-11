use std::process::{Command, Output};
fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_bible"))
        .args(args)
        .env("NO_COLOR", "1")
        .output()
        .unwrap()
}
fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).unwrap()
}

#[test]
fn a_fresh_checkout_has_useful_help() {
    let output = run(&[]);
    assert!(output.status.success());
    assert!(stdout(&output).contains("bible genesis"));
    assert!(stdout(&output).contains("King James Version"));
}
#[test]
fn book_output_distinguishes_quotes_from_adaptation() {
    let output = run(&["john"]);
    assert!(output.status.success());
    let text = stdout(&output);
    assert!(text.contains("KJV / John 1:1"));
    assert!(text.contains("ADAPTATION /"));
    assert!(text.contains("6 waterpots converted"));
    assert!(!text.contains('\u{1b}'));
}
#[test]
fn all_books_and_reference_checks_execute_successfully() {
    let output = run(&["all"]);
    assert!(output.status.success());
    assert!(stdout(&output).contains("66 books executed. 600 adapted scenes."));
    assert!(run(&["all", "--full"]).status.success());
    assert!(run(&["check"]).status.success());
    assert!(run(&["demo"]).status.success());
}
#[test]
fn source_is_the_actual_compiled_book_module() {
    let output = run(&["source", "john"]);
    assert!(output.status.success());
    assert_eq!(
        stdout(&output),
        bible_rs::canon::find("john").unwrap().source
    );
}
#[test]
fn invalid_input_fails_instead_of_running_a_different_book() {
    for args in [
        vec!["not-a-book"],
        vec!["demo", "missing"],
        vec!["source"],
        vec!["all", "--oops"],
        vec!["list", "extra"],
        vec!["--help", "extra"],
    ] {
        let output = run(&args);
        assert_eq!(output.status.code(), Some(2));
        assert!(!output.stderr.is_empty());
    }
}
#[test]
fn numbered_book_names_work_with_spaces() {
    let output = run(&["1", "Samuel"]);
    assert!(output.status.success());
    assert!(stdout(&output).contains("1 SAMUEL"));
}
