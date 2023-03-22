use std::{sync::{Arc, atomic::AtomicBool}, fmt::Debug};

#[derive(Clone, Default)]
pub struct Position(Arc<AtomicBool>);
impl Position {
    pub fn is_absolute(&self) -> bool {
        self.0.load(std::sync::atomic::Ordering::Relaxed)
    }
    pub fn is_relative(&self) -> bool {
        !self.0.load(std::sync::atomic::Ordering::Relaxed)
    }
    pub fn set_absolute(&self) {
        self.0.store(true, std::sync::atomic::Ordering::Relaxed)
    }
    pub fn set_relative(&self) {
        self.0.store(false, std::sync::atomic::Ordering::Relaxed)
    }
}
impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.is_absolute() { "Absolute" } else { "Relative" })
    }
}