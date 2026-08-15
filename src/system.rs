use std::io;
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
enum ProgramStatus {
    Available,
    Missing,
    Failed(String),
}

#[derive(Debug, PartialEq, Eq)]
struct ProgramCheck {
    name: &'static str,
    purpose: &'static str,
    status: ProgramStatus,
}

#[derive(Debug)]
pub struct SystemReport {
    checks: Vec<ProgramCheck>,
}

#[derive(Debug, Error)]
#[error("one or more required system programs are unavailable")]
pub struct SystemCheckError;

impl SystemReport {
    pub fn inspect() -> Self {
        Self::inspect_with(probe_program)
    }

    fn inspect_with(mut probe: impl FnMut(&str) -> ProgramStatus) -> Self {
        let checks = PROGRAMS
            .iter()
            .map(|&(name, purpose)| ProgramCheck {
                name,
                purpose,
                status: probe(name),
            })
            .collect();

        Self { checks }
    }

    fn is_ready(&self) -> bool {
        self.checks
            .iter()
            .all(|check| check.status == ProgramStatus::Available)
    }

    pub fn print(&self) {
        for check in &self.checks {
            match &check.status {
                ProgramStatus::Available => {
                    println!("✓ {}: {}", check.name, check.purpose);
                }
                ProgramStatus::Missing => {
                    println!("✗ {}: not found ({})", check.name, check.purpose);
                }
                ProgramStatus::Failed(reason) => {
                    println!("✗ {}: {} ({})", check.name, reason, check.purpose);
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

fn probe_program(program: &str) -> ProgramStatus {
    match Command::new(program).arg("--version").output() {
        Ok(_) => ProgramStatus::Available,
        Err(error) if error.kind() == io::ErrorKind::NotFound => ProgramStatus::Missing,
        Err(error) => ProgramStatus::Failed(error.to_string()),
    }
}
