use std::sync::Arc;
use ready_paint::*;
use winit::{application::ApplicationHandler, dpi::PhysicalSize, window::Window};

fn main() {
    pollster::block_on(async {
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        let mut app = App::default();
        let _ = event_loop.run_app(&mut app);
    })
}

impl ApplicationHandler for App {
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
                            return;
                        }
                        println!("resize ready");
                        self.render.ready();
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
            winit::event::WindowEvent::RedrawRequested => {
                if let RenderEntry::Ready(ref mut gfx) = self.render.entry {
                    if gfx.surface_config.is_some() {
                        let _ = gfx.test();
                    }
                }
            }
            _ => (),
        }
    }
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let RenderEntry::NotReady = self.render.entry {
            let window = event_loop
                .create_window(
                    winit::window::Window::default_attributes()
                        .with_title("spatial hash".to_string()),
                )
                .unwrap();

            let main_owner_window = Arc::new(window);
            self.window = Some(main_owner_window.clone());
            let gfx = pollster::block_on(async move {
                println!("in async : Loading");
                let gfx = gfx::Gfx::new(main_owner_window.clone()).await;
                println!("in async : Ready");
                return gfx;
            });
            println!("out of async");
            self.render.entry = RenderEntry::Ready(gfx);
            println!("resumed: set render entry ready");
            self.first_resize = true;
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
