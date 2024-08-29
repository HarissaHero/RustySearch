pub struct Args {
    pub word: String,
    pub file: Option<String>,
    pub dir: Option<String>,
    pub num_threads: usize,
}

pub fn extract_and_process_args(args: Vec<String>) -> Args {
    let mut args = args.into_iter();
    let _ = args.next();
    let word = args.next().unwrap();

    let mut extracted_args = Args {
        word: word.to_string(),
        file: None,
        dir: None,
        num_threads: 1,
    };

    while args.len() > 0 {
        match args.next().as_deref() {
            Some("-f") | Some("--file") => {
                extracted_args.file = Some(args.next().unwrap().to_string());
            }
            Some("-d") | Some("--directory") => {
                extracted_args.dir = Some(args.next().unwrap().to_string());
            }
            Some("-t") | Some("--threads") => {
                let num_threads = args.next().unwrap().parse().unwrap();
                extracted_args.num_threads = num_threads;
            }
            _ => {
                panic!("Invalid argument");
            }
        }
    }

    extracted_args
}

#[test]
fn test_extract_and_process_args() {
    let args: Vec<String> = vec![
        "rustysearch".to_string(),
        "hello".to_string(),
        "-f".to_string(),
        "file.txt".to_string(),
        "-d".to_string(),
        "dir".to_string(),
        "-t".to_string(),
        "4".to_string(),
    ];
    let extracted_args = extract_and_process_args(args);

    assert_eq!(extracted_args.word, "hello");
    assert_eq!(extracted_args.file, Some("file.txt".to_string()));
    assert_eq!(extracted_args.dir, Some("dir".to_string()));
    assert_eq!(extracted_args.num_threads, 4);
}

#[test]
fn test_extract_and_process_args_invalid_arg() {
    let args = vec![
        "target/debug/rustysearch".to_string(),
        "word".to_string(),
        "-x".to_string(),
        "file.txt".to_string(),
    ];
    assert!(std::panic::catch_unwind(|| extract_and_process_args(args)).is_err());
}
