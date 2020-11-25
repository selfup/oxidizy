use rand::Rng;
use colored::*;
use rayon::prelude::*;

pub mod core;

use self::core::{proton, neutron};

pub struct Blocks {}

impl Blocks {
    pub fn initialize_universe(parsed_size: u32) -> Vec<core::Block> {
        let mut id: u32 = 0;
        
        let vec_size = (parsed_size * parsed_size * parsed_size) as usize;
        
        let mut universe = Vec::with_capacity(vec_size);
        
        println!("Threads: {}\nBuilding..", rayon::current_num_threads());
        
        for x in 0..parsed_size {
            for y in 0..parsed_size {
                for z in 0..parsed_size {                
                    let (electrons, protons, neutrons): (u32, u32, u32) = (0, 0, 0);

                    let generated_protons = proton::Protons::new(protons);
                    let generated_neutrons = neutron::Neutrons::new(neutrons);
    
                    universe.push(core::Block {
                        id,
                        x,
                        y,
                        z,
                        charge: 0,
                        atom: core::Atom {
                           electrons,
                            nucleus: core::Nucleus {
                                baryon: core::Baryon {
                                    protons: generated_protons,
                                    neutrons: generated_neutrons,
                                },
                            },
                        },
                    });
    
                    id += 1;
                }
            }
        }
    
        universe
    }
    
    pub fn particles(universe: &mut Vec<core::Block>, neutron: &mut [u32; 1], proton: &mut [u32; 1], electron: &mut [u32; 1]) {
        neutron[0] = universe.par_iter().map(|i| i.atom.nucleus.baryon.neutrons.count).sum();
        proton[0] = universe.par_iter().map(|i| i.atom.nucleus.baryon.protons.count).sum();
        electron[0] = universe.par_iter().map(|i| i.atom.electrons).sum();
    }
    
    pub fn charge_of_field(proton: &mut [u32; 1], electron: &mut [u32; 1], u: u32) {
        let size: u32 = u * u * u;
        let cast_size: u32 = size as u32;
        
        if proton[0] == cast_size && electron[0] == cast_size {
            println!("Field is Netural");
        } else if (proton[0] > cast_size) && (electron[0] < proton[0]) {
            println!("Field is Cationic");
        } else {
            println!("Field is Anionic");
        }
    }
    
    pub fn atom_charge(universe: &mut Vec<core::Block>) {
        for block in universe {
            calculate_charge(block);
        }
    }
    
    pub fn tick(parsed_size: u32, universe: &mut Vec<core::Block>) -> Vec<core::Block> {
        let mut uni_copy: Vec<core::Block> = universe.clone();
        
        let chunk_size: usize = (parsed_size) as usize;
    
        uni_copy.par_chunks_mut(chunk_size).for_each_init(|| rand::thread_rng(), |rng, blocks| {
            for block in blocks {
                mutate_blocks_with_new_particles(rng, block);
            }
        });
    
        uni_copy
    }
}

pub fn calculate_charge(block: &mut core::Block) {
    if block.atom.nucleus.baryon.protons.count == block.atom.electrons {
        block.charge = 0;
    } else if block.atom.nucleus.baryon.protons.count > block.atom.electrons {
        block.charge = 1;
    } else {
        block.charge = -1;
    }
}

pub fn mutate_blocks_with_new_particles(rng: &mut rand::rngs::ThreadRng, block: &mut core::Block) {
    let (electrons, protons, neutrons, rotation): (u32, u32, u32, u8) = (
        rng.gen_range(0, 118),
        rng.gen_range(0, 118),
        rng.gen_range(0, 118),
        rng.gen_range(1, 6),
    );

    match rotation {
        1 => block.x += 1,
        2 => block.x -= 1,
        3 => block.y += 1,
        4 => block.y -= 1,
        5 => block.z += 1,
        6 => block.z -= 1,
        _ => (),
    }

    block.atom.electrons = electrons;
    block.atom.nucleus.baryon.protons = proton::Protons::new(protons);
    block.atom.nucleus.baryon.neutrons = neutron::Neutrons::new(neutrons);
}

pub fn generate_universe(parsed_size: u32) -> Vec<core::Block> {
    println!("{}", "--------------------------------".red().bold());

    let mut neturon: [u32; 1] = [0];
    let mut proton: [u32; 1] = [0];
    let mut electron: [u32; 1] = [0];

    let mut generated_universe = Blocks::initialize_universe(parsed_size);

    generated_universe = Blocks::tick(parsed_size, &mut generated_universe);
    Blocks::particles(&mut generated_universe, &mut neturon, &mut proton, &mut electron);

    println!("{}", "--------------------------------".purple().bold());
    println!("Universe built!");
    println!("{}", "--------------------------------".yellow().bold());

    Blocks::charge_of_field(&mut proton, &mut electron, parsed_size as u32);
    Blocks::atom_charge(&mut generated_universe);

    let default_baryons = 236;
    let quarks_per_baryon = 3;
    let generated_universe_length = generated_universe.len();
    let total_atoms = generated_universe_length;
    let total_baryons = generated_universe_length * default_baryons;
    let total_quarks = generated_universe_length * default_baryons* quarks_per_baryon;

    println!("{}", "--------------------------------".magenta().bold());
    println!("Atoms: {}", total_atoms);
    println!("Baryons: {}", total_baryons);
    println!("Quarks: {}", total_quarks);
    println!("{}", "--------------------------------".red().bold());

    generated_universe
}

#[test]
fn it_can_begin() {    
    let universe = Blocks::initialize_universe(5);

    assert_eq!(universe.len(), 125);
    
    assert_eq!(universe[0].x,0);
    assert_eq!(universe[0].y,0);
    assert_eq!(universe[0].z,0);

    assert_eq!(universe[20].x,0);
    assert_eq!(universe[20].y, 4);
    assert_eq!(universe[20].z,0);
}

#[test]
fn it_can_infer_the_charge_of_an_atom() {
    let mut neturon: [u32; 1] = [0];
    let mut proton: [u32; 1] = [0];
    let mut electron: [u32; 1] = [0];
    
    let mut generated_universe: Vec<core::Block> = Blocks::initialize_universe(5);
    Blocks::tick(5, &mut generated_universe);
    Blocks::particles(&mut generated_universe, &mut neturon, &mut proton, &mut electron);
    Blocks::atom_charge(&mut generated_universe);
    
    assert_eq!(generated_universe.len(), 125);
}

#[test]
fn it_can_sense_the_field() {
    let mut neturon: [u32; 1] = [0];
    let mut proton: [u32; 1] = [0];
    let mut electron: [u32; 1] = [0];

    let mut universe = Blocks::initialize_universe(2);
    Blocks::particles(&mut universe, &mut neturon, &mut proton, &mut electron);

    assert_eq!(universe.len(), 8);
    assert_eq!(neturon.len(), 1);
    assert_eq!(proton.len(), 1);
    assert_eq!(electron.len(), 1);
}
