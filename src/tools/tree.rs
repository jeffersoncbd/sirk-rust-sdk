use super::Tools;
use super::project::project;
use crate::Error;
use std::collections::BTreeSet;

impl Tools {
    pub fn tree(&self) -> Result<Vec<String>, Error> {
        let project = project(&self.directory)?;
        let pathspec = if project.prefix.is_empty() {
            "."
        } else {
            project.prefix.trim_end_matches('/')
        };
        let output = std::process::Command::new("git")
            .args([
                "ls-files",
                "--cached",
                "--others",
                "--exclude-standard",
                "-z",
                "--",
            ])
            .arg(pathspec)
            .current_dir(&project.root)
            .output()
            .map_err(|error| Error::Tool(format!("could not run Git for TREE: {error}")))?;
        if !output.status.success() {
            return Err(Error::Tool(format!(
                "TREE requires an accessible Git working tree: {}",
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
        let mut files = Vec::new();
        for path in output
            .stdout
            .split(|byte| *byte == 0)
            .filter(|path| !path.is_empty())
        {
            if excluded.contains(path) {
                continue;
            }
            let name = std::str::from_utf8(path)
                .map_err(|error| Error::Tool(format!("Git path is not UTF-8: {error}")))?;
            let relative = name.strip_prefix(&project.prefix).ok_or_else(|| {
                Error::Tool("Git returned a path outside the local project directory".into())
            })?;
            let metadata = match std::fs::symlink_metadata(project.root.join(name)) {
                Ok(metadata) => metadata,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
                Err(error) => {
                    return Err(Error::Tool(format!("could not inspect `{name}`: {error}")));
                }
            };
            if metadata.is_file() || metadata.file_type().is_symlink() {
                files.push(relative.to_owned());
            }
        }
        files.sort();
        files.dedup();
        Ok(files)
    }
}
