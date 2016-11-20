use super::doc_type::DocType;

pub trait DocSpec {
    /// Get the type of the document.
    fn get_name() -> &'static str;

    /// Get the document type
    fn get_type() -> DocType;
}