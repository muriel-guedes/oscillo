use winit::dpi::PhysicalSize;

use crate::Context;

#[allow(unused_variables)]
pub trait Script {
    fn setup(&self, _c: Context) {}
    fn update(&self, _c: Context) {}
    fn resize(&self, _c: Context, _new_size: PhysicalSize<u32>) {}
}