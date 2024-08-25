use rustysearch::{context::build_context, occurence::display_results, search::search_word};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let context = build_context(args.iter().map(|arg| arg.as_str()).collect());

    let file_content = std::fs::read_to_string(context.file()).unwrap();

    let occurences = search_word(context.word(), &file_content);

    display_results(context, occurences);
}
