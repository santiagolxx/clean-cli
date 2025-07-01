use std::fmt;
use std::path::PathBuf;

pub struct ProjectEntry {
    pub path: PathBuf,
    pub project_type: ProjectType,
}

pub enum ProjectType {
    Cargo,
    Node,
}

pub struct Projects {
    pub entries: Vec<ProjectEntry>,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectType::Cargo => write!(f, "Rust (Cargo)"),
            ProjectType::Node => write!(f, "Node.js"),
        }
    }
}
