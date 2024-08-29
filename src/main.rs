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

    for context in contexts {
        let thread = std::thread::spawn(move || {
            let file_content = std::fs::read_to_string(context.file()).unwrap();
            let occurences = search_word(context.word(), &file_content);
            (context, occurences)
        });
        threads.push(thread);
    }

    for thread in threads {
        let (context, occurences) = thread.join().unwrap();
        display_results(context, occurences);
    }

    println!("Elapsed time: {:?}", timestamp.elapsed());
}
