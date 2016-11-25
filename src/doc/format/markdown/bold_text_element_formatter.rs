use doc::elements::Elements;
use doc::element_formatter::ElementFormatter;

/// Markdown bold text character identifier
const MARKDOWN_BOLD_IDENTIFIER: &'static str = "*";

/// Formatter for bold text in Markdown
pub struct BoldTextElementFormatter {}

impl BoldTextElementFormatter {
    /// Constructor.
    fn new() -> BoldTextElementFormatter {
        BoldTextElementFormatter {}
    }
}

impl ElementFormatter for BoldTextElementFormatter {
    fn format(&self, element: &Elements) -> Vec<u8> {
        match element {
            // Parse the bold text element
            &Elements::BoldText {
                ref text
            } => {
                let mut result = MARKDOWN_BOLD_IDENTIFIER.as_bytes().to_vec();
                result.extend(text.iter().cloned());
                result.extend(MARKDOWN_BOLD_IDENTIFIER.as_bytes());
                result
            }

            // An unsupported element type is given
            _ => {
                panic!("unsupported element type");
            }
        }
    }
}
