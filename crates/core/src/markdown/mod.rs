pub(crate) mod generator;

/// Represents the Markdown dialect to use for parsing or rendering.
///
/// This enum defines the supported Markdown syntax standards:
/// - `CommonMark`: The standard CommonMark specification, which is widely adopted as the baseline for Markdown.
/// - `GitHubFlavored`: GitHub's extended Markdown dialect, which includes additional features such as task lists, fenced code blocks, and GitHub-specific syntax.
///
/// The enum is `Debug`, `Clone`, and `Copy`, making it efficient for use in performance-sensitive contexts.
#[derive(Debug, Clone, Copy)]
pub enum MarkdownFormat {
    CommonMark,
    GitHubFlavored,
}
