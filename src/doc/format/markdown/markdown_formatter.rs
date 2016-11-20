use doc::doc_type::DocType;
use doc::formatter::Formatter;

const WRITER_TYPE: &'static DocType = &DocType::Markdown;

pub struct MarkdownFormatter {}

impl Formatter for MarkdownFormatter {
    fn get_type() -> &'static DocType {
        WRITER_TYPE
    }
}