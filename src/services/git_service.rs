//! Git service for repository operations.
//! Used by the runner to manage git operations on task branches.

use std::path::Path;
use std::process::Command;

use crate::error::{AppError, AppResult};

pub struct GitService;

impl GitService {
    /// Clone a repository to the specified path
    pub fn clone_repo(github_url: &str, local_path: &str) -> AppResult<()> {
        let output = Command::new("git")
            .args(["clone", github_url, local_path])
            .output()
            .map_err(|e| AppError::Internal(format!("Failed to execute git clone: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::Internal(format!("Git clone failed: {}", stderr)));
        }

        Ok(())
    }

    /// Create a new branch
    pub fn create_branch(repo_path: &str, branch_name: &str) -> AppResult<()> {
        let output = Command::new("git")
            .current_dir(repo_path)
            .args(["checkout", "-b", branch_name])
            .output()
            .map_err(|e| AppError::Internal(format!("Failed to execute git checkout: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::Internal(format!(
                "Git checkout failed: {}",
                stderr
            )));
        }

        Ok(())
    }

    /// Switch to an existing branch
    pub fn checkout_branch(repo_path: &str, branch_name: &str) -> AppResult<()> {
        let output = Command::new("git")
            .current_dir(repo_path)
            .args(["checkout", branch_name])
            .output()
            .map_err(|e| AppError::Internal(format!("Failed to execute git checkout: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::Internal(format!(
                "Git checkout failed: {}",
                stderr
            )));
        }

        Ok(())
    }

    /// Apply a patch to the repository
    pub fn apply_patch(repo_path: &str, patch_content: &str) -> AppResult<()> {
        use std::io::Write;
        use std::process::Stdio;

        let mut child = Command::new("git")
            .current_dir(repo_path)
            .args(["apply", "-"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| AppError::Internal(format!("Failed to execute git apply: {}", e)))?;

        if let Some(stdin) = child.stdin.as_mut() {
            stdin
                .write_all(patch_content.as_bytes())
                .map_err(|e| AppError::Internal(format!("Failed to write patch: {}", e)))?;
        }

        let output = child
            .wait_with_output()
            .map_err(|e| AppError::Internal(format!("Failed to wait for git apply: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::Internal(format!("Git apply failed: {}", stderr)));
        }

        Ok(())
    }

    /// Get the current branch name
    pub fn current_branch(repo_path: &str) -> AppResult<String> {
        let output = Command::new("git")
            .current_dir(repo_path)
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .map_err(|e| AppError::Internal(format!("Failed to execute git: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::Internal(format!("Git command failed: {}", stderr)));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    /// Check if a path is a valid git repository
    pub fn is_git_repo(path: &str) -> bool {
        Path::new(path).join(".git").exists()
    }

    /// Pull latest changes
    pub fn pull(repo_path: &str) -> AppResult<()> {
        let output = Command::new("git")
            .current_dir(repo_path)
            .args(["pull"])
            .output()
            .map_err(|e| AppError::Internal(format!("Failed to execute git pull: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::Internal(format!("Git pull failed: {}", stderr)));
        }

        Ok(())
    }
}
