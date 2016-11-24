use std::collections::HashMap;
use super::elements::Elements;
use super::element_formatter::ElementFormatter;

/// Formatter set.
///
/// This set contains a list of element formatters, usually for a specific document type.
/// The set provides a _toolbox_ of formatters the actual formatter can use to format the document.
pub struct FormatterSet {
    /// Map containing all formatters, grouped by an element type.
    formatters: HashMap<Elements, Box<ElementFormatter>>
}

impl FormatterSet {
    /// Create an empty formatter set.
    pub fn new() -> FormatterSet {
        FormatterSet {
            formatters: HashMap::new()
        }
    }
}