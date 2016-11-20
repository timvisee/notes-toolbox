use super::doc_type::DocType;

pub trait Formatter {
    /// Get the document type this formatter is for
    fn get_type() -> &'static DocType;
}
