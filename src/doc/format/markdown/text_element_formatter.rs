use doc::elements::Elements;
use doc::element_formatter::ElementFormatter;

// Text element formatter in Markdown
pub struct TextElementFormatter {}

impl TextElementFormatter {
    /// Constructor.
    fn new() -> TextElementFormatter {
        TextElementFormatter {}
    }
}

impl ElementFormatter for TextElementFormatter {
    fn format(&self, element: &Elements) -> Vec<u8> {
        match element {
            // Format the text element
            &Elements::Text {
                ref text
            } => {
                text.to_vec()
            }

            // An unsupported element type
            _ => {
                panic!("unsupported element type");
            }
        }
    }
}