use gfx::LimitFPS;
use glam::Vec2;
use ready_paint::{time::now, *};
use scene::{get_res, return_res, HashTypeId2Data, Paint, Pass, Queue, Ready, Scene};
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::{application::ApplicationHandler, dpi::PhysicalSize, window::Window};

impl Queue for RectScene {
    fn introduce(scene: &mut Scene) {
        scene.add_ready(RectScene::default());
        scene.add_paint::<PaintScreen>();
    }
}
struct RectScene {
    pub size: Vec2,
    pub index_buffer: Option<wgpu::Buffer>,
    pub render_pipeline: Option<wgpu::RenderPipeline>,
    pub vertexes_buffer: Option<wgpu::Buffer>,
}
impl Default for RectScene {
    fn default() -> Self {
        Self {
            size: Vec2::new(0.1, 0.1),
            index_buffer: None,
            render_pipeline: None,
            vertexes_buffer: None,
        }
    }
}
impl Ready for RectScene {
    fn ready(&mut self, data: &mut HashTypeId2Data, gfx: &gfx::Gfx) {
        let size = self.size;
        let four_point_rect: [[f32; 2]; 4] = [
            [-size.x, size.y],  // Left Up
            [size.x, size.y],   // Right Up
            [size.x, -size.y],  // Right Down
            [-size.x, -size.y], // Left Down
        ];
        let vertexes_buffer = gfx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("screen vertex buffer"),
                contents: bytemuck::cast_slice(&four_point_rect.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let index_buffer = gfx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("screen index buffer"),
                contents: bytemuck::cast_slice(&[0u16, 1, 2, 3, 1, 2, 3, 0]),
                usage: wgpu::BufferUsages::INDEX,
            });
        let vertexes_layout = [wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vec2>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: 0,
            }],
        }];
        let screen_shader = gfx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("screen shader"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(SHADER)),
            });
        let config = gfx.surface_config.as_ref().unwrap();
        let render_pipeline = gfx
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("screen render pipeline"),
                layout: None,
                vertex: wgpu::VertexState {
                    module: &screen_shader,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &vertexes_layout,
                },
                primitive: wgpu::PrimitiveState {
                    cull_mode: Some(wgpu::Face::Back),
                    topology: wgpu::PrimitiveTopology::LineList,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &screen_shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(config.view_formats[0].into())],
                }),
                multiview: None,
                cache: None,
            });
        return_res(
            data,
            RectScene {
                render_pipeline: Some(render_pipeline),
                index_buffer: Some(index_buffer),
                vertexes_buffer: Some(vertexes_buffer),
                ..Default::default()
            },
        )
    }
}

struct PaintScreen;
impl<'a> Pass<'a> for PaintScreen {
    fn pass(
        data: &mut HashTypeId2Data,
        mut render_pass: wgpu::RenderPass<'a>,
    ) -> wgpu::RenderPass<'a> {
        let screen = get_res::<RectScene>(data);
        let index_buffer = screen.index_buffer.as_ref().unwrap();
        let render_pipeline = screen.render_pipeline.as_ref().unwrap();
        let vertexes_buffer = screen.vertexes_buffer.as_ref().unwrap();
        render_pass.set_pipeline(render_pipeline);
        render_pass.set_vertex_buffer(0, vertexes_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..8 as u32, 0, 0..1);
        return render_pass;
    }
}

impl Paint for PaintScreen {
    fn paint(data: &mut HashTypeId2Data, gfx: &gfx::Gfx) {
        let frame = gfx.surface.get_current_texture().unwrap();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = gfx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            let _ = PaintScreen::pass(data, render_pass);
        }
        gfx.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
    }
}

const SHADER: &str = r#"
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};
@vertex
fn vs_main(
    @location(0) line_point_pos: vec2<f32>,
) -> VertexOutput {
    var result: VertexOutput;
    let position = vec2<f32>(line_point_pos.x , -line_point_pos.y);  // Y轴需要翻转
    result.position = vec4<f32>(position, 0.0, 1.0);
    return result;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 1.0, 0.0, 1.0); // 绿色线框
}
"#;

