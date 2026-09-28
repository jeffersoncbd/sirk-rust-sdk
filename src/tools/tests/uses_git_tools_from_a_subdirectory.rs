use super::super::Tools;
use std::{
    fs,
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn uses_git_tools_from_a_subdirectory() {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "sirk-sdk-tools-{}-{stamp}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(root.join("src")).unwrap();
    Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(&root)
        .status()
        .unwrap();
    fs::write(root.join(".treeignore"), "src/hidden.rs\n").unwrap();
    fs::write(root.join("src/tracked.rs"), "old").unwrap();
    fs::write(root.join("src/hidden.rs"), "hidden").unwrap();
    Command::new("git")
        .args(["add", "."])
        .current_dir(&root)
        .status()
        .unwrap();
    Command::new("git")
        .args([
            "-c",
            "user.email=test@example.com",
            "-c",
            "user.name=Test",
            "commit",
            "--quiet",
            "-m",
            "initial",
        ])
        .current_dir(&root)
        .status()
        .unwrap();
    fs::write(root.join("src/tracked.rs"), "new").unwrap();
    fs::write(root.join("src/untracked.rs"), "new").unwrap();
    let tools = Tools {
        directory: root.join("src"),
    };
    assert_eq!(tools.tree().unwrap(), ["tracked.rs", "untracked.rs"]);
    assert_eq!(
        tools.git().status().unwrap(),
        ["tracked.rs", "untracked.rs"]
    );
    tools.git().add().unwrap();
    assert_eq!(
        String::from_utf8(
            Command::new("git")
                .args(["diff", "--cached", "--name-only"])
                .current_dir(&root)
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap(),
        "src/tracked.rs\nsrc/untracked.rs\n"
    );
    fs::remove_dir_all(root).unwrap();
}
