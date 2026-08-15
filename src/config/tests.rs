use super::{Config, ConfigError};

const VALID_CONFIG: &str = r#"
[audio]
device = "Steinberg"
sample_rate = 48000
period_size = 128

[midi]
device = "Arturia"

[synth]
soundfont = "/sounds/piano.sf2"
gain = 0.7
"#;

#[test]
fn parses_valid_configuration() {
    let config = Config::parse(VALID_CONFIG).expect("valid configuration should parse");

    assert_eq!(config.audio.device, "Steinberg");
    assert_eq!(config.audio.sample_rate, 48_000);
    assert_eq!(config.audio.period_size, 128);
    assert_eq!(config.midi.device, "Arturia");
    assert_eq!(config.synth.soundfont, "/sounds/piano.sf2");
    assert_eq!(config.synth.gain, 0.7);
}

#[test]
fn rejects_missing_required_section() {
    let incomplete = r#"
    [audio]
    device = "Steinberg"
    sample_rate = 48000
    period_size = 128
    "#;

    let error = Config::parse(incomplete).expect_err("missing sections should fail");

    assert!(matches!(error, ConfigError::Parse(_)));
}

#[test]
fn rejects_zero_sample_rate() {
    let invalid = VALID_CONFIG.replace("sample_rate = 48000", "sample_rate = 0");

    let error = Config::parse(&invalid).expect_err("zero sample rate should fail");

    assert!(matches!(
        error,
        ConfigError::Validation(message)
            if message == "audio.sample_rate must be greater than zero"
    ));
}

#[test]
fn rejects_empty_midi_device() {
    let invalid = VALID_CONFIG.replace("device = \"Arturia\"", "device = \"   \"");

    let error = Config::parse(&invalid).expect_err("empty MIDI device should fail");

    assert!(matches!(
        error,
        ConfigError::Validation(message) if message == "midi.device cannot be empty"
    ));
}

#[test]
fn rejects_gain_outside_fluidsynth_range() {
    for gain in [0.0, 10.1, f32::INFINITY, f32::NAN] {
        let mut config = Config::parse(VALID_CONFIG).expect("fixture should parse");
        config.synth.gain = gain;

        let error = config
            .validate()
            .expect_err("out-of-range gain should fail");

        assert!(matches!(
            error,
            ConfigError::Validation(message)
                if message == "synth.gain must be greater than 0 and no greater than 10"
        ));
    }
}
