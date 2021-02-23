use super::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

impl Display for Error<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut diagnostic = String::new();

        // The top part of the error.
        // E.g. (with code)
        // ```
        // error[101]: Mismatched types
        // ```
        // E.g. (without code)
        // ```
        // error: Mismatched types
        // ```
        if let Some(code) = self.code {
            diagnostic.push_str(&format!("error[{}]: {}\n", code, self.message));
        } else {
            diagnostic.push_str(&format!("error: {}\n", self.message));
        }

        // Adds file and line information to the diagnostic
        if let (Some(file), Some(line)) = (self.file, self.line) {
            diagnostic.push_str(&format!("cause: {}:{}\n", file, line));
        }

        // The helpful notes of the given error.
        // E.g.
        // ```
        // note: `Foo` does not implement `Clone`
        // ```
        if let Some(note_msgs) = self.note_msgs.clone() {
            for note_msg in note_msgs {
                diagnostic.push_str(&format!("note: {}\n", note_msg));
            }
        }

        // The help messages of the given error.
        // E.g.
        // ```
        // help: implement `Clone` for `Foo`. add `#[derive(Clone)]` above the definition of `Foo`
        // ```
        if let Some(help_msgs) = self.help_msgs.clone() {
            for help_msg in help_msgs {
                diagnostic.push_str(&format!("help: {}\n", help_msg));
            }
        }

        // Write the whole diagnostic to the formatter
        write!(f, "{}", diagnostic)?;
        Ok(())
    }
}
