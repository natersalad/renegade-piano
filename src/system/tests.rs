use super::{ProgramStatus, SystemReport};

#[test]
fn reports_ready_when_every_program_is_available() {
    let report = SystemReport::inspect_with(|_| ProgramStatus::Available);

    assert!(report.is_ready());
    assert!(report.ensure_ready().is_ok());
}

#[test]
fn reports_not_ready_when_a_program_is_missing() {
    let report = SystemReport::inspect_with(|program| {
        if program == "aconnect" {
            ProgramStatus::Missing
        } else {
            ProgramStatus::Available
        }
    });

    assert!(!report.is_ready());
    assert!(report.ensure_ready().is_err());
    assert_eq!(report.checks[1].name, "aconnect");
    assert_eq!(report.checks[1].status, ProgramStatus::Missing);
}

#[test]
fn reports_not_ready_when_a_program_cannot_run() {
    let report = SystemReport::inspect_with(|program| {
        if program == "fluidsynth" {
            ProgramStatus::Failed("permission denied".to_string())
        } else {
            ProgramStatus::Available
        }
    });

    assert!(!report.is_ready());
    assert_eq!(
        report.checks[0].status,
        ProgramStatus::Failed("permission denied".to_string())
    );
}
