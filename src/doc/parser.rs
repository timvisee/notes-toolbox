use super::doc_type::DocType;

/// Document parser.
/// This parser parses a raw document to the document elements supported by this application.
///
/// The proper document type and parser should be set to ensure a given document is parsed from the correct format.
pub struct Parser {
    doc_type: DocType
}

impl Parser {
    /// Constructor.
    ///
    /// The document type for this parser should be passed to the `doc_type` parameter.
    pub fn new(doc_type: DocType) -> Parser {
        Parser {
            doc_type: doc_type
        }
    }

    /// Get the document type this parser is for
    fn get_type(&self) -> &DocType {
        &self.doc_type
    }
}
