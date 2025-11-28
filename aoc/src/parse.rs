pub trait Parse {
    const L: u8;
}

pub struct A {}

impl Parse for A {
    const L: u8 = 3;
}
