use crate::parser::{Parser, ParserType, ParserTypeFactory};

pub(crate) struct SarifParserFactory;

impl ParserTypeFactory for SarifParserFactory {
    fn create_parser(&self, _: ParserType) -> Box<dyn Parser> {
        todo!()
    }
}
