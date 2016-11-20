use doc::doc_type::DocType;
use doc::writer::Writer;

const WRITER_TYPE: DocType = DocType::Markdown;

pub struct MarkdownWriter {}

impl Writer for MarkdownWriter {
    fn get_type() -> DocType {
        &WRITER_TYPE
    }
}