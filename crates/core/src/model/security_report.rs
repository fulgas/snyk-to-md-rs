
pub(crate) struct SecurityReport {
    pub projects: Vec<SecurityProject>,
}

pub(crate) struct SecurityProject {
    pub name: String,
    pub organization: String,
    pub project_type: ProjectType,
    pub target_file: String,
    pub vulnerabilities: Vec<Vulnerability>,
    pub summary: VulnerabilitySummary,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ProjectType {
    DockerImage,
    Application,
}

impl ProjectType {
    pub fn emoji(&self) -> &'static str {
        match self {
            ProjectType::DockerImage => "🐳",
            ProjectType::Application => "📦",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ProjectType::DockerImage => "Docker Image",
            ProjectType::Application => "Application",
        }
    }
}

pub(crate) struct Vulnerability {
    pub id: String,
    pub title: String,
    pub severity: Severity,
    pub package_name: String,
    pub version: String,
    pub cvss_score: Option<f64>,
    pub is_upgradable: bool,
    pub is_patchable: bool,
    pub cve_ids: Vec<String>,
    pub from_paths: Vec<Vec<String>>, // Multiple dependency paths for duplicates
}

#[derive(Default, Clone)]
pub(crate) struct VulnerabilitySummary {
    pub critical: usize,
    pub high: usize,
    pub medium: usize,
    pub low: usize,
    pub unique_count: usize,
}

#[derive(PartialEq, Ord, PartialOrd, Eq)]
pub(crate) enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

impl Severity {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "critical" => Severity::Critical,
            "high" => Severity::High,
            "medium" => Severity::Medium,
            "low" => Severity::Low,
            _ => Severity::Low,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Critical => "Critical",
            Severity::High => "High",
            Severity::Medium => "Medium",
            Severity::Low => "Low",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            Severity::Critical => "🔴",
            Severity::High => "🟠",
            Severity::Medium => "🟡",
            Severity::Low => "🟢",
        }
    }
}
