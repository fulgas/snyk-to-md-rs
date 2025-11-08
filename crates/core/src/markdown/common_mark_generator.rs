use crate::markdown::{GeneratorError, MarkdownGenerator};
use crate::model::security_report::SecurityReport;
use askama::Template;

#[derive(Template)]
#[template(path = "report.md")]
struct ReportTemplate {
    projects: Vec<ReportProject>,
    timestamp: String,
}

struct ReportProject {
    name: String,
    organization: String,
    project_type_name: String,
    project_type_emoji: String,
    target_file: String,
    summary: ReportSummary,
    vulnerabilities: Vec<ReportVulnerability>,
}

#[derive(Clone)]
struct ReportSummary {
    critical: usize,
    high: usize,
    medium: usize,
    low: usize,
    unique_count: usize,
}

struct ReportVulnerability {
    id: String,
    title: String,
    severity: String,
    severity_emoji: String,
    package_name: String,
    version: String,
    cvss_score: Option<f64>,
    is_upgradable: bool,
    is_patchable: bool,
    cve_ids: Vec<String>,
    from_paths: Vec<String>,
}

pub(crate) struct CommonMarkGenerator;

impl MarkdownGenerator for CommonMarkGenerator {
    fn generate_markdown_report(&self, report: &SecurityReport) -> Result<String, GeneratorError> {
        let timestamp = chrono::Utc::now()
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string();

        let projects = report
            .projects
            .iter()
            .map(|project| ReportProject {
                name: project.name.clone(),
                organization: project.organization.clone(),
                project_type_name: project.project_type.as_str().to_string(),
                project_type_emoji: project.project_type.emoji().to_string(),
                target_file: project.target_file.clone(),
                summary: ReportSummary {
                    critical: project.summary.critical,
                    high: project.summary.high,
                    medium: project.summary.medium,
                    low: project.summary.low,
                    unique_count: project.summary.unique_count,
                },
                vulnerabilities: map_vulnerabilities(&project.vulnerabilities),
            })
            .collect();

        let template_ctx = ReportTemplate {
            projects,
            timestamp,
        };

        template_ctx.render().map_err(GeneratorError::AskamaError)
    }
}

fn map_vulnerabilities(
    vulns: &[crate::model::security_report::Vulnerability],
) -> Vec<ReportVulnerability> {
    vulns
        .iter()
        .map(|v| {
            let from_paths: Vec<String> =
                v.from_paths.iter().map(|path| path.join(" → ")).collect();

            ReportVulnerability {
                id: v.id.clone(),
                title: v.title.clone(),
                severity: v.severity.as_str().to_string(),
                severity_emoji: v.severity.emoji().to_string(),
                package_name: v.package_name.clone(),
                version: v.version.clone(),
                cvss_score: v.cvss_score.clone(),
                is_upgradable: v.is_upgradable,
                is_patchable: v.is_patchable,
                cve_ids: v.cve_ids.clone(),
                from_paths,
            }
        })
        .collect()
}
