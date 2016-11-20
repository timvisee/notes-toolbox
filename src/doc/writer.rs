use super::doc_type::DocType;

pub trait Writer {
    /// Get the document type this writer is for
    fn get_type() -> DocType;
}
