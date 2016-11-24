use super::doc_type::DocType;
use super::parser::Parser;
use super::formatter::Formatter;
use super::formatter_set::FormatterSet;

/// Document type specification interface.
/// This interface provides the document type, proper parser and formatter to use.
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
        Formatter::new(Self::get_type(), Self::create_formatter_set())
    }

    /// Create a formatter set for this document type.
    fn create_formatter_set() -> FormatterSet;
}