use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::{AudioContext, AudioNode, BiquadFilterType, OscillatorType, OscillatorNode, GainNode, BiquadFilterNode, AudioParam};

/// Converts a midi note to frequency
///
/// A midi note is an integer, generally in the range of 21 to 108
pub fn midi_to_freq(note: u8) -> f32 {
    27.5 * 2f32.powf((note as f32 - 21.0) / 12.0)
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Envelope {
    pub attack: u32,
    pub decay: u32,
    pub sustain: f32,
    pub release: u32,
}

impl Default for Envelope {
    fn default() -> Envelope {
        Envelope {
            attack: 30, // in milliseconds
            decay: 300, // in milliseconds
            sustain: 1.0, // out of 1
            release: 800, // in milliseconds
        }
    }
}

#[wasm_bindgen]
pub fn default_envelope() -> Envelope {
    Default::default()
}

impl Envelope {
    pub fn adsr(&self) -> (u32, u32, f32, u32) {
        (self.attack, self.decay, self.sustain, self.release)
    }
}

pub struct Filter {
    ctx: AudioContext,
    filter_type: BiquadFilterType,
    pub frequency: u32,
    pub resonance: f32,
    filter: BiquadFilterNode,
}

impl Filter {
    pub fn new(ctx: AudioContext) -> Result<Filter, JsValue> {
        let filter = ctx.create_biquad_filter()?;
        filter.set_type(BiquadFilterType::Lowpass);
        let mut f = Filter {
            frequency: 8000,
            resonance: 0.0,
            ctx: ctx,
            filter: filter,
            filter_type: BiquadFilterType::Lowpass,
        };
        f.set_frequency(f.frequency);
        f.set_resonance(f.resonance);
        Ok(f)
    }

    pub fn set_type(&mut self, filter_type: BiquadFilterType) {
        self.filter_type = filter_type;
        self.filter.set_type(filter_type);
    }

    pub fn set_frequency(&mut self, freq: u32) {
        self.frequency = freq;
        self.filter.frequency().set_value_at_time(freq as f32, self.ctx.current_time()).unwrap();
    }

    pub fn set_resonance(&mut self, q: f32) {
        self.resonance = q;
        self.filter.q().set_value_at_time(q, self.ctx.current_time()).unwrap();
    }
}

pub trait AudioInput {
    fn input(&self) -> AudioNode;
}

pub trait AudioOutput {
    fn output(&self) -> AudioNode;
}

impl AudioInput for Filter {
    fn input(&self) -> AudioNode {
        self.filter.clone().into()
    }
}

impl AudioOutput for Filter {
    fn output(&self) -> AudioNode {
        self.filter.clone().into()
    }
}

impl AudioOutput for Oscillator {
    fn output(&self) -> AudioNode {
        self.amp.clone().into()
    }
}

impl AudioInput for Channel {
    fn input(&self) -> AudioNode {
        self.gain.clone().into()
    }
}

impl AudioOutput for Channel {
    fn output(&self) -> AudioNode {
        self.gain.clone().into()
    }
}

pub trait AudioInputs {
    fn inputs(&self) -> Vec<AudioNode>;
}

pub fn connect<F: AudioOutput, T: AudioInput>(from: &F, to: &T) {
    from.output().connect_with_audio_node(&to.input()).unwrap();
}

pub fn connect_to_one<F: AudioOutput, T: AudioInputs>(from: &F, to: &T, at: usize) {
    let inputs = to.inputs();
    assert!(at < inputs.len(), "index beyond limit");
    from.output().connect_with_audio_node(&inputs[at]).unwrap();
}

use std::collections::HashMap;

pub struct Voice {
    pub unison: usize,
    pub oscs: Vec<OscillatorNode>,
    pub gains: Vec<GainNode>
}

impl Voice {
    pub fn new(ctx: &AudioContext, unison: usize) -> Result<Voice, JsValue> {
        let mut voice = Voice { unison: unison, oscs: vec![], gains: vec![] };
        let now = ctx.current_time();
        for _ in 0..unison {
            let o = ctx.create_oscillator()?;
            let g = ctx.create_gain()?;
            g.gain().set_value_at_time(0.0, now)?;
            o.connect_with_audio_node(&g)?;
            voice.oscs.push(o);
            voice.gains.push(g);
        }
        Ok(voice)
    }

    pub fn start(&self) {
        for o in &self.oscs {
            o.start().unwrap();
        }
    }

    pub fn stop(&self) {
        for o in &self.oscs {
            o.stop().unwrap();
        }
    }

    pub fn set_waveform(&mut self, waveform: OscillatorType) {
        for o in &self.oscs {
            o.set_type(waveform);
        }
    }

    pub fn set_freq(&mut self, ctx: &AudioContext, freq: f32) {
        let now = ctx.current_time();
        for o in &self.oscs {
            o.frequency().set_value_at_time(freq, now).unwrap();
        }
    }

    pub fn amp_envelope_start(&self, ctx: &AudioContext, env: &Envelope, mut max_gain: f32, velocity: u8) {
        let now = ctx.current_time();
        let vel = velocity as f32 / 127.0;
        max_gain = vel * max_gain;
        let attack_s = env.attack as f64 / 1000.0;
        let decay_s = env.decay as f64 / 1000.0;
        for (idx, _) in self.oscs.iter().enumerate() {
            let gain: AudioParam = self.gains[idx].gain();
            gain.linear_ramp_to_value_at_time(max_gain, now + attack_s).unwrap();
            gain.linear_ramp_to_value_at_time(env.sustain * max_gain, now + attack_s + decay_s).unwrap();
        }
    }

    pub fn amp_envelope_end(&self, ctx: &AudioContext, env: &Envelope) {
        let now = ctx.current_time();
        let release_s = env.release as f64 / 1000.0;
        for (idx, _) in self.oscs.iter().enumerate() {
            let gain: AudioParam = self.gains[idx].gain();
            gain.linear_ramp_to_value_at_time(0.0, now + release_s).unwrap();
        }
    }

    pub fn connect_to_audio(&self, to: &AudioNode) {
        for g in &self.gains {
            g.connect_with_audio_node(&to).unwrap();
        }
    }
}

pub struct Oscillator {
    ctx: AudioContext,
    voices: Vec<Voice>,
    last_voice: usize,
    pub env: Envelope,
    pub osc_type: OscillatorType,
    pub polyphony: usize,
    playing_notes: HashMap<u8, usize>,
    amp: GainNode,
    gain: f32,
}

impl Oscillator {
    pub fn new(ctx: AudioContext, polyphony: usize) -> Result<Oscillator, JsValue> {
        let env: Envelope = Default::default();
        let amp = ctx.create_gain()?;

        let mut voices: Vec<Voice> = vec![];
        for _ in 0..polyphony {
            let v = Voice::new(&ctx, 1)?;
            v.connect_to_audio(&amp);
            voices.push(v);
        }

        let osc_type = OscillatorType::Sine;

        let mut o = Oscillator {
            playing_notes: HashMap::new(),
            gain: 0.9,
            polyphony,
            last_voice: 999,
            osc_type,
            ctx,
            voices,
            env,
            amp,
        };

        o.set_waveform(osc_type);

        Ok(o)
    }

    pub fn on(&self) {
        for v in &self.voices {
            v.start();
        }
    }

    pub fn off(&self) {
        for v in &self.voices {
            v.stop();
        }
    }

    fn is_voice_free(&self, idx: usize) -> bool {
        let mut free = true;
        for (_note, voice) in &self.playing_notes {
            if *voice == idx {
                free = false;
            }
        }
        free
    }

    fn get_voice(&self) -> usize {
        let mut voice = if self.last_voice == 999 {
            0
        } else {
            self.last_voice + 1
        };
        if voice > self.polyphony-1 {
            voice = 0;
        }

        for _ in 0..self.polyphony {
            if !self.is_voice_free(voice) {
                voice += 1;
                if voice > self.polyphony-1 {
                    voice = 0;
                }
            }
        }
        voice
    }

    pub fn note_on(&mut self, note: u8, velocity: u8) {
        let current_voice = self.get_voice();
        self.last_voice = current_voice;
        self.playing_notes.insert(note, current_voice);

        let voice = &mut self.voices[current_voice];
        voice.set_freq(&self.ctx, midi_to_freq(note));
        voice.amp_envelope_start(&self.ctx, &self.env, self.gain, velocity);
    }

    pub fn note_off(&mut self, note: u8) {
        match self.playing_notes.get(&note) {
            None => { panic!("Note off without note on") },
            Some(idx) => {
                let voice = &self.voices[*idx];
                voice.amp_envelope_end(&self.ctx, &self.env);
                self.playing_notes.remove(&note);
            }
        }
    }

    pub fn set_waveform(&mut self, waveform: OscillatorType) {
        self.osc_type = waveform;
        for v in &mut self.voices {
            v.set_waveform(waveform);
        }
    }

    pub fn set_gain(&mut self, g: f32) {
        let gain = if g > 1.0 {
            1.0
        } else if g < 0.0 {
            0.0
        } else {
            g
        };
        self.gain = gain;
    }

    pub fn set_env_attack(&mut self, v: u32) {
        self.env.attack = v;
    }
    pub fn set_env_decay(&mut self, v: u32) {
        self.env.decay = v;
    }
    pub fn set_env_sustain(&mut self, v: f32) {
        self.env.sustain = v;
    }
    pub fn set_env_release(&mut self, v: u32) {
        self.env.release = v;
    }

    pub fn connect_with_audio_node(&self, destination: &AudioNode) -> Result<AudioNode, JsValue> {
        let node = self.amp.connect_with_audio_node(&destination)?;
        Ok(node)
    }
  }

#[wasm_bindgen]
pub struct Channel {
    ctx: AudioContext,
    gain: GainNode,
}

#[wasm_bindgen]
impl Channel {
    pub fn new(ctx: AudioContext) -> Result<Channel, JsValue> {
        let gain = ctx.create_gain()?;
        gain.gain().set_value_at_time(0.0, ctx.current_time())?; // start off
        Ok(Channel {
            ctx,
            gain,
        })
    }

    #[wasm_bindgen]
    pub fn set_gain(&self, gain: f32) {
        self.gain.gain().set_value_at_time(gain, self.ctx.current_time()).unwrap();
    }
}

#[wasm_bindgen]
pub struct Mixer {
    channels: Vec<Channel>,
    ctx: AudioContext,
    master: Channel,
}

#[wasm_bindgen]
impl Mixer {
    #[wasm_bindgen(constructor)]
    pub fn new(ctx: AudioContext, channel_count: u8) -> Result<Mixer, JsValue> {
        let master = Channel::new(ctx.clone()).unwrap();

        let channels: Vec<Channel> = (0..channel_count).map(|_| {
            let c = Channel::new(ctx.clone()).unwrap();
            connect(&c, &master);
            c
        }).collect();

        let m = Mixer {
            channels,
            master,
            ctx,
        };

        m.set_master_gain(0.9);
        for idx in 0..channel_count {
            m.set_gain(idx as usize, 0.8);
        }

        Ok(m)
    }

    #[wasm_bindgen]
    pub fn set_master_gain(&self, gain: f32) {
        self.master.set_gain(gain);
    }

    #[wasm_bindgen]
    pub fn set_gain(&self, idx: usize, gain: f32) {
        assert!(self.channels.len() > idx, "Not enough channels");
        self.channels[idx].set_gain(gain);
    }

    #[wasm_bindgen]
    pub fn connect_to_speakers(&self) {
        self.master.output().connect_with_audio_node(&self.ctx.destination()).unwrap();
    }
}

impl AudioInputs for Mixer {
    fn inputs(&self) -> Vec<AudioNode> {
        self.channels.iter().map(|x| x.input()).collect()
    }
}

#[wasm_bindgen]
pub struct Subjam {
    osc1: Oscillator,
    osc2: Oscillator,
    pub osc_mix: f32,
    filter: Filter
}

impl AudioOutput for Subjam {
    fn output(&self) -> AudioNode {
        self.filter.output()
    }
}

#[wasm_bindgen]
impl Subjam {
    #[wasm_bindgen(constructor)]
    pub fn new(ctx: AudioContext) -> Result<Subjam, JsValue> {
        let c1 = ctx.clone();
        let c2 = ctx.clone();
        let c3 = ctx.clone();
        let polyphony = 16;
        let mut osc1 = Oscillator::new(c1, polyphony)?;
        let mut osc2 = Oscillator::new(c2, polyphony)?;
        let filter = Filter::new(c3)?;

        osc1.set_waveform(OscillatorType::Sawtooth);
        osc2.set_waveform(OscillatorType::Square);

        osc1.on();
        osc2.on();

        connect(&osc1, &filter);
        connect(&osc2, &filter);

        let mut subjam = Subjam {
            osc_mix: 0.5,
            osc1,
            osc2,
            filter,
        };

        subjam.set_osc_mix(0.5);

        Ok(subjam)
    }

    #[wasm_bindgen]
    pub fn set_osc1_type(&mut self, waveform: OscillatorType) {
        self.osc1.set_waveform(waveform);
    }

    #[wasm_bindgen]
    pub fn set_osc2_type(&mut self, waveform: OscillatorType) {
        self.osc2.set_waveform(waveform);
    }

    #[wasm_bindgen]
    pub fn get_osc1_type(&self) -> OscillatorType {
        self.osc1.osc_type
    }

    #[wasm_bindgen]
    pub fn get_osc2_type(&self) -> OscillatorType {
        self.osc2.osc_type
    }

    #[wasm_bindgen]
    pub fn set_filter_frequency(&mut self, f: u32) {
        self.filter.set_frequency(f);
    }

    #[wasm_bindgen]
    pub fn set_filter_resonance(&mut self, q: f32) {
        self.filter.set_resonance(q);
    }

    #[wasm_bindgen]
    pub fn get_filter_frequency(&self) -> u32 {
        self.filter.frequency
    }

    #[wasm_bindgen]
    pub fn get_filter_resonance(&self) -> f32 {
        self.filter.resonance
    }

    #[wasm_bindgen]
    pub fn set_env_attack(&mut self, v: u32) {
        self.osc1.set_env_attack(v);
        self.osc2.set_env_attack(v);
    }
    #[wasm_bindgen]
    pub fn set_env_decay(&mut self, v: u32) {
        self.osc1.set_env_decay(v);
        self.osc2.set_env_decay(v);
    }
    #[wasm_bindgen]
    pub fn set_env_sustain(&mut self, v: f32) {
        self.osc1.set_env_sustain(v);
        self.osc2.set_env_sustain(v);
    }
    #[wasm_bindgen]
    pub fn set_env_release(&mut self, v: u32) {
        self.osc1.set_env_release(v);
        self.osc2.set_env_release(v);
    }

    #[wasm_bindgen]
    pub fn get_osc1_env(&self) -> Envelope {
        self.osc1.env.clone()
    }

    #[wasm_bindgen]
    pub fn get_osc2_env(&self) -> Envelope {
        self.osc2.env.clone()
    }

    #[wasm_bindgen]
    pub fn set_osc_mix(&mut self, osc2_gain: f32) {
        self.osc_mix = osc2_gain;
        self.osc2.set_gain(osc2_gain);
        self.osc1.set_gain(1.0 - osc2_gain);
    }

    #[wasm_bindgen]
    pub fn note_on(&mut self, note: u8, velocity: u8) {
        self.osc1.note_on(note, velocity);
        self.osc2.note_on(note, velocity);
    }

    #[wasm_bindgen]
    pub fn note_off(&mut self, note: u8) {
        self.osc1.note_off(note);
        self.osc2.note_off(note);
    }

    #[wasm_bindgen]
    pub fn connect_to_mixer(&self, mixer: &Mixer, at: usize) {
        connect_to_one(self, mixer, at);
    }
}