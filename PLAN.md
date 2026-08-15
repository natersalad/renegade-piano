# Renegade Piano

## Goal

Turn the ROC-RK3328-CC into a lightweight, headless piano sound module. It
should boot ready to play without a laptop or DAW and eventually support
multiple sounds, drums, and controls mapped from the Arturia keyboard.

## Signal Flow

Arturia keyboard -> USB MIDI -> Renegade -> FluidSynth -> USB audio interface
-> speakers

The latency-sensitive note path should stay inside ALSA and FluidSynth. The
Rust application will configure, start, monitor, and reconnect those pieces.

## Development Workflow

- Write code locally on the Mac with Neovim
- Run tests and Clippy locally
- Track small units of progress with Git
- Run `cargo sync` to copy source to the Renegade
- Compile and run hardware integration tests on the Renegade

## Current Software

- Armbian Debian 13 on the Renegade
- Rust and Cargo 1.85 on the Renegade
- FluidSynth 2.4.4
- ALSA utilities (`aconnect` and `aplay`)
- TimGM6mb SoundFont at `/usr/share/sounds/sf2/TimGM6mb.sf2`
- Rust CLI with `check` and `run` commands

TimGM6mb is a small proof-of-concept SoundFont. We can select a better piano
after the live signal path is stable.

## Completed

- [x] Boot Armbian Debian 13
- [x] Configure SSH over Tailscale
- [x] Establish safe operating temperature by positioning the board vertically
- [x] Create the Rust project, configuration model, and unit tests
- [x] Add `cargo sync` for Mac-to-Renegade deployment
- [x] Validate required programs and the configured SoundFont
- [x] Build and pass tests on both macOS and ARM64 Linux
- [x] Load the SoundFont and render an audible piano note to a WAV file

## Next Hardware Session

### 1. Identify the hardware

- Record the exact Arturia keyboard model
- Record the exact Steinberg audio-interface model
- Record their USB vendor and product IDs

### 2. Prove live audio output

Connect only the Steinberg interface first.

- Confirm it appears in `lsusb`
- Inspect ALSA playback devices with `aplay -l` and `aplay -L`
- Send a test signal through the interface
- Start FluidSynth with the interface selected explicitly
- Confirm audio reaches headphones or powered speakers
- Record the stable ALSA device name in `config.toml`

Success means the Renegade can produce audible piano audio through the
Steinberg interface without the MIDI keyboard.

### 3. Prove MIDI input

Connect the Arturia keyboard after audio output works.

- Confirm it appears in `lsusb`
- Inspect ALSA sequencer ports with `aconnect -l`
- Inspect incoming events with `aseqdump`
- Verify note-on, note-off, velocity, and sustain-pedal events
- Record the stable ALSA MIDI device name in `config.toml`

Success means key presses and pedal changes are visible as MIDI events on the
Renegade.

### 4. Complete the live signal path

- Start FluidSynth with ALSA audio and ALSA sequencer MIDI
- Connect the Arturia MIDI output to FluidSynth
- Play through the Steinberg interface
- Check for stuck notes, crackling, disconnects, and noticeable latency

Success means pressing a key produces sound directly from the speakers without
using the Mac.

## Later Milestones

- [ ] Discover audio and MIDI devices by stable names
- [ ] Start FluidSynth from the Rust `run` command
- [ ] Reconnect automatically after USB disconnection
- [ ] Start automatically with a systemd service
- [ ] Tune sample rate, period size, CPU use, and latency
- [ ] Configure real-time audio scheduling without running as root
- [ ] Map Arturia controls to presets and actions
- [ ] Add multiple sounds, drums, splits, and layers
- [ ] Evaluate a higher-quality piano SoundFont

## Initial Performance Targets

- Playable within 30 seconds of boot
- End-to-end latency at or below 15 ms
- Reconnect within 5 seconds after a USB interruption
- Play for 30 minutes without audio dropouts
- Work without SSH, Tailscale, or internet access
- Remain below 70 degrees Celsius during normal use

## Open Questions

- What is the exact Arturia keyboard model?
- What is the exact Steinberg audio-interface model?
- Which ALSA device identifiers remain stable across reboots?
- What buffer size is stable on the RK3328 with the selected interface?
