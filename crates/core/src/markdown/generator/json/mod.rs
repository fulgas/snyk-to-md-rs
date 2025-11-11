use crate::markdown::generator::json::common_mark::CommonMarkGenerator;
use crate::markdown::generator::json::gfm::GitFlavouredMarkdownGenerator;
use crate::markdown::generator::{MarkdownFormatFactory, MarkdownGenerator};
use crate::markdown::MarkdownFormat;

mod common_mark;
mod gfm;
pub(crate) struct JsonMarkdownGeneratorFactory;

impl MarkdownFormatFactory for JsonMarkdownGeneratorFactory {
    fn create_generator(
        &self,
        markdown_format: MarkdownFormat,
        with_emoji: bool,
    ) -> Box<dyn MarkdownGenerator> {
        match markdown_format {
            MarkdownFormat::CommonMark => Box::new(CommonMarkGenerator::new(with_emoji)),
            MarkdownFormat::GitHubFlavored => Box::new(GitFlavouredMarkdownGenerator),
        }
    }
}
