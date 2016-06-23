//#[derive(Debug)]
//pub struct Proton {
//    pub charge: i8 = 1,
//    pub location: (i64, i64, i64),
//}
//
//#[derive(Debug)]
//pub struct Neutron {
//    pub charge: i8 = 0,
//    pub location: (i64, i64, i64),
//}
//
//#[derive(Debug)]
//pub struct Electron {
//    pub charge: i8 = -1,
//    pub location: (i64, i64, i64),
//}


#[derive(Debug)]
pub struct Nucleus {
    pub protons:  i8,
    pub neutrons: i8,
}

#[derive(Debug)]
pub struct Atom {
    pub electrons: i8,
    pub nucleus: Nucleus,
    pub location: (i64, i64, i64)
}

impl Atom {
    pub fn charge(&self) -> i8 {
        self.nucleus.protons - self.electrons
    }
}
