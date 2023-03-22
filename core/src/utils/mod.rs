use winit::dpi::PhysicalSize;

mod logger;       pub use logger::*;
mod color;        pub use color::*;
mod initializer;  pub use initializer::*;
mod coord;        pub use coord::*;
mod background;   pub use background::*;
mod position;     pub use position::*;

#[cfg(feature = "screenshot")]
mod screenshot;

#[derive(Copy, Clone)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32
}
impl Into<WindowSize> for PhysicalSize<u32> {
    fn into(self) -> WindowSize {
        WindowSize { width: self.width as f32, height: self.height as f32 }
    }
}