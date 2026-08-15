use serde::Deserialize;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[cfg(test)]
mod tests;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub audio: AudioConfig,
    pub midi: MidiConfig,
    pub synth: SynthConfig,
}

#[derive(Debug, Deserialize)]
pub struct AudioConfig {
    pub device: String,
    pub sample_rate: u32,
    pub period_size: u32,
}

#[derive(Debug, Deserialize)]
pub struct MidiConfig {
    pub device: String,
}

#[derive(Debug, Deserialize)]
pub struct SynthConfig {
    pub soundfont: String,
    pub gain: f32,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("could not read configuration: {0}")]
    Read(#[from] std::io::Error),

    #[error("could not parse configuration: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("invalid configuration: {0}")]
    Validation(String),
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let contents = fs::read_to_string(path)?;

        Self::parse(&contents)
    }

    fn parse(contents: &str) -> Result<Self, ConfigError> {
        let config: Self = toml::from_str(contents)?;

        config.validate()?;

        Ok(config)
    }

    fn validate(&self) -> Result<(), ConfigError> {
        if self.audio.device.trim().is_empty() {
            return Err(ConfigError::Validation(
                "audio.device cannot be empty".to_string(),
            ));
        }

        if self.audio.sample_rate == 0 {
            return Err(ConfigError::Validation(
                "audio.sample_rate must be greater than zero".to_string(),
            ));
        }

        if self.audio.period_size == 0 {
            return Err(ConfigError::Validation(
                "audio.period_size must be greater than zero".to_string(),
            ));
        }

        if self.midi.device.trim().is_empty() {
            return Err(ConfigError::Validation(
                "midi.device cannot be empty".to_string(),
            ));
        }

        if self.synth.soundfont.trim().is_empty() {
            return Err(ConfigError::Validation(
                "synth.soundfont cannot be empty".to_string(),
            ));
        }

        if !self.synth.gain.is_finite() || self.synth.gain <= 0.0 || self.synth.gain > 10.0 {
            return Err(ConfigError::Validation(
                "synth.gain must be greater than 0 and no greater than 10".to_string(),
            ));
        }

        Ok(())
    }
}
