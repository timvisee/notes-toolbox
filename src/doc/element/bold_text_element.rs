use doc::base_element::BaseElement;

const ELEMENT_TYPE_NAME: &'static str = "Bold text element";

pub struct BoldTextElement {
    text: String
}

impl BoldTextElement {
    /// Constructor.
    pub fn new() -> BoldTextElement {
        BoldTextElement {
            text: "".to_owned()
        }
    }
}

impl BaseElement for BoldTextElement {
    fn get_type_name() -> &'static str {
        return ELEMENT_TYPE_NAME;
    }
}