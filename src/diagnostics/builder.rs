use super::error::Error;

/// Stores all errors the diagnostics in your program.
#[derive(PartialEq, Debug)]
pub struct DiagnosticBuilder<'err> {
    /// Vector of all the errors in your program.
    pub errors: Vec<Error<'err>>,
}

impl<'err> DiagnosticBuilder<'err> {
    /// Returns a new `DiagnosticBuilder` with an empty vector of errors.
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Adds an error to the error vector.
    pub fn add_err(&mut self, error: Error<'err>) {
        self.errors.push(error);
    }

    /// Emits all the errors by calling `print_all_errs`.
    pub fn emit(&mut self) {
        self.print_all_errs();
    }

    /// Prints all of the errors by running `display`.
    fn print_all_errs(&mut self) {}
}
