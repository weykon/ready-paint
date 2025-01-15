use paint::PaintScene;
use ready_paint::scene::Queue;

pub struct NiceViewScene;
impl Queue for NiceViewScene {
    fn introduce(scene: &mut ready_paint::scene::Scene) {
        scene
            .add_ready(world::World::default())
            .add_ready(object::Tetrahedron::default());
        scene.add_paint::<PaintScene>();
    }
}

pub mod object;
pub mod paint;
mod run;
pub mod world;
fn main() {
    pollster::block_on(run::run());
}
