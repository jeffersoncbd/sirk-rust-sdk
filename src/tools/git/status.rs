#[path = "status/visible_file.rs"]
mod visible_file;

use self::visible_file::visible_file;
use super::Git;
use crate::Error;
use crate::tools::project::project;
use std::collections::BTreeSet;

impl Git {
    pub fn status(&self) -> Result<Vec<String>, Error> {
        let project = project(&self.directory)?;
        let pathspec = if project.prefix.is_empty() {
            "."
        } else {
            project.prefix.trim_end_matches('/')
        };
        let output = std::process::Command::new("git")
            .args([
                "status",
                "--porcelain=v1",
                "-z",
                "--untracked-files=all",
                "--",
            ])
            .arg(pathspec)
            .current_dir(&project.root)
            .output()
            .map_err(|error| Error::Tool(format!("could not run `git status`: {error}")))?;
        if !output.status.success() {
            return Err(Error::Tool(format!(
                "`git status` exited with {}: {}",
                output.status,
                String::from_utf8_lossy(&output.stderr).trim()
            )));
        }
        let ignored = std::process::Command::new("git")
            .args([
                "ls-files",
                "--cached",
                "--others",
                "--ignored",
                "--exclude-per-directory=.treeignore",
                "-z",
                "--",
            ])
            .arg(pathspec)
            .current_dir(&project.root)
            .output()
            .map_err(|error| Error::Tool(format!("could not evaluate `.treeignore`: {error}")))?;
        if !ignored.status.success() {
            return Err(Error::Tool(format!(
                "could not evaluate `.treeignore`: {}",
                String::from_utf8_lossy(&ignored.stderr).trim()
            )));
        }
        let excluded: BTreeSet<&[u8]> = ignored.stdout.split(|byte| *byte == 0).collect();
        let mut records = output.stdout.split(|byte| *byte == 0);
        let mut paths = Vec::new();
        while let Some(record) = records.next() {
            if record.is_empty() {
                continue;
            }
            if record.len() < 4 || record[2] != b' ' {
                return Err(Error::Tool("malformed `git status` output".into()));
            }
            let name = &record[3..];
            if record[..2].iter().any(|code| matches!(code, b'R' | b'C')) {
                records.next().ok_or_else(|| {
                    Error::Tool("incomplete rename or copy in `git status` output".into())
                })?;
            }
            if excluded.contains(name) {
                continue;
            }
            let path = std::str::from_utf8(name)
                .map_err(|error| Error::Tool(format!("Git path is not UTF-8: {error}")))?;
            let relative = path.strip_prefix(&project.prefix).ok_or_else(|| {
                Error::Tool("Git returned a path outside the local project directory".into())
            })?;
            let deleted = record[..2].contains(&b'D');
            if deleted || visible_file(&project.root, path)? {
                paths.push(relative.to_owned());
            }
        }
        paths.sort();
        paths.dedup();
        Ok(paths)
    }
}
