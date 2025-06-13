use crate::error::error::HulkError;

pub struct ErrorHandler {
    pub line_breaks: Vec<usize>,
    pub errors: Vec<HulkError>,
}

impl ErrorHandler {
    pub fn new(program_text: &str) -> Self {
        Self {
            line_breaks: Self::get_line_breaks(program_text),
            errors: Vec::new(),
        }
    }

    fn get_line_breaks(program_text: &str) -> Vec<usize> {
        let mut line_breaks = vec![0];
        for (i, c) in program_text.char_indices() {
            if c == '\n' {
                line_breaks.push(i);
            }
        }
        line_breaks
    }
}
