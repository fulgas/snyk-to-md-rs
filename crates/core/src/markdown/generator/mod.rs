mod json;
mod sarif;

use crate::markdown::generator::json::JsonMarkdownGeneratorFactory;
use crate::markdown::generator::sarif::SarifMarkdownGeneratorFactory;
use crate::markdown::MarkdownFormat;
use crate::parser::{ParsedReport, ParserFormat};

#[derive(Debug, thiserror::Error)]
pub enum GeneratorError {
    #[error("Failed to generate markdown report from template")]
    AskamaError(#[from] askama::Error),
}

pub(crate) trait MarkdownGenerator {
    fn generate_markdown_template(&self, report: &ParsedReport) -> Result<String, GeneratorError>;
}

pub(crate) struct MarkdownParserFormatFactory;

impl MarkdownParserFormatFactory {
    pub(crate) fn create_generator_format(
        parser_format: &ParserFormat,
    ) -> Box<dyn MarkdownFormatFactory> {
        match parser_format {
            ParserFormat::Json => Box::new(JsonMarkdownGeneratorFactory),
            ParserFormat::Sarif => Box::new(SarifMarkdownGeneratorFactory),
        }
    }
}

pub(crate) trait MarkdownFormatFactory {
    fn create_generator(
        &self,
        markdown_format: MarkdownFormat,
        with_emoji: bool,
    ) -> Box<dyn MarkdownGenerator>;
}
