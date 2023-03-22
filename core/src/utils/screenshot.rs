use std::{num::NonZeroU32, sync::mpsc::channel};
use image::{ImageBuffer, Rgba};

use crate::Context;

const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

impl Context {
    pub fn screenshot(&self, width: u32, height: u32) {
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            view_formats: &[FORMAT],
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: FORMAT,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None
        });
        let texture_view = texture.create_view(&Default::default());

        let u32_size = std::mem::size_of::<u32>() as u32;

        let output_buffer_desc = wgpu::BufferDescriptor {
            size: (u32_size * width * height) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            label: None,
            mapped_at_creation: false
        };
        let output_buffer = self.device.create_buffer(&output_buffer_desc);

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let containers = self.containers.lock().unwrap().clone();
        for container in containers {
            container.render(self, &mut encoder, &texture_view, false);
        }
        
        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO
            },
            wgpu::ImageCopyBuffer {
                buffer: &output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(NonZeroU32::new(u32_size * width).unwrap()),
                    rows_per_image: Some(NonZeroU32::new(height).unwrap())
                }
            },
            texture.size()
        );

        self.queue.submit(Some(encoder.finish()));

        {
            let buffer_slice = output_buffer.slice(..);
        
            let (tx, rx) = channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });
            self.device.poll(wgpu::Maintain::Wait);
            rx.recv().unwrap().unwrap();
        
            let mut data: Vec<u8> = buffer_slice.get_mapped_range().to_vec();
            data.reverse();

            let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, data).unwrap();
            buffer.save("image.png").unwrap();
        }
        output_buffer.unmap();
    }
}