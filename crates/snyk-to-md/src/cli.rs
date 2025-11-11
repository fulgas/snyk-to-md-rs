use clap::{Parser, Subcommand, ValueEnum};
use snyk_to_md_core::markdown::MarkdownFormat;
use snyk_to_md_core::parser::{ParserFormat, ParserType};
use std::path::PathBuf;

#[derive(Debug, Clone, Subcommand, ValueEnum)]
pub(crate) enum CliParserType {
    Container,
    Code,
}

impl From<CliParserType> for ParserType {
    fn from(cli_type: CliParserType) -> Self {
        match cli_type {
            CliParserType::Container => ParserType::Container,
            CliParserType::Code => ParserType::Code,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum CliInputFormat {
    #[value(name = "json")]
    Json,
    #[value(name = "sarif")]
    Sarif,
}

impl From<CliInputFormat> for ParserFormat {
    fn from(cli_type: CliInputFormat) -> Self {
        match cli_type {
            CliInputFormat::Json => ParserFormat::Json,
            CliInputFormat::Sarif => ParserFormat::Sarif,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum CliOutputFormat {
    #[value(name = "github-flavored")]
    GitHubFlavored,
    #[value(name = "common-mark")]
    CommonMark,
}

impl From<CliOutputFormat> for MarkdownFormat {
    fn from(cli_format: CliOutputFormat) -> Self {
        match cli_format {
            CliOutputFormat::GitHubFlavored => MarkdownFormat::GitHubFlavored,
            CliOutputFormat::CommonMark => MarkdownFormat::CommonMark,
        }
    }
}

#[derive(Parser)]
#[command(name = "snyk-to-md")]
#[command(about, version)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: CliParserType,

    #[arg(short = 't', long, value_parser, default_value = "json")]
    pub(crate) input_format: CliInputFormat,

    #[arg(short = 'f', long, value_parser, default_value = "CommonMark")]
    pub(crate) output_format: CliOutputFormat,

    #[arg(short = 'i', long)]
    pub(crate) input: PathBuf,

    #[arg(short = 'o', long)]
    pub(crate) output: Option<PathBuf>,

    #[arg(short = 'e', long, default_value = "false")]
    pub(crate) with_emoji: bool,
}
