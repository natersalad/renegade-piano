# Renegade Piano

## Goal

I'm trying to trun my ROC-RK3328-CC into a headless piano sound module that boots ready to play without requiring to connect my midi to my laptop and open up my daw. The idea is that this will be lightweight and always on. I want everything to be controlled via my keyboard and have a config for keybinds but for the keys of my keyboard. I want there to be multiple sounds, and drums, etc.

## Signal Flow
Arturia keyboard -> USB MIDI -> Renegade -> software instrument -> USB audio interface (my Steinburg22) -> speakers

## Dev workflow
While I am using codex for this project, I want to use this a learning experience for neovim and learn vim motions and such. I want to find a good workflow for me.

- Write code locally through my mac with Neovim
- Track with git
- Sync on the renegade to test

## Milestones
- [x] Boot Armbian Debian 13
- [x] Configure SSH over Tailscale
- [x] Looked into not overheating my renegade (just put it vertical lol)
- [ ] Detect the Arturia Lab MIDI controller
- [ ] Detect the Steinburg audio interface 
- [ ] Produce a sound test 
- [ ] Route the MIDI into a software instrument
- [ ] Start the piano automatically at boot
- [ ] Recover automatically after USB disconnection
- [ ] Tune latency and audio
- [ ] Add a config file for all buttons on my Artura Lab

## Initial Software Direction

Start with FluidSynth and a small SoundFont. Prove the complete path before selecting a larger piano library

## Open Questions
- get exact keyboard model
- get exact audio interface model
