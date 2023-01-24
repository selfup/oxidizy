use super::quark::{Quark, QuarkData};

#[derive(Debug, Copy, Clone)]
pub struct Neutrons {
    pub count: u8,
    pub neutrons: [NeutronData; 118],
}

impl Neutrons {
    pub fn new(count: u8) -> Self {
        let mut neutrons = [NeutronData::Unknown; 118];

        neutrons[0..(count as usize)]
            .iter_mut()
            .for_each(|n| *n = NeutronData::new(Neutron::new()));

        Self { count, neutrons }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Neutron {
    pub quarks: (Quark, Quark, Quark),
}

impl Neutron {
    pub const fn new() -> Self {
        Self {
            quarks: (Quark::new(0, 1), Quark::new(1, 0), Quark::new(2, 0)),
        }
    }
}

impl Default for Neutron {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NeutronData {
    Unknown,

    RedBlueGreenUpDownDownQuark,
    RedBlueAlphaUpDownDownQuark,
    RedGreenBlueUpDownDownQuark,
    RedGreenAlphaUpDownDownQuark,
    RedAlphaBlueUpDownDownQuark,
    RedAlphaGreenUpDownDownQuark,

    GreenBlueRedUpDownDownQuark,
    GreenBlueAlphaUpDownDownQuark,
    GreenRedBlueUpDownDownQuark,
    GreenRedAlphaUpDownDownQuark,
    GreenAlphaBlueUpDownDownQuark,
    GreenAlphaRedUpDownDownQuark,

    BlueGreenRedUpDownDownQuark,
    BlueGreenAlphaUpDownDownQuark,
    BlueRedGreenUpDownDownQuark,
    BlueRedAlphaUpDownDownQuark,
    BlueAlphaGreenUpDownDownQuark,
    BlueAlphaRedUpDownDownQuark,

    AlphaBlueRedUpDownDownQuark,
    AlphaBlueGreenUpDownDownQuark,
    AlphaRedBlueUpDownDownQuark,
    AlphaRedGreenUpDownDownQuark,
    AlphaGreenRedUpDownDownQuark,
    AlphaGreenBlueUpDownDownQuark,
}

impl NeutronData {
    pub const fn new(neutron: Neutron) -> Self {
        let first_quark: QuarkData = Quark::data(neutron.quarks.0);
        let second_quark: QuarkData = Quark::data(neutron.quarks.1);
        let third_quark: QuarkData = Quark::data(neutron.quarks.2);

        match first_quark {
            QuarkData::RedUpQuark => match (second_quark, third_quark) {
                (QuarkData::BlueDownQuark, QuarkData::GreenDownQuark) => {
                    NeutronData::RedBlueGreenUpDownDownQuark
                }

                (QuarkData::BlueDownQuark, QuarkData::AlphaDownQuark) => {
                    NeutronData::RedBlueAlphaUpDownDownQuark
                }

                (QuarkData::GreenDownQuark, QuarkData::BlueDownQuark) => {
                    NeutronData::RedGreenBlueUpDownDownQuark
                }

                (QuarkData::GreenDownQuark, QuarkData::AlphaDownQuark) => {
                    NeutronData::RedGreenAlphaUpDownDownQuark
                }

                (QuarkData::AlphaDownQuark, QuarkData::BlueDownQuark) => {
                    NeutronData::RedAlphaGreenUpDownDownQuark
                }

                (QuarkData::AlphaDownQuark, QuarkData::GreenDownQuark) => {
                    NeutronData::RedAlphaGreenUpDownDownQuark
                }

                _ => NeutronData::Unknown,
            },

            QuarkData::GreenUpQuark => match (second_quark, third_quark) {
                (QuarkData::BlueDownQuark, QuarkData::RedDownQuark) => {
                    NeutronData::GreenBlueRedUpDownDownQuark
                }

                (QuarkData::BlueDownQuark, QuarkData::AlphaDownQuark) => {
                    NeutronData::GreenBlueAlphaUpDownDownQuark
                }

                (QuarkData::RedDownQuark, QuarkData::BlueDownQuark) => {
                    NeutronData::GreenRedBlueUpDownDownQuark
                }

                (QuarkData::RedDownQuark, QuarkData::AlphaDownQuark) => {
                    NeutronData::GreenRedAlphaUpDownDownQuark
                }

                (QuarkData::AlphaDownQuark, QuarkData::BlueDownQuark) => {
                    NeutronData::GreenAlphaBlueUpDownDownQuark
                }

                (QuarkData::AlphaDownQuark, QuarkData::RedDownQuark) => {
                    NeutronData::GreenAlphaRedUpDownDownQuark
                }

                _ => NeutronData::Unknown,
            },

            QuarkData::BlueUpQuark => match (second_quark, third_quark) {
                (QuarkData::GreenDownQuark, QuarkData::RedDownQuark) => {
                    NeutronData::BlueGreenRedUpDownDownQuark
                }

                (QuarkData::GreenDownQuark, QuarkData::AlphaDownQuark) => {
                    NeutronData::BlueGreenAlphaUpDownDownQuark
                }

                (QuarkData::RedDownQuark, QuarkData::GreenDownQuark) => {
                    NeutronData::BlueRedGreenUpDownDownQuark
                }

                (QuarkData::RedDownQuark, QuarkData::AlphaDownQuark) => {
                    NeutronData::BlueRedAlphaUpDownDownQuark
                }

                (QuarkData::AlphaDownQuark, QuarkData::GreenDownQuark) => {
                    NeutronData::BlueAlphaGreenUpDownDownQuark
                }

                (QuarkData::AlphaDownQuark, QuarkData::RedDownQuark) => {
                    NeutronData::BlueAlphaRedUpDownDownQuark
                }

                _ => NeutronData::Unknown,
            },

            QuarkData::AlphaUpQuark => match (second_quark, third_quark) {
                (QuarkData::BlueDownQuark, QuarkData::RedDownQuark) => {
                    NeutronData::AlphaBlueRedUpDownDownQuark
                }

                (QuarkData::BlueDownQuark, QuarkData::GreenDownQuark) => {
                    NeutronData::AlphaBlueGreenUpDownDownQuark
                }

                (QuarkData::RedDownQuark, QuarkData::BlueDownQuark) => {
                    NeutronData::AlphaRedBlueUpDownDownQuark
                }

                (QuarkData::RedDownQuark, QuarkData::GreenDownQuark) => {
                    NeutronData::AlphaRedGreenUpDownDownQuark
                }

                (QuarkData::GreenDownQuark, QuarkData::RedDownQuark) => {
                    NeutronData::AlphaGreenRedUpDownDownQuark
                }

                (QuarkData::GreenDownQuark, QuarkData::BlueDownQuark) => {
                    NeutronData::AlphaGreenBlueUpDownDownQuark
                }

                _ => NeutronData::Unknown,
            },

            _ => NeutronData::Unknown,
        }
    }
}

#[test]
fn it_defaults_all_neutrons_as_unknown() {
    let neutrons = Neutrons::new(0);

    for neutron in neutrons.neutrons.iter() {
        assert_eq!(neutron, &NeutronData::Unknown);
    }
}

#[test]
fn it_can_match_a_single_neutron_correctly() {
    let neutrons = Neutrons::new(1);

    let first_neutron = neutrons.neutrons[0];

    assert_eq!(first_neutron, NeutronData::RedGreenBlueUpDownDownQuark);
}
