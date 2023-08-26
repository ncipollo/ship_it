pub mod operation;

use std::path::Path;
use git2::{Repository};
use crate::config::Config;
use crate::error::SheepError;
use crate::project::operation::Operation;
use crate::repo;
use crate::repo::clone::GitCloner;
use crate::repo::open::{GitOpener};
use crate::repo::path;

struct Project {
    config: Config,
    repo: Repository,
}

impl Project {
    pub fn new_local_project(path: &str) -> Result<Project, SheepError> {
        let repo = GitOpener::new().open(path)?;
        let config = Config {};
        let project = Project {
            config,
            repo,
        };
        Ok(project)
    }

    pub fn new_remote_project(url: &str, directory: &str) -> Result<Project, SheepError> {
        let repo_path = path::repo_path(url, directory)?;
        let repo = GitCloner::new().clone(url, repo_path)?;
        let config = Config {};
        let project = Project {
            config,
            repo,
        };
        Ok(project)
    }

    pub fn new_dry_run_project(path: &str) -> Result<Project, SheepError> {
        todo!("implement dry run project constructor")
    }

    pub fn update(&self, operation: Operation) {
        // First get the version update information based upon operations type

        // Next create project strings object based upon configuration & version update

        // Create branch if enabled in configuration

        // Create commit if enabled in configuration

        // Create tag if enabled in configuration

        // Push if enabled in configuration

        // Process subprojects if there are any
    }
}