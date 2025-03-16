use ready_paint::{
    multi::{refs_muts, Mut, Read, Ref},
    scene::{DetectGraphics, HashTypeId2Data, Paint, Ready},
};

fn main() {
    let mut data = HashTypeId2Data::default();
    refs_muts::<(Ref<HereReady>, Mut<SecondReady>)>(&mut data);
}

struct CustomGfx {}

impl DetectGraphics for CustomGfx {
    type Graphics = CustomGfx;
}

struct HereReady {}
impl Ready<CustomGfx> for HereReady {
    type Graphics = CustomGfx;
    fn ready(&mut self, _: &mut HashTypeId2Data, _: &CustomGfx) {}
}
struct SecondReady {}
impl Ready<CustomGfx> for SecondReady {
    type Graphics = CustomGfx;
    fn ready(&mut self, _: &mut HashTypeId2Data, _: &CustomGfx) {}
}

struct HerePaint {}
impl Paint<CustomGfx> for HerePaint {
    fn paint(data: &mut HashTypeId2Data, gfx: &CustomGfx) {}
}
