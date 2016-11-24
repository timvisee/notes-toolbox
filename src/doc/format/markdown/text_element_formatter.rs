use doc::elements::Elements;
use doc::element_formatter::ElementFormatter;

struct TextElementFormatter {}

impl ElementFormatter for TextElementFormatter {
    fn format(&self, element: &Elements) -> Vec<u8> {
        match element {
            &Elements::Text {
                ref text
            } => {
                text.to_vec()
            }
            _ => {
                panic!("unsupported element type");
            }
        }
    }
}