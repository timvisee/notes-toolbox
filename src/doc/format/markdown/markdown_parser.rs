use doc::doc_type::DocType;
use doc::parser::Parser;

const PARSER_TYPE: &'static DocType = &DocType::Markdown;

pub struct MarkdownParser {}

impl Parser for MarkdownParser {
    fn get_type() -> &'static DocType {
        PARSER_TYPE
    }
}
