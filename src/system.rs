use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use thiserror::Error;

#[cfg(test)]
mod tests;

const PROGRAMS: &[(&str, &str)] = &[
    ("fluidsynth", "SoundFont synthesizer"),
    ("aconnect", "ALSA MIDI routing"),
    ("aplay", "ALSA audio diagnostics"),
];

#[derive(Debug, PartialEq, Eq)]
enum RequirementStatus {
    Available,
    Missing,
    Failed(String),
}

#[derive(Debug, PartialEq, Eq)]
struct RequirementCheck {
    name: &'static str,
    detail: String,
    status: RequirementStatus,
}

#[derive(Debug)]
pub struct SystemReport {
    checks: Vec<RequirementCheck>,
}

#[derive(Debug, Error)]
#[error("one or more system requirements are unavailable")]
pub struct SystemCheckError;

impl SystemReport {
    pub fn inspect(soundfont: &Path) -> Self {
        Self::inspect_with(soundfont, probe_program, probe_soundfont)
    }

    fn inspect_with(
        soundfont: &Path,
        mut program_probe: impl FnMut(&str) -> RequirementStatus,
        mut soundfont_probe: impl FnMut(&Path) -> RequirementStatus,
    ) -> Self {
        let mut checks: Vec<_> = PROGRAMS
            .iter()
            .map(|&(name, purpose)| RequirementCheck {
                name,
                detail: purpose.to_string(),
                status: program_probe(name),
            })
            .collect();

        checks.push(RequirementCheck {
            name: "soundfont",
            detail: soundfont.display().to_string(),
            status: soundfont_probe(soundfont),
        });

        Self { checks }
    }

    fn is_ready(&self) -> bool {
        self.checks
            .iter()
            .all(|check| check.status == RequirementStatus::Available)
    }

    pub fn print(&self) {
        for check in &self.checks {
            match &check.status {
                RequirementStatus::Available => {
                    println!("✓ {}: {}", check.name, check.detail);
                }
                RequirementStatus::Missing => {
                    println!("✗ {}: not found ({})", check.name, check.detail);
                }
                RequirementStatus::Failed(reason) => {
                    println!("✗ {}: {} ({})", check.name, reason, check.detail);
                }
            }
        }
    }

    pub fn ensure_ready(&self) -> Result<(), SystemCheckError> {
        if self.is_ready() {
            Ok(())
        } else {
            Err(SystemCheckError)
        }
    }
}

fn probe_program(program: &str) -> RequirementStatus {
    match Command::new(program).arg("--version").output() {
        Ok(_) => RequirementStatus::Available,
        Err(error) if error.kind() == io::ErrorKind::NotFound => RequirementStatus::Missing,
        Err(error) => RequirementStatus::Failed(error.to_string()),
    }
}

fn probe_soundfont(path: &Path) -> RequirementStatus {
    match fs::metadata(path) {
        Ok(metadata) if metadata.is_file() => RequirementStatus::Available,
        Ok(_) => RequirementStatus::Failed("not a regular file".to_string()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => RequirementStatus::Missing,
        Err(error) => RequirementStatus::Failed(error.to_string()),
    }
}
