use super::doc_type::DocType;

pub struct Formatter {
    doc_type: &'static DocType
}

impl Formatter {
    /// Get the document type this formatter is for
    fn get_type(&self) -> &'static DocType {
        self.doc_type
    }
}
