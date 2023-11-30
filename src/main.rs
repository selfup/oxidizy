extern crate rand;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use std::env;

const DEFAULT_SIZE: u32 = 30;

use unigen::builder;

type Material = StandardMaterial;

#[allow(unused_imports)]
#[derive(Resource)]
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
        .insert_resource(Msaa::default())
        .add_plugins((
            DefaultPlugins,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_block_atoms,
                update_block_spheres,
                update_sphere_positions,
                camera_movement,
            ),
        )
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

    let mesh_handle = meshes.add(Mesh::from(shape::UVSphere {
        radius: 0.15,
        ..default()
    }));

    for block in blocks {
        let x = block.x as f32;
        let y = block.y as f32;
        let z = block.z as f32;

        let r = block.charge;

        commands
            .spawn(PbrBundle {
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
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-60.0, 50.0, 50.0))
                .looking_at(Vec3::default(), up),
            ..Default::default()
        })
        .insert(CameraMatcher());
}

fn update_block_atoms(mut query: Query<&mut BlockMatcher>) {
    query.par_iter_mut().for_each(|mut block| {
        builder::mutate_blocks_with_new_particles(&mut rand::thread_rng(), &mut block.block);

        builder::calculate_charge(&mut block.block);
    });
}

fn update_block_spheres(
    charged_atom_materials: Res<ChargedAtomMaterials>,
    mut query: Query<(&mut Handle<Material>, &mut BlockMatcher)>,
) {
    query
        .par_iter_mut()
        .for_each(|(mut material_handle, block_matcher)| {
            let r = block_matcher.block.charge;
            *material_handle = charged_atom_materials.get(r).clone();
        });
}

fn update_sphere_positions(mut query: Query<(&mut Transform, &BlockMatcher)>) {
    query
        .par_iter_mut()
        .for_each(|(mut transform, block_matcher)| {
            let block = block_matcher.block;

            let new_translation = Vec3::new(block.x as f32, block.y as f32, block.z as f32);

            transform.translation = new_translation;
        });
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CameraMatcher)>,
    block_query: Query<&BlockMatcher>,
) {
    let results = get_input_dir(keyboard_input, block_query);
    let input_dir = results.0;
    let snap_to_grid = results.1;

    if snap_to_grid {
        for (mut transform, _camera) in query.iter_mut() {
            transform.translation = input_dir;
        }
    } else {
        if input_dir.length() > 0. {
            for (mut transform, _camera) in query.iter_mut() {
                let input_dir = (transform.rotation * input_dir).normalize();

                transform.translation += input_dir * (time.delta_seconds_f64() * 50.0) as f32;
            }
        }
    }
}

fn get_input_dir(keyboard_input: Res<Input<KeyCode>>, query: Query<&BlockMatcher>) -> (Vec3, bool) {
    let mut input_dir = Vec3::default();
    let mut snap_to_universe = false;

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

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        let up = Vec3::new(0.0, 1.0, 0.0);
        input_dir -= up;
    }

    if keyboard_input.pressed(KeyCode::F) {
        let mut known_location = Vec3::default();

        for block in &query {
            if block.block.id == 0 {
                info!(
                    "block_id_zero x: {} - y: {} - z: {}",
                    block.block.x, block.block.y, block.block.z
                );

                known_location.x = block.block.x as f32;
                known_location.y = block.block.y as f32;
                known_location.z = block.block.z as f32;

                snap_to_universe = true;

                break;
            }
        }

        input_dir = known_location;
    }

    (input_dir, snap_to_universe)
}
