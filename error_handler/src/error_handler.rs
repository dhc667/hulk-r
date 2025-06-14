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

    pub fn add_error(&mut self, error: HulkError) {
        self.errors.push(error);
    }

    pub fn extend_errors(&mut self, errors: Vec<HulkError>) {
        self.errors.extend(errors);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn get_error_messages(&self) -> Vec<String> {
        self.errors.iter().map(|error| error.to_string()).collect()
    }
}
