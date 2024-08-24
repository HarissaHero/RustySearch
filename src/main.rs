use std::fmt::{Display, Formatter, Result};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let context = build_context(args.iter().map(|arg| arg.as_str()).collect());

    let file_content = std::fs::read_to_string(context.file).unwrap();

    let occurences = search_word(context.word, &file_content);

    display_results(context, occurences);
}

type Word<'a> = &'a str;
type File<'a> = &'a str;

struct Context<'a> {
    word: Word<'a>,
    file: File<'a>,
}

#[derive(Debug)]
struct Occurence {
    line: usize,
    column: usize,
    preview: String,
}

impl<'a> Context<'a> {
    fn new() -> Self {
        Self { word: "", file: "" }
    }

    fn set_word(&mut self, word: Word<'a>) {
        self.word = word;
    }

    fn set_file(&mut self, file: File<'a>) {
        self.file = file;
    }

    fn validate(&self) {
        if self.word.is_empty() || self.file.is_empty() {
            panic!("Invalid argument")
        }
    }
}

impl Display for Occurence {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[{}, {}] {}", self.line, self.column, self.preview)
    }
}

fn build_context(args: Vec<&str>) -> Context {
    let mut args = args.into_iter();
    let _ = args.next();
    let word = args.next().unwrap();
    let mut context = Context::new();
    context.set_word(word);

    while args.len() > 0 {
        let current_arg = args.next().unwrap();
        match current_arg {
            "-f" | "--file" => {
                let file = args.next().unwrap();
                context.set_file(file);
            }
            _ => {
                panic!("Unknown argument: {}", current_arg);
            }
        }
    }

    context.validate();
    context
}

fn search_word(word: Word, file_content: &str) -> Vec<Occurence> {
    let mut occurences = vec![];
    for (line_number, line) in file_content.lines().enumerate() {
        let line_words: Vec<&str> = line.split_whitespace().collect();
        for (column_number, current_word) in line_words.iter().enumerate() {
            if *current_word == word {
                let start_column = if column_number > 5 {
                    column_number - 5
                } else {
                    0
                };

                let end_column = if column_number + 5 > line_words.len() {
                    line_words.len()
                } else {
                    column_number + 5
                };

                occurences.push(Occurence {
                    line: line_number,
                    column: column_number,
                    preview: line_words[start_column..end_column].join(" "),
                });
            }
        }
    }

    occurences
}

fn display_results(context: Context, occurences: Vec<Occurence>) {
    println!(
        "found {} occurences of {} in {}",
        occurences.len(),
        context.word,
        context.file
    );
    for occurence in occurences {
        println!("{}", occurence);
    }
}

#[test]
fn test_check_args() {
    let args = vec!["target/debug/rustysearch", "word", "-f", "file.txt"];
    let result = build_context(args);
    assert_eq!(result.word, "word");
    assert_eq!(result.file, "file.txt");
}

#[test]
fn test_check_args_invalid_arg() {
    let args = vec!["target/debug/rustysearch", "word", "-x", "file.txt"];
    assert!(std::panic::catch_unwind(|| build_context(args)).is_err());
}

#[test]
fn test_check_args_invalid_number_of_args() {
    let args = vec!["target/debug/rustysearch", "word"];
    assert!(std::panic::catch_unwind(|| build_context(args)).is_err());
}

#[test]
fn test_check_args_invalid_word() {
    let args = vec!["target/debug/rustysearch", "", "-f", "file.txt"];
    assert!(std::panic::catch_unwind(|| build_context(args)).is_err());
}

#[test]
fn test_check_args_invalid_file() {
    let args = vec!["target/debug/rustysearch", "word", "-f", ""];
    assert!(std::panic::catch_unwind(|| build_context(args)).is_err());
}

#[test]
fn test_search_word() {
    let word = "word";
    let file_content = "This is a test file with the word word in it";
    let result = search_word(word, file_content);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].line, 0);
    assert_eq!(result[0].column, 7);
    assert_eq!(result[0].preview, "a test file with the word word in it");
    assert_eq!(result[1].line, 0);
    assert_eq!(result[1].column, 8);
    assert_eq!(result[1].preview, "test file with the word word in it");
}
