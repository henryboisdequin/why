use super::error::Error;

/// Stores all errors the diagnostics in your program.
#[derive(PartialEq, Debug)]
pub struct DiagnosticBuilder<'err> {
    /// Vector of all the errors in your program.
    pub errors: Vec<Error<'err>>,
    /// How many errors have occurred.
    pub error_count: usize,
}

impl<'err> DiagnosticBuilder<'err> {
    /// Returns a new `DiagnosticBuilder` with an empty vector of errors.
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            error_count: 0,
        }
    }

    /// Adds an error to the error vector.
    pub fn add_err(&mut self, error: Error<'err>) {
        self.errors.push(error);
        self.error_count += 1;
    }

    /// Emits all the errors by calling `print_all_errs`.
    pub fn emit(self) {
        self.print_all_errs();
    }

    /// Prints all of the errors by running `display`.
    fn print_all_errs(self) {
        for error in &self.errors {
            println!("{}", error);
            if self.error_count == 1 {
                println!("aborting due to the previous error")
            } else {
                println!("aborting due to {} previous errors", self.error_count);
            }
        }
    }
}
