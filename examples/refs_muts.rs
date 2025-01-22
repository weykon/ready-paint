use ready_paint::gfx::Gfx;
use ready_paint::scene::HashTypeId2Data;
use std::any::TypeId;

use ready_paint::{
    multi::{refs_muts, Mut, Ref},
    scene::Ready,
};
#[derive(Debug)]
struct Data1 {
    val: i32,
}
#[derive(Debug)]
struct Data2 {
    val: i32,
}
impl Ready for Data1 {
    fn ready(&mut self, _: &mut HashTypeId2Data, _: &Gfx) {}
}
impl Ready for Data2 {
    fn ready(&mut self, _: &mut HashTypeId2Data, _: &Gfx) {}
}

fn main() {
    use std::any::Any;
    use std::collections::HashMap;
    let mut data: HashMap<TypeId, Box<dyn Any>> = HashMap::new();
    data.insert(TypeId::of::<Data1>(), Box::new(Data1 { val: 42 }));
    data.insert(TypeId::of::<Data2>(), Box::new(Data2 { val: 3 }));

    let (r1, r2) = refs_muts::<(Ref<Data1>, Mut<Data2>)>(&mut data);
    r2.val = 5;
    assert_eq!(r1.val, 42);
    assert_eq!(r2.val, 5);

    println!("r1.val is {:}", r1.val);
    println!("r2.val is {:}", r2.val);
}
