use std::sync::Arc;
use wgpu::RequestAdapterOptions;
use winit::window::Window;
pub struct Gfx {
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'static>,
    pub surface_config: Option<wgpu::SurfaceConfiguration>,
    pub last_update: std::time::Instant,
    pub time: Arc<std::sync::Mutex<f32>>,
    pub limit_fps: LimitFPS,
    pub fps_history: Vec<f32>,
    pub delta_time: f32,
}
impl Gfx {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::default();

        let adapter = instance
            .request_adapter(&RequestAdapterOptions::default())
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();

        let surface = instance.create_surface(window.clone()).unwrap();

        Gfx {
            device,
            queue,
            surface,
            adapter,
            surface_config: None,
            last_update: std::time::Instant::now(),
            time: Arc::new(std::sync::Mutex::new(0.0)),
            limit_fps: LimitFPS::default(),
            fps_history: Vec::new(),
            delta_time: 0.0,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let mut surface_config = self
            .surface
            .get_default_config(&self.adapter, width, height)
            .unwrap();
        self.surface.configure(&self.device, &surface_config);
        let view_format = surface_config.format.add_srgb_suffix();
        surface_config.view_formats.push(view_format);
        self.surface_config = Some(surface_config);
    }

    pub fn test(&self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}

#[derive(PartialEq)]
pub enum LimitFPS {
    Limit(f32),
    NoLimit,
}
impl Default for LimitFPS {
    fn default() -> Self {
        LimitFPS::Limit(30.0)
    }
}