use super::doc_type::DocType;

pub struct Formatter {
    doc_type: &'static DocType
}

impl Formatter {
    /// Constructor.
    ///
    /// The document type for this formatter should be passed to the `doc_type` parameter.
    fn new(doc_type: &'static DocType) -> Formatter {
        Formatter {
            doc_type: doc_type
        }
    }

    /// Get the document type this formatter is for
    fn get_type(&self) -> &'static DocType {
        self.doc_type
    }
}
