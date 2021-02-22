#[cfg(test)]
mod test_error {
    use crate::diagnostics::error::Error;

    #[test]
    fn test_span_label() {
        let mut error = Error::new(false);
        error.span_label(None, "an error");
        assert_eq!(error.message, "an error");
    }

    #[test]
    fn test_code() {
        let mut error = Error::new(false);
        error.span_label(Some(101), "this is a test");
        assert_eq!(error.code, Some(101));
        assert_eq!(error.message, "this is a test");
    }

    #[test]
    fn test_file_and_line() {
        let mut error = Error::new(false);
        error.span_label(None, "");
        error.add_file_and_line(file!(), line!() as usize);
        assert_eq!(error.file, Some("src/tests/error.rs"));
        assert_eq!(error.line, Some(24));
    }

    #[test]
    fn test_notes() {
        let mut error = Error::new(false);
        error.span_label(None, "test notes");
        error.note("foo");
        error.note("bar");
        assert_eq!(error.message, "test notes");
        assert_eq!(error.note_msgs, Some(vec!["foo", "bar"]));
    }

    #[test]
    fn test_help() {
        let mut error = Error::new(false);
        error.span_label(None, "test help");
        error.help("foo");
        error.help("bar");
        assert_eq!(error.message, "test help");
        assert_eq!(error.help_msgs, Some(vec!["foo", "bar"]));
    }

    #[test]
    fn complete_error() {
        let mut error = Error::new(false);
        error.span_label(Some(101), "expected semi colon");
        error.help("add a semi colon: `;`");
        error.note("this programming language uses semi colons to end expressions");
        error.add_file_and_line(file!(), line!() as usize);

        assert_eq!(error.message, "expected semi colon");
        assert_eq!(error.code, Some(101));
        assert_eq!(error.help_msgs, Some(vec!["add a semi colon: `;`"]));
        assert_eq!(
            error.note_msgs,
            Some(vec![
                "this programming language uses semi colons to end expressions"
            ])
        );
        assert_eq!(error.file, Some("src/tests/error.rs"));
        assert_eq!(error.line, Some(55))
    }
}
