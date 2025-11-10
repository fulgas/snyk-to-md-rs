use crate::model::security_report::SecurityReport;
use crate::parser::code::SnykCodeParser;
use crate::parser::container::container_parser::SnykContainerParser;
use anyhow::Result;

mod code;
mod container;

#[derive(Clone, Debug)]
pub enum ParserType {
    Container,
    Code,
}

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Failed to parse Snyk report JSON")]
    JsonError(#[from] serde_json::Error),
}

pub(crate) trait Parser {
    fn parse(&self, content: &str) -> Result<SecurityReport, ParserError>;
}

pub(crate) struct ParserFactory;

impl ParserFactory {
    pub fn create_parser(report_type: ParserType) -> Box<dyn Parser> {
        match report_type {
            ParserType::Container => Box::new(SnykContainerParser),
            ParserType::Code => Box::new(SnykCodeParser),
        }
    }
}
