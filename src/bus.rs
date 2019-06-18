use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use web_sys::console;

use std::rc::Rc;
use std::cell::RefCell;

pub struct EventBus {
    controls: Rc<RefCell<HashMap<String, Box<Fn(f32)>>>>,
    last_value: Rc<RefCell<HashMap<String, f32>>>,
    modulations: Rc<RefCell<HashMap<String, Vec<String>>>>
}

impl EventBus {
    pub fn new() -> EventBus {
        EventBus {
            controls: Rc::new(RefCell::new(HashMap::new())),
            last_value: Rc::new(RefCell::new(HashMap::new())),
            modulations: Rc::new(RefCell::new(HashMap::new()))
        }

    }

    pub fn control(&self, id: String, initial_value: f32, on_change: Box<Fn(f32)>) {
        let mut last_value = self.last_value.borrow_mut();
        last_value.insert(id.clone(), initial_value);
        let mut controls = self.controls.borrow_mut();
        if let Some(_) = controls.insert(id, on_change) {
            panic!("Adding control twice");
        }
    }

    pub fn modulate(&self, from: String, to: String) {
        let mut modulations = self.modulations.borrow_mut();
        let entries: &mut Vec<String> = modulations.entry(from).or_insert(vec![]);
        entries.push(to);
    }

    pub fn value(&self, id: String) -> f32 {
        let last_value = self.last_value.borrow();
        if let Some(v) = last_value.get(&id) {
            *v
        } else {
            0.0
        }
    }

    pub fn trigger(&self, id: String, value: f32) {
        let controls = self.controls.borrow();

        console::log_1(&"trying to trigger".into());

        if let Some(f) = controls.get(&id) {
            let mut last_value = self.last_value.borrow_mut();
            last_value.insert(id.clone(), value);
            f(value);
        }
        let modulations = self.modulations.borrow();
        if let Some(modulations) = modulations.get(&id) {
            for modulated_id in modulations {
                self.trigger(modulated_id.to_string(), value);
            }
        }
    }
}
