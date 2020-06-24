use std::ops::Deref;

use crate::state::State;

pub struct Store {
    state: StateWrapper,
    listeners: Vec<Box<dyn Fn() -> ()>>,
}

impl Store {
    pub fn new(state: State) -> Self {
        Self {
            listeners: vec![],
            state: StateWrapper(state),
        }
    }

    pub fn add_image(&mut self, image_url: &str) {
        self.state.add_image(image_url);
    }

    pub fn next(&mut self) -> String {
        self.state.next()
    }

    pub fn cur_image(&self) -> String {
        self.state.cur_image()
    }

    pub fn update(&self) {
        for callback in self.listeners.iter() {
            callback();
        }
    }

    pub fn subscribe(&mut self, callback: Box<dyn Fn() -> ()>) {
        self.listeners.push(callback);
    }
}

impl Deref for Store {
    type Target = State;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.state
    }
}

pub struct StateWrapper(State);

impl Deref for StateWrapper {
    type Target = State;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl StateWrapper {
    pub fn add_image(&mut self, image_url: &str) {
        self.0.add_image(image_url);
    }

    pub fn next(&mut self) -> String {
        self.0.next()
    }

    pub fn cur_image(&self) -> String {
        self.0.cur_image()
    }
}
