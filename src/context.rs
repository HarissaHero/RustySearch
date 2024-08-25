use std::{fs, path::Path};

pub type Word<'a> = &'a str;
pub type File = String;

pub struct Context<'a> {
    word: Word<'a>,
    file: File,
}

impl<'a> Context<'a> {
    fn new(word: Word<'a>, file: File) -> Self {
        Self { word, file }
    }

    pub fn word(&self) -> Word<'a> {
        self.word
    }

    pub fn file(&self) -> &str {
        &self.file
    }

    fn validate(&self) {
        if self.word.is_empty() || self.file.is_empty() {
            panic!("Invalid argument")
        }
    }
}

pub fn build_context(args: Vec<&str>) -> Vec<Context> {
    let mut args = args.into_iter();
    let _ = args.next();
    let word = args.next().unwrap();
    let mut contexts: Vec<Context> = vec![];

    while args.len() > 0 {
        let current_arg = args.next().unwrap();
        match current_arg {
            "-f" | "--file" => {
                let file = args.next().unwrap();
                let context = Context::new(word, file.to_string());
                context.validate();
                contexts.push(context);
            }
            "-d" | "--directory" => {
                let directory = Path::new(args.next().unwrap());
                if directory.is_dir() {
                    let dir_content = fs::read_dir(directory).unwrap();
                    for entry in dir_content {
                        let entry = entry.unwrap();
                        let path = entry.path();
                        if path.is_file() {
                            let file = path.to_str().unwrap();
                            let context = Context::new(word, file.to_string());
                            context.validate();
                            contexts.push(context);
                        }
                    }
                } else {
                    panic!("Not a directory");
                }
            }
            _ => {
                panic!("Unknown argument: {}", current_arg);
            }
        }
    }

    if contexts.is_empty() {
        panic!("No files to search");
    }

    contexts
}

#[test]
fn test_check_args() {
    let args = vec!["target/debug/rustysearch", "word", "-f", "file.txt"];
    let result = build_context(args);
    assert_eq!(result[0].word(), "word");
    assert_eq!(result[0].file(), "file.txt");
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
fn test_check_args_directory() {
    let args = vec!["target/debug/rustysearch", "word", "-d", "./test/"];
    let result = build_context(args);

    assert_eq!(result.len(), 3);

    let another_file = result
        .iter()
        .find(|context| context.file() == "./test/another-file.txt")
        .unwrap();
    let poem = result
        .iter()
        .find(|context| context.file() == "./test/poem.txt")
        .unwrap();
    let readme_test = result
        .iter()
        .find(|context| context.file() == "./test/readme-test.txt")
        .unwrap();

    assert_eq!(another_file.word(), "word");
    assert_eq!(another_file.file(), "./test/another-file.txt");
    assert_eq!(poem.word(), "word");
    assert_eq!(poem.file(), "./test/poem.txt");
    assert_eq!(readme_test.word(), "word");
    assert_eq!(readme_test.file(), "./test/readme-test.txt");
}
