use doc::doc_type::DocType;
use doc::writer::Writer;

struct MarkdownWriter {}

impl Writer for MarkdownWriter {
    fn get_type() -> DocType {
        DocType::Markdown
    }
}