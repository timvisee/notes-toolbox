use super::doc_type::DocType;

pub struct Formatter {
    doc_type: DocType
}

impl Formatter {
    /// Constructor.
    ///
    /// The document type for this formatter should be passed to the `doc_type` parameter.
    pub fn new(doc_type: DocType) -> Formatter {
        Formatter {
            doc_type: doc_type
        }
    }

    /// Get the document type this formatter is for
    fn get_type(&self) -> &DocType {
        &self.doc_type
    }
}
