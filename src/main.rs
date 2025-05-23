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
    fn new(mut asset_server: ResMut<Assets<StandardMaterial>>) -> Self {
        let lower_bound: i8 = 0;
        let upper_bound: i8 = 3;

        Self {
            materials: {
                (lower_bound..upper_bound)
                    .map(|r| {
                        let color: Color = Color::srgb(r as f32, 0., 1.);

                        asset_server.add(color)
                    })
                    .into_iter()
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
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
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
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
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
    commands.spawn(DirectionalLight {
        illuminance: light_consts::lux::OVERCAST_DAY,
        shadows_enabled: false,
        ..default()
    });

    commands.spawn(Transform {
        translation: Vec3::new(0.0, 2.0, 0.0),
        rotation: Quat::from_rotation_x(-3.14 / 4.),
        ..default()
    });

    let parsed_size: u32 = if let Some(arg) = env::args().nth(1) {
        arg.trim().parse().unwrap()
    } else {
        DEFAULT_SIZE
    };

    let charged_atom_materials = ChargedAtomMaterials::new(asset_server);

    let blocks = builder::generate_universe(parsed_size);

    let mesh_handle = meshes.add(Sphere::new(0.15).mesh().ico(5).unwrap());

    for block in blocks {
        let x = block.x as f32;
        let y = block.y as f32;
        let z = block.z as f32;

        let r = block.charge;

        commands
            .spawn((
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(charged_atom_materials.get(r).clone()),
                Transform::from_xyz(x, y, z),
            ))
            .insert(BlockMatcher { block });
    }

    commands.insert_resource(charged_atom_materials);

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.35,
        affects_lightmapped_meshes: false,
    });

    let up = Vec3::new(0.0, 1.0, 0.0);

    commands
        .spawn((
            Camera3d::default(),
            Transform::from_translation(Vec3::new(200.0, 150.0, 250.0))
                .looking_at(Vec3::default(), up),
        ))
        .insert(CameraMatcher());
}

fn update_block_atoms(mut query: Query<&mut BlockMatcher>) {
    query.par_iter_mut().for_each(|mut block| {
        builder::mutate_blocks_with_new_particles(&mut unigen::rand::rng(), &mut block.block);

        builder::calculate_charge(&mut block.block);
    });
}

fn update_block_spheres(
    charged_atom_materials: Res<ChargedAtomMaterials>,
    mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &mut BlockMatcher)>,
) {
    query
        .par_iter_mut()
        .for_each(|(mut material_handle, block_matcher)| {
            let r = block_matcher.block.charge;

            let _id = material_handle.id();

            *material_handle = MeshMaterial3d(charged_atom_materials.get(r).clone());
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
    keyboard_input: Res<ButtonInput<KeyCode>>,
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

                transform.translation += input_dir * (time.delta_secs_f64() * 50.0) as f32;
            }
        }
    }
}

fn get_input_dir(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<&BlockMatcher>,
) -> (Vec3, bool) {
    let mut input_dir = Vec3::default();
    let mut snap_to_universe = false;

    if keyboard_input.pressed(KeyCode::KeyW) {
        let forward = Vec3::new(0.0, 0.0, 1.0);
        input_dir -= forward;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        let forward = Vec3::new(0.0, 0.0, 1.0);
        input_dir += forward;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        let right = Vec3::new(1.0, 0.0, 0.0);
        input_dir -= right;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
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

    if keyboard_input.pressed(KeyCode::KeyF) {
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
