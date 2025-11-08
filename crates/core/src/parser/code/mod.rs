use crate::model::security_report::SecurityReport;
use crate::parser::{Parser, ParserError};
use anyhow::Result;

pub(crate) struct SnykCodeParser;

impl Parser for SnykCodeParser {
    fn parse<'a>(&self, _: &str) -> Result<SecurityReport, ParserError> {
        Ok(SecurityReport { projects: vec![] })
    }
}