// -------------- normal code --------------
fn main() {
    pollster::block_on(async {
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        let mut app = App::default();
        let _ = event_loop.run_app(&mut app);
    })
}
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let RenderEntry::NotReady = self.render.entry {
            let window = event_loop
                .create_window(
                    winit::window::Window::default_attributes()
                        .with_title("queue read paint example".to_string()),
                )
                .unwrap();
            let main_owner_window = Arc::new(window);
            self.window = Some(main_owner_window.clone());
            let gfx = pollster::block_on(async move {
                let gfx = gfx::Gfx::new(main_owner_window.clone()).await;
                return gfx;
            });
            self.render.entry = RenderEntry::Ready(gfx);
            // add scene
            self.render.add_scene::<RectScene>("rect line");
            self.first_resize = true;
        }
    }
    fn about_to_wait(&mut self, _: &winit::event_loop::ActiveEventLoop) {
        if let RenderEntry::Ready(ref mut gfx) = self.render.entry {
            let _now = now();

            #[cfg(not(target_arch = "wasm32"))]
            {
                let delta_time = _now - gfx.last_update;

                let time = *gfx.time.lock().unwrap() + delta_time.as_secs_f32();
                if let LimitFPS::Limit(fps) = gfx.limit_fps {
                    let frame_duration = std::time::Duration::from_secs_f32(1.0 / fps as f32);
                    if delta_time < frame_duration {
                        spin_sleep::sleep(frame_duration - delta_time);
                    }
                }
                gfx.last_update = now();
                *gfx.time.lock().unwrap() = time;
                let real_delta = gfx.last_update - _now;
                gfx.delta_time = real_delta.as_secs_f32();
                self.window.as_ref().unwrap().request_redraw();
            }
            #[cfg(target_arch = "wasm32")]
            {
                let delta_time = _now - gfx.last_update;
                let time = *gfx.time.lock().unwrap() + delta_time as f32;
                if let LimitFPS::Limit(fps) = gfx.limit_fps {
                    let frame_duration = std::time::Duration::from_secs_f32(1.0 / fps as f32);
                    if delta_time < frame_duration.as_secs_f32() {
                        #[cfg(not(target_arch = "wasm32"))]
                        spin_sleep::sleep(frame_duration - delta_time);
                        #[cfg(target_arch = "wasm32")]
                        wasm_bindgen_futures::spawn_local(async move {
                            wasm_timer::Delay::new(frame_duration).await.unwrap();
                        });
                    }
                    gfx.last_update = now();
                    *gfx.time.lock().unwrap() = time;
                    let real_delta = gfx.last_update - _now;
                    gfx.delta_time = real_delta;
                    self.window.as_ref().unwrap().request_redraw();
                }
            }
        }
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::Resized(size) => {
                println!("winit event: resized");
                let PhysicalSize { width, height } = size;
                match self.render.entry {
                    RenderEntry::Ready(ref mut gfx) => {
                        gfx.resize(width, height);
                        if self.first_resize {
                            self.first_resize = false;
                            self.render.ready();
                            return;
                        }
                        println!("resize ready");
                        self.window.as_ref().unwrap().request_redraw();
                    }
                    _ => {
                        println!("resize not ready");
                    }
                }
            }
            winit::event::WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            winit::event::WindowEvent::RedrawRequested => match self.render.entry {
                RenderEntry::Ready(ref mut gfx) => {
                    if gfx.surface_config.is_none() {
                        return;
                    }
                    let _ = gfx.test();
                    self.render.paint();
                }
                _ => {
                    println!("redraw not ready");
                }
            },
            _ => (),
        }
    }
}
#[derive(Default)]
pub struct App {
    pub window: Option<Arc<Window>>,
    pub wgpu_instance: Option<wgpu::Instance>,
    pub render: Render,
    pub first_resize: bool,
}
// -------------- normal code --------------
