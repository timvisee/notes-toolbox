use doc::elements::Elements;
use doc::element_formatter::ElementFormatter;

const MARKDOWN_BOLD_IDENTIFIER: &'static str = "*";

struct BoldTextElementFormatter {}

impl ElementFormatter for BoldTextElementFormatter {
    fn format(element: &Elements) -> Vec<u8> {
        match element {
            &Elements::BoldText {
                ref text
            } => {
                let mut result = MARKDOWN_BOLD_IDENTIFIER.as_bytes().to_vec();
                result.extend(text.iter().cloned());
                result.extend(MARKDOWN_BOLD_IDENTIFIER.as_bytes());
                result
            }
            _ => {
                panic!("unsupported element type");
            }
        }
    }
}