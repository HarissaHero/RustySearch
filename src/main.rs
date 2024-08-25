use rustysearch::{context::build_context, occurence::display_results, search::search_word};

fn main() {
    let timestamp = std::time::Instant::now();
    let args: Vec<String> = std::env::args().collect();
    let contexts = build_context(args.iter().map(|arg| arg.as_str()).collect());

    for context in contexts {
        let file_content = std::fs::read_to_string(context.file()).unwrap();
        let occurences = search_word(context.word(), &file_content);
        display_results(context, occurences);
    }

    println!("Elapsed time: {:?}", timestamp.elapsed());
}
