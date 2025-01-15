


use ready_paint::scene::{Paint, Pass, Update};
use crate::{object::Tetrahedron, world::World};
pub struct PaintScene;

impl Paint for PaintScene {
    fn paint(data: &mut ready_paint::scene::HashTypeId2Data, gfx: &ready_paint::gfx::Gfx) {
        let mut encoder = gfx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        let frame = gfx.surface.get_current_texture().unwrap();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let _ = World::update(data, gfx);
        let _ = Tetrahedron::update(data, gfx);
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            let mut rpass = World::pass(data, &mut rpass);
            let _ = Tetrahedron::pass(data, &mut rpass);
        }
        gfx.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
