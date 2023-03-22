use std::{sync::{Arc, Mutex, atomic::AtomicBool}, collections::HashSet};
use wgpu::{Surface, Device, Queue, SurfaceConfiguration, Instance, RenderPipeline};
use winit::{window::{Window, WindowBuilder}, event_loop::EventLoop, dpi::PhysicalSize};

use crate::{Key, Container, utils, shader};

#[derive(Clone)]
pub struct Context {
    pub window: Arc<Window>,
    surface: Arc<Surface>,
    pub(crate) device: Arc<Device>,
    pub(crate) queue: Arc<Queue>,
    surface_config: Arc<Mutex<SurfaceConfiguration>>,
    exit: Arc<AtomicBool>,
    keys_pressed: Arc<Mutex<HashSet<Key>>>,
    pub(crate) containers: Arc<Mutex<Vec<Container>>>,
    pub(crate) shader: Arc<RenderPipeline>
}
impl Context {
    pub(crate) fn new(event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new().build(event_loop).unwrap();
        let instance = Instance::new(Default::default());
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = utils::create_adapter(&instance, &surface);
        let (device, queue) = utils::create_device_queue(&adapter);
        let surface_config = utils::configure_surface(&window, &device, &adapter, &surface);
        let shader = shader::new(&device, surface_config.format);
        
        Self {
            window: window.into(),
            surface: surface.into(),
            device: device.into(),
            queue: queue.into(),
            surface_config: Arc::new(Mutex::new(surface_config)),
            exit: AtomicBool::new(false).into(),
            keys_pressed: Default::default(),
            containers: Default::default(),
            shader: shader.into()
        }
    }
    
    pub(crate) fn request_redraw(&self) {
        self.window.request_redraw()
    }

    pub(crate) fn key_pressed(&self, key: Key) {
        self.keys_pressed.lock().unwrap().insert(key);
    }
    pub(crate) fn key_released(&self, key: &Key) {
        self.keys_pressed.lock().unwrap().remove(key);
    }
    pub fn is_key_pressed(&self, key: &Key) -> bool {
        self.keys_pressed.lock().unwrap().contains(key)
    }
    
    pub fn exit(&self) {
        self.exit.store(true, std::sync::atomic::Ordering::Relaxed)
    }
    pub(crate) fn is_running(&self) -> bool {
        !self.exit.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn new_container(&self) -> Container {
        let container = Container::default();
        self.containers.lock().unwrap().push(container.clone());
        container
    }

    pub(crate) fn resize(&self, new_size: PhysicalSize<u32>) {
        let mut surface_config = self.surface_config.lock().unwrap();
        surface_config.width = new_size.width;
        surface_config.height = new_size.height;
        self.surface.configure(&self.device, &surface_config);
        for container in self.containers.lock().unwrap().iter() {
            container.resize(self, new_size)
        }
    }

    pub(crate) fn setup(&self) {
        for container in self.containers.lock().unwrap().iter() {
            container.setup(self)
        }
    }

    pub(crate) fn update(&self) {
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let output_texture = match self.surface.get_current_texture() {
            Ok(v) => v,
            Err(wgpu::SurfaceError::Lost) | Err(wgpu::SurfaceError::Outdated) => return self.resize(self.window.inner_size()),
            Err(e) => panic!("Error getting current surface texture: {}", e)
        };
        let view = output_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let containers = self.containers.lock().unwrap().clone();
        for container in containers {
            container.update(self);
            container.render(self, &mut encoder, &view, true);
        }
        
        self.queue.submit(Some(encoder.finish()));
        output_texture.present();
    }
}