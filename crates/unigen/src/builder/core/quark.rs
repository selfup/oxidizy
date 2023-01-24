#[derive(Debug, Copy, Clone)]
pub struct Quarks {
    pub up: Quark,
    pub down: Quark,
    pub charm: Quark,
    pub strange: Quark,
    pub top: Quark,
    pub bottom: Quark,
}

#[derive(Debug, Copy, Clone)]
pub struct Quark {
    pub color: StrongCharge,
    pub electric_charge: ElectricCharge,
}

impl Quark {
    pub const fn new(color_charge: u8, electric_charge: u8) -> Quark {
        Quark {
            color: StrongCharge::new(color_charge),
            electric_charge: ElectricCharge::new(electric_charge),
        }
    }

    pub const fn data(quark: Quark) -> QuarkData {
        match (quark.color, quark.electric_charge) {
            (StrongCharge::Red, ElectricCharge::NegativeOneThird) => QuarkData::RedDownQuark,
            (StrongCharge::Red, ElectricCharge::PositiveTwoThirds) => QuarkData::RedUpQuark,

            (StrongCharge::Blue, ElectricCharge::NegativeOneThird) => QuarkData::BlueDownQuark,
            (StrongCharge::Blue, ElectricCharge::PositiveTwoThirds) => QuarkData::BlueUpQuark,

            (StrongCharge::Green, ElectricCharge::NegativeOneThird) => QuarkData::GreenDownQuark,
            (StrongCharge::Green, ElectricCharge::PositiveTwoThirds) => QuarkData::GreenUpQuark,

            (StrongCharge::Alpha, ElectricCharge::NegativeOneThird) => QuarkData::AlphaDownQuark,
            (StrongCharge::Alpha, ElectricCharge::PositiveTwoThirds) => QuarkData::AlphaUpQuark,

            _ => QuarkData::Unknown,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum QuarkData {
    Unknown,
    RedUpQuark,
    RedDownQuark,
    BlueUpQuark,
    BlueDownQuark,
    GreenUpQuark,
    GreenDownQuark,
    AlphaUpQuark,
    AlphaDownQuark,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StrongCharge {
    Unknown,
    Red,
    Green,
    Blue,
    Alpha,
}

impl StrongCharge {
    pub const fn new(color: u8) -> StrongCharge {
        match color {
            0 => StrongCharge::Red,
            1 => StrongCharge::Green,
            2 => StrongCharge::Blue,
            3 => StrongCharge::Alpha,
            _ => StrongCharge::Unknown,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ElectricCharge {
    Unknown,
    PositiveTwoThirds,
    NegativeOneThird,
}

impl ElectricCharge {
    pub const fn new(charge: u8) -> ElectricCharge {
        match charge {
            0 => ElectricCharge::NegativeOneThird,
            1 => ElectricCharge::PositiveTwoThirds,
            _ => ElectricCharge::Unknown,
        }
    }
}

#[test]
fn it_infers_correct_strong_charges() {
    let quark = Quark::new(0, 0);
    assert_eq!(quark.color, StrongCharge::Red);

    let quark = Quark::new(1, 0);
    assert_eq!(quark.color, StrongCharge::Green);

    let quark = Quark::new(2, 0);
    assert_eq!(quark.color, StrongCharge::Blue);

    let quark = Quark::new(3, 0);
    assert_eq!(quark.color, StrongCharge::Alpha);
}

#[test]
fn it_infers_correct_electric_charges() {
    let quark = Quark::new(0, 0);
    assert_eq!(quark.electric_charge, ElectricCharge::NegativeOneThird);

    let quark = Quark::new(0, 1);
    assert_eq!(quark.electric_charge, ElectricCharge::PositiveTwoThirds);
}

#[test]
fn it_infers_the_correct_combination_of_charges() {
    let quark = Quark::new(42, 42);
    let quark_data = Quark::data(quark);
    assert_eq!(quark_data, QuarkData::Unknown);

    let quark = Quark::new(23, 23);
    let quark_data = Quark::data(quark);
    assert_eq!(quark_data, QuarkData::Unknown);

    let quark = Quark::new(0, 0);
    let quark_data = Quark::data(quark);
    assert_eq!(quark_data, QuarkData::RedDownQuark);

    let quark = Quark::new(0, 1);
    let quark_data = Quark::data(quark);
    assert_eq!(quark_data, QuarkData::RedUpQuark);

    let quark = Quark::new(1, 0);
    let quark_data = Quark::data(quark);
    assert_eq!(quark_data, QuarkData::GreenDownQuark);

    let quark = Quark::new(1, 1);
    let quark_data = Quark::data(quark);
    assert_eq!(quark_data, QuarkData::GreenUpQuark);

    let quark = Quark::new(2, 0);
    let quark_data = Quark::data(quark);
    assert_eq!(quark_data, QuarkData::BlueDownQuark);

    let quark = Quark::new(2, 1);
    let quark_data = Quark::data(quark);
    assert_eq!(quark_data, QuarkData::BlueUpQuark);

    let quark = Quark::new(3, 0);
    let quark_data = Quark::data(quark);
    assert_eq!(quark_data, QuarkData::AlphaDownQuark);

    let quark = Quark::new(3, 1);
    let quark_data = Quark::data(quark);
    assert_eq!(quark_data, QuarkData::AlphaUpQuark);
}
