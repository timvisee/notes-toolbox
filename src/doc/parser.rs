use super::doc_type::DocType;

pub struct Parser {
    doc_type: &'static DocType
}

impl Parser {
    /// Get the document type this parser is for
    fn get_type(&self) -> &'static DocType {
        self.doc_type
    }
}
