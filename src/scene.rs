use crate::gfx::Gfx;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};
pub type HashTypeId2Data = HashMap<TypeId, Box<dyn Any>>;
/// Render -> Scene
///        -> Scene is impl Queue (mean its process in sequence)
///        -> Queue just inroduce your Scene
/// Ready And Paint in Scene
///        -> [[add_ready, add_ready, ..],
pub trait Ready<G = Gfx> {
    type Graphics: 'static;
    fn ready(&mut self, data: &mut HashTypeId2Data, gfx: &G);
}
/// Paint can handle lots of code
/// you can use Update or Pass to reduce code in Paint
/// fn paint will be called at frame render
pub trait Paint<G> {
    fn paint(data: &mut HashTypeId2Data, gfx: &G);
}
/// running in Paint function {
///    [update, update, ..]
///       (may also have)
///    [{pass}, {pass}, ..]
/// }
pub trait Update<G> {
    fn update(data: &mut HashTypeId2Data, gfx: &G);
}
/// running in Paint function {
///    [update, update, ..]
///       (may also have)
///    [{pass}, {pass}, ..]
/// }
pub trait Pass<'a> {
    fn pass(data: &mut HashTypeId2Data, render_pass: wgpu::RenderPass<'a>) -> wgpu::RenderPass<'a>;
}

/// Render -> Scene
///        -> Scene is impl Queue (mean its process in sequence)
///        -> Queue just inroduce your Scene
pub trait Queue {
    type G;
    fn introduce(scene: &mut Scene<Self::G>);
}

/// get res from hashmap by type into box data
pub fn get_res<T: Any + 'static>(data: &HashTypeId2Data) -> &T {
    data.get(&TypeId::of::<T>())
        .and_then(|data| data.downcast_ref::<T>())
        .expect(&format!(
            "Failed to get resource of type: {}",
            std::any::type_name::<T>()
        ))
}
/// get ref mut from hashmap by type into box data
pub fn get_res_mut<T: Any + 'static>(data: &mut HashTypeId2Data) -> &mut T {
    data.get_mut(&TypeId::of::<T>())
        .and_then(|data| data.downcast_mut::<T>())
        .expect(&format!(
            "Failed to get mutable resource of type: {}",
            std::any::type_name::<T>()
        ))
}

/// get ref and mut from hashmap by type into box data
pub fn get_ref_and_mut<Ref: Any + 'static, Mut: Any + 'static>(
    data: &mut HashTypeId2Data,
) -> (&Ref, &mut Mut) {
    assert_ne!(
        TypeId::of::<Ref>(),
        TypeId::of::<Mut>(),
        "Ref and Mut should not be the same type"
    );
    unsafe {
        let data_ptr = data as *mut HashTypeId2Data;
        let t1 = (&*data_ptr)
            .get(&TypeId::of::<Ref>())
            .and_then(|r| r.downcast_ref::<Ref>())
            .expect(&format!(
                "Failed to get resource of type: {}",
                std::any::type_name::<Ref>()
            ));
        let t2 = (&mut *data_ptr)
            .get_mut(&TypeId::of::<Mut>())
            .and_then(|d| d.downcast_mut::<Mut>())
            .expect(&format!(
                "Failed to get mutable resource of type: {}",
                std::any::type_name::<Mut>()
            ));
        (t1, t2)
    }
}

/// create a new box data of type in hashmap (directly cover)
pub fn return_res<T: Any + 'static>(data: &mut HashMap<TypeId, Box<dyn Any>>, new_data: T) {
    data.insert(TypeId::of::<T>(), Box::new(new_data));
}

pub struct Scene<G> {
    name: String,
    res: HashMap<TypeId, Box<dyn Any>>,
    readys: Vec<TypeId>,
    paints: Vec<TypeId>,
    readys_hashmap: HashMap<TypeId, Box<dyn FnMut(&mut HashMap<TypeId, Box<dyn Any>>, &G)>>,
    paints_hashmap: HashMap<TypeId, Box<dyn Fn(&mut HashMap<TypeId, Box<dyn Any>>, &G)>>,
}

impl<G: 'static> Scene<G> {
    pub fn new(name: String) -> Self {
        Scene {
            name,
            res: HashMap::new(),
            readys: Vec::new(),
            paints: Vec::new(),
            readys_hashmap: HashMap::new(),
            paints_hashmap: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_ready<T: Ready<G> + Default + 'static>(&mut self, mut ready_res: T) -> &mut Self {
        let type_id = TypeId::of::<T>();
        self.readys.push(type_id);
        self.res.insert(type_id, Box::new(T::default()));
        self.readys_hashmap.insert(
            type_id,
            Box::new(move |data, gfx| {
                ready_res.ready(data, gfx);
            }),
        );

        self
    }

    pub fn add_paint<T: Paint<G> + 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.paints.push(type_id);
        self.paints_hashmap.insert(type_id, Box::new(T::paint));
    }

    pub fn ready(&mut self, gfx: &G) {
        println!("<Scene>::ready");
        for ready_type_id in self.readys.iter() {
            if let Some(ready_fn) = self.readys_hashmap.get_mut(ready_type_id) {
                ready_fn(&mut self.res, gfx);
            }
        }
    }

    pub fn paint(&mut self, gfx: &G) {
        for paint_type_id in self.paints.iter() {
            if let Some(paint_fn) = self.paints_hashmap.get_mut(paint_type_id) {
                paint_fn(&mut self.res, gfx);
            }
        }
    }
}

pub trait DetectGraphics {
    type Graphics: 'static;
}
impl<T> DetectGraphics for T
where
    T: Ready + ?Sized,
{
    type Graphics = <T as Ready>::Graphics;
}