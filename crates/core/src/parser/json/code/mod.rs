use crate::parser::{ParsedReport, Parser, ParserError};
use anyhow::Result;

pub(crate) struct SnykCodeParser;

impl Parser for SnykCodeParser {
    fn parse(&self, _: &str) -> Result<ParsedReport, ParserError> {
        Ok(ParsedReport::Code(String::new()))
    }
}
