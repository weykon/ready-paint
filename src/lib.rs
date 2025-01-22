pub mod scene;
pub mod gfx;
pub mod multi;
use crate::gfx::Gfx;
use crate::scene::{
     Queue, Scene,
};

#[derive(Default)]
pub enum RenderEntry {
    #[default]
    NotReady,
    Ready(Gfx),
}

#[derive(Default)]
pub struct Render {
    pub entry: RenderEntry,
    pub scenes: Vec<Scene>,
}

impl Render {
    pub fn new() -> Self {
        Render {
            entry:
                RenderEntry::NotReady,
            scenes: Vec::new(),
        }
    }

    pub fn ready(&mut self) {
        println!("Render::ready");
        match &self.entry {
            RenderEntry::Ready(ref gfx) => {
                for scene in
                    self.scenes.iter_mut()
                {
                    scene.ready(gfx);
                }
            },
            _ => panic!("Render::get_gfx called before gfx is ready"),
        }
    }

    pub fn paint(&mut self) {
        match &self.entry {
            RenderEntry::Ready(ref gfx) => {
                for scene in self.scenes.iter_mut() {
                    scene.paint(gfx);
                }
            },
            _ => panic!("Render::get_gfx called before gfx is ready"),
        }
    }

    pub fn add_scene<T: Queue>(
        &mut self,
        name: impl Into<String> + Clone,
    ) -> &mut Self {
        println!(
            "add_scene: {}",
            name.clone().into()
        );
        let mut s =
            Scene::new(name.into());
        T::introduce(&mut s);
        self.scenes.push(s);
        self
    }
}
