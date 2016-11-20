use doc::element::Element;

const ELEMENT_TYPE_NAME: &'static str = "Text element";

pub struct TextElement {
    text: String
}

impl TextElement {
    /// Constructor.
    pub fn new() -> TextElement {
        TextElement {
            text: "".to_owned()
        }
    }
}

impl Element for TextElement {
    fn get_type_name() -> &'static str {
        return ELEMENT_TYPE_NAME;
    }
}