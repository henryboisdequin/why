use std::backtrace::Backtrace;

/// Error type which holds all of the errors information.
#[derive(PartialEq, Debug)]
pub struct Error<'err> {
    /// The main message/cause of the error.
    pub message: &'err str,
    /// Any help messages which could point the user to the right direction.
    pub help_msgs: Option<Vec<&'err str>>,
    /// Any notes which could give the user additional information.
    pub note_msgs: Option<Vec<&'err str>>,
    /// The error code of the current error.
    pub code: Option<usize>,
    /// The file in which the error occurred.
    pub file: Option<&'err str>,
    /// The line in which the error occurred.
    pub line: Option<usize>,
    /// Whether to display a stack backtrace or not.
    pub debug: bool,
}

impl<'err> Error<'err> {
    /// Returns a new default `Error`.
    pub fn new(debug: bool) -> Self {
        Self {
            message: "",
            help_msgs: None,
            note_msgs: None,
            code: None,
            file: None,
            line: None,
            debug,
        }
    }

    /// Adds a main label to the error.
    pub fn span_label(&mut self, code: Option<usize>, message: &'err str) {
        if let Some(err_code) = code {
            self.code = Some(err_code);
        }

        self.message = message;
    }

    /// Adds a note to the error.
    pub fn note(&mut self, note: &'err str) {
        if let Some(notes_msgs) = self.note_msgs.as_mut() {
            notes_msgs.push(note);
            self.note_msgs = Some(notes_msgs.to_vec());
        } else {
            self.note_msgs = Some(vec![note]);
        }
    }

    /// Adds a help message to the error.
    pub fn help(&mut self, help_msg: &'err str) {
        if let Some(help_msgs) = self.help_msgs.as_mut() {
            help_msgs.push(help_msg);
            self.help_msgs = Some(help_msgs.to_vec());
        } else {
            self.help_msgs = Some(vec![help_msg]);
        }
    }

    /// Adds the file and line in which this error occurred.
    pub fn add_file_and_line(&mut self, file: &'err str, line: usize) {
        self.file = Some(file);
        self.line = Some(line);
    }

    /// Shows the backtrace of the error.
    pub fn show_backtrace(self) -> Option<Backtrace> {
        if self.debug {
            Some(Backtrace::capture())
        } else {
            None
        }
    }
}
