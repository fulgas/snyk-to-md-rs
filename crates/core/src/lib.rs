use crate::markdown::generator::{GeneratorError, MarkdownParserFormatFactory};
use crate::markdown::MarkdownFormat;
use crate::parser::{ParserError, ParserFormatFactory};
use crate::parser::{ParserFormat, ParserType};

pub mod markdown;
pub mod parser;

#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    #[error("JSON content was not provided")]
    MissingContent,
    #[error("Parser type was not specified")]
    MissingParserType,
    #[error("Parser format was not specified")]
    MissingParserFormat,
    #[error("Markdown format was not specified")]
    MissingMarkdownFormat,
    #[error(transparent)]
    ParseError(#[from] ParserError),
    #[error(transparent)]
    GeneratedReportError(#[from] GeneratorError),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseError(#[from] ParserError),
    #[error(transparent)]
    GeneratedReportError(#[from] GeneratorError),
}

pub struct ReportProcessor {
    parser_type: ParserType,
    parser_format: ParserFormat,
    content: String,
    markdown_format: MarkdownFormat,
    with_emoji: bool,
}

impl ReportProcessor {
    pub fn generate(self) -> anyhow::Result<String, Error> {
        let parser_report = ParserFormatFactory::create_parser_format(&self.parser_format)
            .create_parser(self.parser_type)
            .parse(self.content.as_str())?;

        let template = MarkdownParserFormatFactory::create_generator_format(&self.parser_format)
            .create_generator(self.markdown_format, self.with_emoji)
            .generate_markdown_template(&parser_report)?;

        Ok(template)
    }
}

#[derive(Default)]
pub struct ReportProcessorBuilder {
    parser_type: Option<ParserType>,
    parser_format: Option<ParserFormat>,
    content: Option<String>,
    markdown_format: Option<MarkdownFormat>,
    with_emoji: bool,
}

impl ReportProcessorBuilder {
    pub fn new() -> Self {
        Self {
            parser_type: None,
            parser_format: None,
            content: None,
            markdown_format: None,
            with_emoji: false,
        }
    }

    pub fn parser_type(mut self, parser_type: ParserType) -> Self {
        self.parser_type = Some(parser_type);
        self
    }

    pub fn parser_format(mut self, parser_format: ParserFormat) -> Self {
        self.parser_format = Some(parser_format);
        self
    }

    pub fn content(mut self, content: &str) -> Self {
        self.content = Some(content.to_string());
        self
    }

    pub fn markdown_format(mut self, markdown_format: MarkdownFormat) -> Self {
        self.markdown_format = Some(markdown_format);
        self
    }

    pub fn with_emoji(mut self, with_emoji: bool) -> Self {
        self.with_emoji = with_emoji;
        self
    }

    pub fn build(self) -> Result<ReportProcessor, BuilderError> {
        Ok(ReportProcessor {
            parser_type: self.parser_type.ok_or(BuilderError::MissingParserType)?,
            parser_format: self
                .parser_format
                .ok_or(BuilderError::MissingParserFormat)?,
            content: self.content.ok_or(BuilderError::MissingContent)?,
            markdown_format: self
                .markdown_format
                .ok_or(BuilderError::MissingMarkdownFormat)?,
            with_emoji: self.with_emoji,
        })
    }
}
