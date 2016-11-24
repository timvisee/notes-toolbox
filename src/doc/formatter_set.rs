use std::collections::HashMap;
use super::elements::Elements;
use super::element_formatter::ElementFormatter;

pub struct FormatterSet {
    formatters: HashMap<Elements, Box<ElementFormatter>>
}

impl FormatterSet {
    /// Constructor.
    pub fn new() -> FormatterSet {
        FormatterSet {
            formatters: HashMap::new()
        }
    }
}