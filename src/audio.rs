use web_sys::AudioNode;

pub trait AudioInput {
    fn input(&self) -> AudioNode;
}

pub trait AudioOutput {
    fn output(&self) -> AudioNode;
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
