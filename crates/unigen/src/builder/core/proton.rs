use super::quark::{Quark, QuarkData};

#[derive(Debug, Copy, Clone)]
pub struct Protons {
    pub count: u8,
    pub protons: [ProtonData; 118],
}

impl Protons {
    pub fn new(count: u8) -> Self {
        let mut protons = [ProtonData::Unknown; 118];

        protons[0..(count as usize)]
            .iter_mut()
            .for_each(|p| *p = ProtonData::new(Proton::new()));

        Self { count, protons }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Proton {
    pub quarks: (Quark, Quark, Quark),
}

impl Proton {
    pub const fn new() -> Self {
        Self {
            quarks: (Quark::new(0, 1), Quark::new(1, 1), Quark::new(2, 0)),
        }
    }
}

impl Default for Proton {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ProtonData {
    Unknown,

    RedBlueGreenUpUpDownQuark,
    RedBlueAlphaUpUpDownQuark,
    RedGreenBlueUpUpDownQuark,
    RedGreenAlphaUpUpDownQuark,
    RedAlphaBlueUpUpDownQuark,
    RedAlphaGreenUpUpDownQuark,

    GreenBlueRedUpUpDownQuark,
    GreenBlueAlphaUpUpDownQuark,
    GreenRedBlueUpUpDownQuark,
    GreenRedAlphaUpUpDownQuark,
    GreenAlphaBlueUpUpDownQuark,
    GreenAlphaRedUpUpDownQuark,

    BlueGreenRedUpUpDownQuark,
    BlueGreenAlphaUpUpDownQuark,
    BlueRedGreenUpUpDownQuark,
    BlueRedAlphaUpUpDownQuark,
    BlueAlphaGreenUpUpDownQuark,
    BlueAlphaRedUpUpDownQuark,

    AlphaBlueRedUpUpDownQuark,
    AlphaBlueGreenUpUpDownQuark,
    AlphaRedBlueUpUpDownQuark,
    AlphaRedGreenUpUpDownQuark,
    AlphaGreenRedUpUpDownQuark,
    AlphaGreenBlueUpUpDownQuark,
}

impl ProtonData {
    pub const fn new(proton: Proton) -> Self {
        let first_quark: QuarkData = Quark::data(proton.quarks.0);
        let second_quark: QuarkData = Quark::data(proton.quarks.1);
        let third_quark: QuarkData = Quark::data(proton.quarks.2);

        match first_quark {
            QuarkData::RedUpQuark => match (second_quark, third_quark) {
                (QuarkData::BlueUpQuark, QuarkData::GreenDownQuark) => {
                    ProtonData::RedBlueGreenUpUpDownQuark
                }

                (QuarkData::BlueUpQuark, QuarkData::AlphaDownQuark) => {
                    ProtonData::RedBlueAlphaUpUpDownQuark
                }

                (QuarkData::GreenUpQuark, QuarkData::BlueDownQuark) => {
                    ProtonData::RedGreenBlueUpUpDownQuark
                }

                (QuarkData::GreenUpQuark, QuarkData::AlphaDownQuark) => {
                    ProtonData::RedGreenAlphaUpUpDownQuark
                }

                (QuarkData::AlphaUpQuark, QuarkData::BlueDownQuark) => {
                    ProtonData::RedAlphaGreenUpUpDownQuark
                }

                (QuarkData::AlphaUpQuark, QuarkData::GreenDownQuark) => {
                    ProtonData::RedAlphaGreenUpUpDownQuark
                }

                _ => ProtonData::Unknown,
            },

            QuarkData::GreenUpQuark => match (second_quark, third_quark) {
                (QuarkData::BlueUpQuark, QuarkData::RedDownQuark) => {
                    ProtonData::GreenBlueRedUpUpDownQuark
                }

                (QuarkData::BlueUpQuark, QuarkData::AlphaDownQuark) => {
                    ProtonData::GreenBlueAlphaUpUpDownQuark
                }

                (QuarkData::RedUpQuark, QuarkData::BlueDownQuark) => {
                    ProtonData::GreenRedBlueUpUpDownQuark
                }

                (QuarkData::RedUpQuark, QuarkData::AlphaDownQuark) => {
                    ProtonData::GreenRedAlphaUpUpDownQuark
                }

                (QuarkData::AlphaUpQuark, QuarkData::BlueDownQuark) => {
                    ProtonData::GreenAlphaBlueUpUpDownQuark
                }

                (QuarkData::AlphaUpQuark, QuarkData::RedDownQuark) => {
                    ProtonData::RedBlueGreenUpUpDownQuark
                }

                _ => ProtonData::Unknown,
            },

            QuarkData::BlueUpQuark => match (second_quark, third_quark) {
                (QuarkData::GreenUpQuark, QuarkData::RedDownQuark) => {
                    ProtonData::BlueGreenRedUpUpDownQuark
                }

                (QuarkData::GreenUpQuark, QuarkData::AlphaDownQuark) => {
                    ProtonData::BlueGreenAlphaUpUpDownQuark
                }

                (QuarkData::RedUpQuark, QuarkData::GreenDownQuark) => {
                    ProtonData::BlueRedGreenUpUpDownQuark
                }

                (QuarkData::RedUpQuark, QuarkData::AlphaDownQuark) => {
                    ProtonData::BlueRedAlphaUpUpDownQuark
                }

                (QuarkData::AlphaUpQuark, QuarkData::GreenDownQuark) => {
                    ProtonData::BlueAlphaGreenUpUpDownQuark
                }

                (QuarkData::AlphaUpQuark, QuarkData::RedDownQuark) => {
                    ProtonData::BlueAlphaRedUpUpDownQuark
                }

                _ => ProtonData::Unknown,
            },

            QuarkData::AlphaUpQuark => match (second_quark, third_quark) {
                (QuarkData::BlueUpQuark, QuarkData::RedDownQuark) => {
                    ProtonData::AlphaBlueRedUpUpDownQuark
                }

                (QuarkData::BlueUpQuark, QuarkData::GreenDownQuark) => {
                    ProtonData::AlphaBlueGreenUpUpDownQuark
                }

                (QuarkData::RedUpQuark, QuarkData::BlueDownQuark) => {
                    ProtonData::AlphaRedBlueUpUpDownQuark
                }

                (QuarkData::RedUpQuark, QuarkData::GreenDownQuark) => {
                    ProtonData::AlphaRedGreenUpUpDownQuark
                }

                (QuarkData::GreenUpQuark, QuarkData::RedDownQuark) => {
                    ProtonData::AlphaGreenRedUpUpDownQuark
                }

                (QuarkData::GreenUpQuark, QuarkData::BlueDownQuark) => {
                    ProtonData::AlphaGreenBlueUpUpDownQuark
                }

                _ => ProtonData::Unknown,
            },

            _ => ProtonData::Unknown,
        }
    }
}

#[test]
fn it_defaults_all_protons_as_unknown() {
    let protons = Protons::new(0);

    for proton in protons.protons.iter() {
        assert_eq!(proton, &ProtonData::Unknown);
    }
}

#[test]
fn it_can_match_a_single_proton_correctly() {
    let protons = Protons::new(1);

    let first_proton = protons.protons[0];

    assert_eq!(first_proton, ProtonData::RedGreenBlueUpUpDownQuark);
}
