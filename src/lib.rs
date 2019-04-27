use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::{AudioContext, AudioNode, BiquadFilterType, OscillatorType, OscillatorNode, GainNode, BiquadFilterNode, AudioParam};

/// Converts a midi note to frequency
///
/// A midi note is an integer, generally in the range of 21 to 108
pub fn midi_to_freq(note: u8) -> f32 {
    27.5 * 2f32.powf((note as f32 - 21.0) / 12.0)
}

pub struct Envelope {
    attack: u32,
    decay: u32,
    sustain: f32,
    release: u32,
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

impl Envelope {
    pub fn adsr(&self) -> (u32, u32, f32, u32) {
        (self.attack, self.decay, self.sustain, self.release)
    }
    pub fn default_adsr() -> (u32, u32, f32, u32) {
        let env: Envelope = Default::default();
        env.adsr()
    }
}

pub struct Oscillator {
    ctx: AudioContext,
    osc: OscillatorNode,
    pub env: Envelope,
    amp: GainNode,
    gain: f32,
}

impl Oscillator {
    pub fn new(ctx: AudioContext) -> Result<Oscillator, JsValue> {
        let osc = ctx.create_oscillator()?;
        let env: Envelope = Default::default();
        let amp = ctx.create_gain()?;

        amp.gain().set_value_at_time(0.0, ctx.current_time())?; // start off

        osc.connect_with_audio_node(&amp)?;

        Ok(Oscillator {
            gain: 0.9,
            ctx,
            osc,
            env,
            amp,
        })
    }

    pub fn on(&self) -> Result<(), JsValue> {
        let x = self.osc.start()?;
        Ok(x)
    }

    pub fn off(&self) -> Result<(), JsValue> {
        let x = self.osc.stop()?;
        Ok(x)
    }

    pub fn start(&self, velocity: u8) {
        let now = self.now();
        let gain: AudioParam = self.amp.gain();
        let vel = velocity as f32 / 127.0;
        let max_gain = self.gain * vel;
        let attack_s = self.env.attack as f64 / 1000.0;
        let decay_s = self.env.decay as f64 / 1000.0;
        gain.linear_ramp_to_value_at_time(max_gain, now + attack_s).unwrap();
        gain.linear_ramp_to_value_at_time(self.env.sustain * max_gain, now + attack_s + decay_s).unwrap();
    }

    pub fn pitch(&self, note: u8) {
        let now = self.ctx.current_time();
        let freq = midi_to_freq(note);
        self.osc.frequency().set_value_at_time(freq, now).unwrap();
    }

    pub fn stop(&self) {
        let now = self.now();
        let gain: AudioParam = self.amp.gain();
        let release_s = self.env.release as f64 / 1000.0;
        gain.linear_ramp_to_value_at_time(0.0, now + release_s).unwrap();
    }

    pub fn note_on(&self, note: u8, velocity: u8) {
        self.pitch(note);
        self.start(velocity);
    }

    pub fn note_off(&self, _note: u8) {
        self.stop();
    }

    pub fn set_waveform(&self, waveform: OscillatorType) {
        self.osc.set_type(waveform);
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

    fn now(&self) -> f64 {
        let now = self.ctx.current_time();
        let gain: AudioParam = self.amp.gain();
        gain.cancel_scheduled_values(now).unwrap();
        gain.set_value_at_time(self.amp.gain().value(), now).unwrap();
        now
    }

    pub fn connect_with_audio_node(&self, destination: &AudioNode) -> Result<AudioNode, JsValue> {
        let node = self.amp.connect_with_audio_node(&destination)?;
        Ok(node)
    }
  }

pub struct Subjam {
    ctx: AudioContext,
    pub osc1: Oscillator,
    pub osc2: Oscillator
}

#[wasm_bindgen]
impl Subjam {
    #[wasm_bindgen(constructor)]
    pub fn new(ctx: AudioContext) -> Result<Subjam, JsValue> {
        let c1 = ctx.clone();
        let c2 = ctx.clone();
        let mut osc1 = Oscillator::new(c1)?;
        let mut osc2 = Oscillator::new(c2)?;

        osc1.set_waveform(OscillatorType::Sawtooth);
        osc2.set_waveform(OscillatorType::Square);

        osc1.set_gain(0.9);
        osc2.set_gain(0.7);

        osc1.on()?;
        osc2.on()?;

        //let filter = ctx.create_biquad_filter()?;
        //filter.set_type(BiquadFilterType::Lowpass);
        //filter.connect_with_audio_node(&amp1)?;

        osc1.connect_with_audio_node(&ctx.destination())?;
        osc2.connect_with_audio_node(&ctx.destination())?;

        Ok(Subjam {
            ctx: ctx,
            osc1,
            osc2,
        })
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
    pub fn set_amp_mix(&mut self, osc1_gain: f32) {
        self.osc1.set_gain(osc1_gain);
        self.osc2.set_gain(1.0 - osc1_gain);
    }

    #[wasm_bindgen]
    pub fn note_on(&self, note: u8, velocity: u8) {
        self.osc1.note_on(note, velocity);
        self.osc2.note_on(note, velocity);
    }

    #[wasm_bindgen]
    pub fn note_off(&self, note: u8) {
        self.osc1.note_off(note);
        self.osc2.note_off(note);
    }
}

impl Drop for Subjam {
    fn drop(&mut self) {
        let _ = self.ctx.close();
    }
}