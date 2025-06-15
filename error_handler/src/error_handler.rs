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
        if !program_text.ends_with('\n') {
            line_breaks.push(program_text.len());
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
            .map(|error| self.format_message(error))
            .collect()
    }

    pub fn get_raw_errors(&mut self) -> Vec<String> {
        self.errors
            .sort_by(|a, b| a.get_position().cmp(&b.get_position()));

        self.errors.iter().map(|error| error.to_string()).collect()
    }

    fn get_line_number(&self, position: usize) -> usize {
        self.line_breaks
            .partition_point(|&line_start| line_start < position)
            - 1
    }

    fn format_message(&self, error: &HulkError) -> String {
        let pos = error.get_position();
        let line_number = self.get_line_number(pos);
        let line_start = self.line_breaks[line_number];
        let line_end = self.line_breaks[line_number + 1];

        let line_text = &self.program_text[line_start..line_end]
            .trim_end_matches('\n')
            .trim_start_matches("\n");
        let col = pos - line_start;
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
    }
}
