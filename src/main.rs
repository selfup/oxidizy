extern crate rand;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::tasks::prelude::*;
use std::env;

const CHUNK_SIZE: usize = 128;
const DEFAULT_SIZE: u32 = 30;

use unigen::builder;

type Material = StandardMaterial;

#[allow(unused_imports)]

struct ChargedAtomMaterials {
    materials: Vec<Handle<Material>>,
}

impl ChargedAtomMaterials {
    fn new(mut asset_server: ResMut<Assets<Material>>) -> Self {
        let lower_bound: i8 = 0;
        let upper_bound: i8 = 3;

        Self {
            materials: {
                (lower_bound..upper_bound)
                    .map(|r| {
                        let color: Color = Color::rgb(r as f32, 0., 1.);

                        asset_server.add(color.into())
                    })
                    .collect()
            },
        }
    }

    fn get(&self, r: i8) -> &Handle<Material> {
        let lower_bound: i8 = -1;
        let lower_bound_index_map: usize = 2;

        if r == lower_bound {
            &self.materials[lower_bound_index_map]
        } else {
            &self.materials[r as usize]
        }
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(update_block_atoms)
        .add_system(update_block_spheres.after(update_block_atoms))
        .add_system(update_sphere_positions.after(update_block_spheres))
        .add_system(camera_movement)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .run();
}

#[derive(Component)]
struct CameraMatcher();

#[derive(Component)]
struct BlockMatcher {
    block: builder::core::Block,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: ResMut<Assets<Material>>,
) {
    let parsed_size: u32 = if let Some(arg) = env::args().nth(1) {
        arg.trim().parse().unwrap()
    } else {
        DEFAULT_SIZE
    };

    let charged_atom_materials = ChargedAtomMaterials::new(asset_server);

    let blocks = builder::generate_universe(parsed_size);

    let mesh_handle = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.15,
        subdivisions: 1,
    }));

    for block in blocks {
        let x = block.x as f32;
        let y = block.y as f32;
        let z = block.z as f32;

        let r = block.charge;

        commands
            .spawn_bundle(PbrBundle {
                mesh: mesh_handle.clone(),
                material: charged_atom_materials.get(r).clone(),
                transform: Transform::from_xyz(x, y, z),
                ..Default::default()
            })
            .insert(BlockMatcher { block });
    }

    commands.insert_resource(charged_atom_materials);

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.35,
    });

    let up = Vec3::new(0.0, 1.0, 0.0);

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(-60.0, 50.0, 50.0))
                .looking_at(Vec3::default(), up),
            ..Default::default()
        })
        .insert(CameraMatcher());
}

fn update_block_atoms(pool: Res<ComputeTaskPool>, mut query: Query<&mut BlockMatcher>) {
    query.par_for_each_mut(&pool, CHUNK_SIZE, |mut block| {
        for _n in 1..20 {
            builder::mutate_blocks_with_new_particles(&mut rand::thread_rng(), &mut block.block);

            builder::calculate_charge(&mut block.block);
        }
    });
}

fn update_block_spheres(
    pool: Res<ComputeTaskPool>,
    charged_atom_materials: Res<ChargedAtomMaterials>,
    mut query: Query<(&mut Handle<Material>, &mut BlockMatcher)>,
) {
    query.par_for_each_mut(&pool, CHUNK_SIZE, |(mut material_handle, block_matcher)| {
        let r = block_matcher.block.charge;
        *material_handle = charged_atom_materials.get(r).clone();
    });
}

fn update_sphere_positions(
    pool: Res<ComputeTaskPool>,
    mut query: Query<(&mut Transform, &BlockMatcher)>,
) {
    query.par_for_each_mut(&pool, CHUNK_SIZE, |(mut transform, block_matcher)| {
        let block = block_matcher.block;

        let new_translation = Vec3::new(block.x as f32, block.y as f32, block.z as f32);

        transform.translation = new_translation;
    });
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CameraMatcher)>,
) {
    let input_dir = get_input_dir(keyboard_input);

    if input_dir.length() > 0. {
        for (mut transform, _camera) in query.iter_mut() {
            let input_dir = (transform.rotation * input_dir).normalize();

            transform.translation += input_dir * (time.delta_seconds_f64() * 50.0) as f32;
        }
    }
}

fn get_input_dir(keyboard_input: Res<Input<KeyCode>>) -> Vec3 {
    let mut input_dir = Vec3::default();

    if keyboard_input.pressed(KeyCode::W) {
        let forward = Vec3::new(0.0, 0.0, 1.0);
        input_dir -= forward;
    }

    if keyboard_input.pressed(KeyCode::S) {
        let forward = Vec3::new(0.0, 0.0, 1.0);
        input_dir += forward;
    }

    if keyboard_input.pressed(KeyCode::A) {
        let right = Vec3::new(1.0, 0.0, 0.0);
        input_dir -= right;
    }

    if keyboard_input.pressed(KeyCode::D) {
        let right = Vec3::new(1.0, 0.0, 0.0);
        input_dir += right;
    }

    if keyboard_input.pressed(KeyCode::Space) {
        let up = Vec3::new(0.0, 1.0, 0.0);
        input_dir += up;
    }

    if keyboard_input.pressed(KeyCode::LShift) {
        let up = Vec3::new(0.0, 1.0, 0.0);
        input_dir -= up;
    }

    input_dir
}
