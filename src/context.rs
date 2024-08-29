use std::{fs, path::Path};

pub type Word = String;
pub type File = String;

pub struct Context {
    word: Word,
    file: File,
}

impl Context {
    fn new(word: Word, file: File) -> Self {
        Self { word, file }
    }

    pub fn word(&self) -> &str {
        &self.word
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

fn create_context(word: &str, file: &str) -> Context {
    let context = Context::new(word.to_string(), file.to_string());
    context.validate();
    context
}

pub fn build_context(word: &str, file: Option<&str>, dir: Option<&str>) -> Vec<Context> {
    let mut contexts: Vec<Context> = vec![];

    if file.is_some() {
        let context = create_context(word, file.unwrap());
        contexts.push(context);
    }

    if dir.is_some() {
        let directory = Path::new(dir.unwrap());
        if directory.is_dir() {
            let dir_content = fs::read_dir(directory).unwrap();
            for entry in dir_content {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_file() {
                    let file = path.to_str().unwrap();
                    let context = create_context(word, file);
                    contexts.push(context);
                }
            }
        } else {
            panic!("Not a directory");
        }
    }

    if contexts.is_empty() {
        panic!("No files to search");
    }

    contexts
}

#[test]
fn test_check_args() {
    let result = build_context("word", Some("file.txt"), None);
    assert_eq!(result[0].word(), "word");
    assert_eq!(result[0].file(), "file.txt");
}

#[test]
fn test_check_args_directory() {
    let result = build_context("word", None, Some("./test/"));

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
