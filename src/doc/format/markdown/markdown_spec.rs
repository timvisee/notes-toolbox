use doc::doc_type::DocType;
use doc::doc_spec::DocSpec;
use doc::formatter_set::FormatterSet;

const TYPE_NAME: &'static str = "Markdown";

struct MarkdownSpec {}

impl DocSpec for MarkdownSpec {
    fn get_name() -> &'static str {
        return TYPE_NAME;
    }

    fn get_type() -> DocType {
        DocType::Markdown
    }

    fn create_formatter_set() -> FormatterSet {
        // TODO: Properly initialize the formatter set, make sure that we're properly adding all element formatters
        FormatterSet::new()
    }
}