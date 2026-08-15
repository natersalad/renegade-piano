use std::process::{Command, ExitCode};

const DESTINATION: &str = "renegade:dev/renegade-piano/";

fn main() -> ExitCode {
    let dry_run = std::env::args()
        .skip(1)
        .any(|argument| argument == "--dry-run");

    println!("syncing project to {DESTINATION}");

    match Command::new("rsync")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(rsync_args(dry_run))
        .status()
    {
        Ok(status) if status.success() => {
            println!("sync complete");
            ExitCode::SUCCESS
        }
        Ok(status) => {
            eprintln!("sync failed: rsync exited with {status}");
            ExitCode::FAILURE
        }
        Err(error) => {
            eprintln!("sync failed: could not start rsync: {error}");
            ExitCode::FAILURE
        }
    }
}

fn rsync_args(dry_run: bool) -> Vec<&'static str> {
    let mut arguments = vec!["-az", "--exclude", ".git/", "--exclude", "target/"];

    if dry_run {
        arguments.push("--dry-run");
    }

    arguments.extend(["./", DESTINATION]);
    arguments
}

#[cfg(test)]
mod tests {
    use super::{DESTINATION, rsync_args};

    #[test]
    fn excludes_git_metadata_and_build_artifacts() {
        let arguments = rsync_args(false);

        assert!(
            arguments
                .windows(2)
                .any(|pair| pair == ["--exclude", ".git/"])
        );
        assert!(
            arguments
                .windows(2)
                .any(|pair| pair == ["--exclude", "target/"])
        );
        assert_eq!(arguments[arguments.len() - 2..], ["./", DESTINATION]);
        assert!(!arguments.contains(&"--delete"));
    }

    #[test]
    fn supports_dry_run() {
        assert!(rsync_args(true).contains(&"--dry-run"));
    }
}
