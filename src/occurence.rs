use std::fmt::{Display, Formatter, Result};

use crate::context::Context;

#[derive(Debug)]
pub struct Occurence {
    line: usize,
    column: usize,
    preview: String,
}

impl Occurence {
    pub fn new(line: usize, column: usize, preview: String) -> Self {
        Occurence {
            line,
            column,
            preview,
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn preview(&self) -> &str {
        &self.preview
    }
}

impl Display for Occurence {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[{}, {}] {}", self.line, self.column, self.preview)
    }
}

pub fn display_results(context: Context, occurences: Vec<Occurence>) {
    println!(
        " - {}: {} times",
        context.file(),
        occurences.len(),
    );
    for occurence in occurences {
        println!("   {}", occurence);
    }
    println!()
}
