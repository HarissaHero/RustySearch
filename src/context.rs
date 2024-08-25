pub type Word<'a> = &'a str;
pub type File<'a> = &'a str;

pub struct Context<'a> {
    word: Word<'a>,
    file: File<'a>,
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

    pub fn word(&self) -> Word<'a> {
        self.word
    }

    pub fn file(&self) -> File<'a> {
        self.file
    }
}

pub fn build_context(args: Vec<&str>) -> Context {
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
