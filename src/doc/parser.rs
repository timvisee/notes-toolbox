use super::doc_type::DocType;

pub trait Parser {
    /// Get the document type this parser is for
    fn get_type() -> &'static DocType;
}
