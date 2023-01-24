pub mod neutron;
pub mod proton;
pub mod quark;

use neutron::Neutrons;
use proton::Protons;
use quark::Quarks;

#[derive(Debug, Copy, Clone)]
pub struct Block {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub charge: i8,
    pub atom: Atom,
}

#[derive(Debug, Copy, Clone)]
pub struct Atom {
    pub electrons: u8,
    pub nucleus: Nucleus,
}

#[derive(Debug, Copy, Clone)]
pub struct Nucleus {
    pub baryon: Baryon,
}

#[derive(Debug, Copy, Clone)]
pub struct Baryon {
    pub protons: Protons,
    pub neutrons: Neutrons,
}

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

#[derive(Debug, Copy, Clone)]
pub struct Bozons {}

#[derive(Debug, Copy, Clone)]
pub struct Hadrons {}

#[derive(Debug, Copy, Clone)]
pub enum Flip {
    Unknown,
    Zero,
    One,
    NegativeOne,
}

#[derive(Debug, Copy, Clone)]
pub struct MinkowskiSpace {
    pub flips: [Flip; 16],
}
