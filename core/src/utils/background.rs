use std::{sync::{Arc, Mutex}, fmt::Debug};

use super::Color;

#[derive(Copy, Clone, Debug)]
pub enum BackgroundType {
    Solid(Color),
    None
}
impl Default for BackgroundType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Default)]
pub struct Background(Arc<Mutex<BackgroundType>>);
impl Background {
    pub fn get(&self) -> BackgroundType {
        *self.0.lock().unwrap()
    }
    pub fn set(&self, v: BackgroundType) {
        *self.0.lock().unwrap() = v
    }
}
impl Debug for Background {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.get())
    }
}