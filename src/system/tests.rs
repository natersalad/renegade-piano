use super::{RequirementStatus, SystemReport, probe_soundfont};
use std::path::Path;

const SOUNDFONT: &str = "/sounds/piano.sf2";

#[test]
fn reports_ready_when_every_requirement_is_available() {
    let report = SystemReport::inspect_with(
        Path::new(SOUNDFONT),
        |_| RequirementStatus::Available,
        |_| RequirementStatus::Available,
    );

    assert!(report.is_ready());
    assert!(report.ensure_ready().is_ok());
}

#[test]
fn reports_not_ready_when_a_program_is_missing() {
    let report = SystemReport::inspect_with(
        Path::new(SOUNDFONT),
        |program| {
            if program == "aconnect" {
                RequirementStatus::Missing
            } else {
                RequirementStatus::Available
            }
        },
        |_| RequirementStatus::Available,
    );

    assert!(!report.is_ready());
    assert!(report.ensure_ready().is_err());
    assert_eq!(report.checks[1].name, "aconnect");
    assert_eq!(report.checks[1].status, RequirementStatus::Missing);
}

#[test]
fn reports_not_ready_when_a_program_cannot_run() {
    let report = SystemReport::inspect_with(
        Path::new(SOUNDFONT),
        |program| {
            if program == "fluidsynth" {
                RequirementStatus::Failed("permission denied".to_string())
            } else {
                RequirementStatus::Available
            }
        },
        |_| RequirementStatus::Available,
    );

    assert!(!report.is_ready());
    assert_eq!(
        report.checks[0].status,
        RequirementStatus::Failed("permission denied".to_string())
    );
}

#[test]
fn reports_not_ready_when_soundfont_is_missing() {
    let report = SystemReport::inspect_with(
        Path::new(SOUNDFONT),
        |_| RequirementStatus::Available,
        |_| RequirementStatus::Missing,
    );

    assert!(!report.is_ready());
    assert_eq!(report.checks[3].name, "soundfont");
    assert_eq!(report.checks[3].detail, SOUNDFONT);
    assert_eq!(report.checks[3].status, RequirementStatus::Missing);
}

#[test]
fn recognizes_a_regular_file() {
    assert_eq!(
        probe_soundfont(Path::new("Cargo.toml")),
        RequirementStatus::Available
    );
}
