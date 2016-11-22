use super::doc_type::DocType;
use super::parser::Parser;
use super::formatter::Formatter;

pub trait DocSpec {
    /// Get the type of the document.
    fn get_name() -> &'static str;

    /// Get the document type.
    fn get_type() -> DocType;

    /// Create a parser for this document specification.
    ///
    /// Returns the configured parser.
    fn create_parser() -> Parser {
        Parser::new(Self::get_type())
    }

    /// Create a formatter for this document specification.
    ///
    /// Returns the configured formatter.
    fn create_formatter() -> Formatter {
        Formatter::new(Self::get_type())
    }
}