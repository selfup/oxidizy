#[derive(Debug, Copy, Clone)]
pub struct Atom {
    pub electrons: u32,
    pub nucleus: Nucleus,
}

#[derive(Debug, Copy, Clone)]
pub struct Nucleus {
    pub protons: Protons,
    pub neutrons: Neutrons,
}

#[derive(Debug, Copy, Clone)]
pub struct Block {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub charge: i8,
    pub atom: Atom,
}

#[derive(Debug, Copy, Clone)]
pub struct Protons {
    pub count: u32,
    pub protons: [Proton; 118],
}

impl Protons {
    pub fn new(count: u32) -> Protons {
        let protons = [Proton::new(); 118];
        
        Protons {
            count, 
            protons,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Proton {
    //Protons need up, up, down quarks
    pub quarks: (Quark, Quark, Quark),
}

impl Proton {
    pub fn new() -> Proton {
        Proton {
            quarks: (Quark::new(0,0), Quark::new(1,1), Quark::new(2,1))
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Neutrons {
    pub count: u32,
    pub neutrons: [Neutron; 118],
}

impl Neutrons {
    pub fn new(count: u32) -> Neutrons {
        let neutrons = [Neutron::new(); 118];
        
        Neutrons {
            count, 
            neutrons,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Neutron {
    //Neutrons need up, down, down quarks
    pub quarks: (Quark, Quark, Quark),
}

impl Neutron {
    pub fn new() -> Neutron {
        Neutron {
            quarks: (Quark::new(0,1), Quark::new(1,0), Quark::new(2,0))
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Quarks {
    pub u: Quark,
    pub d: Quark,
    pub c: Quark,
    pub s: Quark,
    pub t: Quark,
    pub b: Quark,
}

#[derive(Debug, Copy, Clone)]
pub struct Quark {
    pub color: StrongCharge,
    pub elementary_charge: ElectricCharge,
}

impl Quark {
    pub fn new(colorcharge: u8, electriccharge: u8) -> Quark {
        Quark{
            color: StrongCharge::new(colorcharge),
            elementary_charge: ElectricCharge::new(electriccharge),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum StrongCharge {
    Red,
    Green,
    Blue,
    Alpha,
}

impl StrongCharge {
    pub fn new(color: u8) -> StrongCharge {
        match color {
            0 => StrongCharge::Red,
            1 => StrongCharge::Green,
            2 => StrongCharge::Blue,
            _ => StrongCharge::Alpha,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ElectricCharge {
    PositiveTwoThirds,
    NegativeOneThird,
}

impl ElectricCharge {
    pub fn new(charge: u8) -> ElectricCharge {
        match charge {
            0 => ElectricCharge::NegativeOneThird,
            _ => ElectricCharge::PositiveTwoThirds,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Baryons {}

#[derive(Debug, Copy, Clone)]
pub struct Fermion {
    pub quarks: Quarks,
    pub leptons: Leptons,
}

#[derive(Debug, Copy, Clone)]
pub struct Leptons {
    pub e: Lepton,
    pub ve: Lepton,
    pub u: Lepton,
    pub vu: Lepton,
    pub t: Lepton,
    pub vt: Lepton,
}

#[derive(Debug, Copy, Clone)]
pub struct Lepton {}
