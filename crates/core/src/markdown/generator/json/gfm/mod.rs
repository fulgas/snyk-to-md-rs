use crate::markdown::generator::{GeneratorError, MarkdownGenerator};
use crate::parser::ParsedReport;

pub(crate) struct GitFlavouredMarkdownGenerator;

impl MarkdownGenerator for GitFlavouredMarkdownGenerator {
    fn generate_markdown_template(&self, _: &ParsedReport) -> Result<String, GeneratorError> {
        todo!()
    }
}
