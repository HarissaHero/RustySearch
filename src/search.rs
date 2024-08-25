use crate::context::Word;
use crate::occurence::Occurence;

pub fn search_word(word: Word, file_content: &str) -> Vec<Occurence> {
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

                occurences.push(Occurence::new(
                    line_number,
                    column_number,
                    line_words[start_column..end_column].join(" "),
                ));
            }
        }
    }

    occurences
}

#[test]
fn test_search_word() {
    let word = "word";
    let file_content = "This is a test file with the word word in it";
    let result = search_word(word, file_content);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].line(), 0);
    assert_eq!(result[0].column(), 7);
    assert_eq!(result[0].preview(), "a test file with the word word in it");
    assert_eq!(result[1].line(), 0);
    assert_eq!(result[1].column(), 8);
    assert_eq!(result[1].preview(), "test file with the word word in it");
}
