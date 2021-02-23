use why::diagnostics::{builder::DiagnosticBuilder, error::Error};

fn main() {
    let mut builder = DiagnosticBuilder::new();
    let mut error = Error::new(false);
    error.span_label(Some(101), "fail");
    error.note("it failed");
    error.add_file_and_line(file!(), line!() as usize);
    builder.add_err(error);
    builder.emit();
}
