use crate::error::error::{HulkError, HulkErrorTrait};

pub struct ErrorHandler {
    program_text: String,
    line_breaks: Vec<usize>,
    pub errors: Vec<HulkError>,
}

impl ErrorHandler {
    pub fn new(program_text: &str) -> Self {
        Self {
            program_text: program_text.to_string(),
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

    pub fn get_error_messages(&mut self) -> Vec<String> {
        self.errors
            .sort_by(|a, b| a.get_position().cmp(&b.get_position()));

        self.errors
            .iter()
            .map(|error| {
                let line_number = self.get_line_number(error.get_position());
                let line_start = self.line_breaks[line_number];
                let line_end = if line_number + 1 < self.line_breaks.len() {
                    self.line_breaks[line_number + 1]
                } else {
                    self.program_text.len()
                };
                let line_text = &self.program_text[line_start..line_end].trim_end_matches('\n');
                let col = error.get_position() - line_start;
                let pointer_line = format!("{}{}", " ".repeat(col), "^",);

                format!(
                    "{}\n --> line {}:{}\n  |\n{:3} | {}\n  |   {}\n",
                    error.to_string(),
                    line_number + 1,
                    col + 1,
                    line_number + 1,
                    line_text,
                    pointer_line
                )
            })
            .collect()
    }

    fn get_line_number(&self, position: usize) -> usize {
        match self.line_breaks.binary_search_by(|&line| {
            if line > position {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        }) {
            Ok(idx) | Err(idx) => idx,
        }
    }
}
