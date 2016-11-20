use doc::doc_spec::DocSpec;

const TYPE_NAME: &'static str = "Markdown";

struct MarkdownSpec {}

impl DocSpec for MarkdownSpec {
    fn get_name() -> &'static str {
        return TYPE_NAME;
    }
}