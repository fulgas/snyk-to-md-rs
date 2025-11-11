use crate::markdown::generator::{MarkdownFormatFactory, MarkdownGenerator};
use crate::markdown::MarkdownFormat;

pub(crate) struct SarifMarkdownGeneratorFactory;

impl MarkdownFormatFactory for SarifMarkdownGeneratorFactory {
    fn create_generator(&self, _: MarkdownFormat, _: bool) -> Box<dyn MarkdownGenerator> {
        todo!()
    }
}
