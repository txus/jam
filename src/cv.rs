use std::ops::Range;
use web_sys::GainNode;
use wasm_bindgen::prelude::*;

pub trait Control<T> {
    fn range() -> Range<T>;
    fn set(&mut self, value: T);
    fn get(&self) -> T;
    fn default_value() -> T;
}

#[wasm_bindgen]
pub struct GainControl {
    node: GainNode,
    value: f32
}

impl Control<f32> for GainControl {
    fn range() -> Range<f32> {
        0.0..1.0
    }

    fn set(&mut self, value: f32) {
        self.value = value;
    }

    fn get(&self) -> f32 {
        self.value
    }

    fn default_value() -> f32 { 0.0 }
}


#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct MixControl {
    value: f32
}

impl Control<f32> for MixControl {
    fn range() -> Range<f32> {
        0.0..1.0
    }

    fn set(&mut self, value: f32) {
        self.value = value;
    }

    fn get(&self) -> f32 {
        self.value
    }

    fn default_value() -> f32 { 0.5 }
}
