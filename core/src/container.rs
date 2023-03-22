use std::sync::{Arc, Mutex};
use wgpu::{util::DeviceExt, CommandEncoder, TextureView};
use winit::dpi::PhysicalSize;

use crate::{Context, Script, Element};

#[derive(Clone, Default)]
pub struct Container {
    scripts: Arc<Mutex<Vec<Arc<Box<dyn Script>>>>>,
    pub root: Element,
    buffer: Arc<Mutex<Option<wgpu::Buffer>>>,
    bind_group: Arc<Mutex<Option<wgpu::BindGroup>>>
}
impl Container {
    pub fn add_script(&self, script: impl Script + 'static) -> &Self {
        self.scripts.lock().unwrap().push(Arc::new(Box::new(script)));
        self
    }
    
    pub fn setup(&self, c: &Context) {
        self.root.style.width.set_perc(1.);
        self.root.style.height.set_perc(1.);
        for script in self.scripts.lock().unwrap().iter() {
            script.setup(c.clone())
        }
    }

    pub fn resize(&self, c: &Context, new_size: PhysicalSize<u32>) {
        for script in self.scripts.lock().unwrap().iter() {
            script.resize(c.clone(), new_size)
        }
    }

    pub fn update(&self, c: &Context) {
        let scripts = self.scripts.lock().unwrap().clone();
        for script in scripts {
            script.update(c.clone());
        }
        
        let window_size = c.window.inner_size().into();
        let mut data = self.root.get_data(window_size);

        if data.len() == 0 {
            data.append(&mut vec![0.;4]);
        }
        
        let data: &[u8] = bytemuck::cast_slice(&data);

        let mut buffer = self.buffer.lock().unwrap();
        let mut bind_group = self.bind_group.lock().unwrap();

        if data.len() == 0 {
            *buffer = None;
            *bind_group = None;
        } else {
            match buffer.as_mut() {
                Some(buffer) => {
                    if buffer.size() >= data.len() as u64 {
                        c.queue.write_buffer(buffer, 0, data);
                    } else {
                        *buffer = c.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: None,
                            contents: data,
                            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST
                        });
                        *bind_group = None
                    }
                }
                None => {
                    *buffer = Some(c.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: None,
                        contents: data,
                        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST
                    }));
                    *bind_group = None
                }
            }
            if bind_group.is_none() {
                *bind_group = Some(c.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &c.shader.get_bind_group_layout(0),
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_ref().unwrap().as_entire_binding()
                    }]
                }))
            }
        }
    }
    pub fn render(&self, c: &Context, encoder: &mut CommandEncoder, view: &TextureView, set_scissor_rect: bool) {
        let bind_group = self.bind_group.lock().unwrap();
        let bind_group = match bind_group.as_ref() { Some(v) => v, None => return };
        
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: true }
            })],
            depth_stencil_attachment: None
        });
        if set_scissor_rect {
            self.root.set_scissor_rect(&mut render_pass, c.window.inner_size().into());
        }
        render_pass.set_pipeline(&c.shader);
        render_pass.set_bind_group(0, bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }
}