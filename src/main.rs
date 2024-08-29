use std::sync::{Arc, Mutex};

use rustysearch::{
    args::extract_and_process_args, context::build_context, occurence::display_results,
    search::search_word,
};

fn main() {
    let timestamp = std::time::Instant::now();
    let args: Vec<String> = std::env::args().collect();
    let extracted_arguments = extract_and_process_args(args);
    let contexts = build_context(
        &extracted_arguments.word,
        extracted_arguments.file.as_deref(),
        extracted_arguments.dir.as_deref(),
    );

    let mut threads = vec![];

    let contexts_mutex = Arc::new(Mutex::new(contexts));

    for _ in 0..extracted_arguments.num_threads {
        let mutex = contexts_mutex.clone();
        let thread = std::thread::spawn(move || {
            let mut results = vec![];
            loop {
                let mut contexts = mutex.lock().unwrap();
                match contexts.pop() {
                    Some(context) => {
                        let context_clone = context.clone();
                        drop(contexts);
                        let file_content = std::fs::read_to_string(context_clone.file()).unwrap();
                        let occurences = search_word(context.word(), &file_content);
                        results.push((context_clone, occurences));
                    }
                    None => {
                        break;
                    }
                };
            }
            results
        });
        threads.push(thread);
    }

    for thread in threads {
        let results = thread.join().unwrap();
        for (context, occurences) in results {
            display_results(context, occurences);
        }
    }

    println!("Elapsed time: {:?}", timestamp.elapsed());
}
