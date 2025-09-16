

pub struct Motherboard {
    pub cpu: CPU,

}


impl Motherboard<'_> {
    pub fn new<'a>(motherboard: &Motherboard) -> Motherboard<'a> {
        motherboard.cpu = self
    }
}
