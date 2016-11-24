use doc::elements::Elements;
use doc::element_formatter::ElementFormatter;

const MARKDOWN_HEADER_IDENTIFIER: &'static str = "#";
const MARKDOWN_NEWLINE: &'static str = "\n";

struct HeaderElementFormatter {}

impl ElementFormatter for HeaderElementFormatter {
    fn format(&self, element: &Elements) -> Vec<u8> {
        match element {
            &Elements::Header {
                ref level,
                ref text
            } => {
                // Create the result vector
                let mut result = Vec::new();

                // Push the header identifiers
                for i in 0..level {
                    result.push(MARKDOWN_BOLD_IDENTIFIER.as_bytes());
                }

                // Push the separating space
                result.push(" ".as_bytes());

                // Push the header text
                result.extend(text.iter().cloned());

                // Push a new line, and return
                result.extend(MARKDOWN_NEWLINE.as_bytes());
                result
            }
            _ => {
                panic!("unsupported element type");
            }
        }
    }
}