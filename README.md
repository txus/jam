# Jam

![Screenshot](/screenshot.png?raw=true "Screenshot")

This will be hopefully a complete, extensible and playable modular synth running in the browser.

It's written in Rust (interfacing with WebAudio API), compiled to WebAssembly and run in the browser -- the UI is a Vue.js application. If you have a MIDI controller, just plug it in and the browser should recognize it (probably just Chrome)!

For now I'm just playing around! Definitely not very usable.

## Modules

### Subjam

A basic monophonic 2-oscillator synth with an amp envelope.
