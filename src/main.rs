extern crate rand;

use std::env;

use bevy::prelude::*;
use bevy::render::pass::ClearColor;

use rayon::prelude::*;

mod builder;

#[allow(unused_imports)]
use rand::Rng;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(update_block_atoms.system())
        .add_system(update_block_spheres.system())
        .add_system(camera_movement.system())
        .add_system(random_movement.system())
        .add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .run();
}

struct CameraMatcher();

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut size = String::new();
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        size = args[1].clone();
    }

    let parsed_size = size.trim().parse::<u32>().unwrap();

    let blocks = builder::generate_universe(parsed_size);

    for block in blocks {
        let y = block.y as f32;
        let x = block.x as f32;
        let z = block.z as f32;

        let mut r = block.charge as f32;

        if r < 0.0 {
            r = 2.0;
        }

        commands
            .spawn(PbrComponents {
                mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.15, subdivisions: 1 })),
                material: materials.add(Color::rgb(r, 0.7, 0.6).into()),
                transform: Transform::from_translation(Vec3::new(x, y, z)),
                ..Default::default()
            })
            .with(block);
    }

    commands
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(-60.0, 50.0, 50.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .with(CameraMatcher());
}

fn update_block_spheres(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Handle<StandardMaterial>, &builder::core::Block)>,
) {
    for (material_handle, block) in query.iter_mut() {
        update_albedo(&mut materials, material_handle, block);
    }
}

fn update_albedo(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    material_handle: &Handle<StandardMaterial>,
    block: &builder::core::Block,
) {
    let mut material = materials.get_mut(material_handle).unwrap();
            
    let mut r = block.charge as f32;

    if r < 0.0 {
        r = 2.0;
    }

    material.albedo = Color::rgb(r, 0.0, 1.0).into();
}

fn update_block_atoms(
    mut query: Query<&mut builder::core::Block>,
) {
    let mut query_vec = vec![];

    for block in query.iter_mut() {
        query_vec.push(block);
    }
    
    query_vec.par_chunks_mut(8).for_each_init(|| rand::thread_rng(), |rng, blocks| {
        for block in blocks {
            builder::mutate_blocks_with_new_particles(rng, block);
    
            builder::calculate_charge(block);
        }
    });
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CameraMatcher)>,
) {
    let input_dir = get_input_dir(keyboard_input);

    if input_dir.length() > 0. {
        for (mut transform,  _camera) in query.iter_mut() {
            let input_dir = (transform.rotation * input_dir).normalize();
 
            transform.translation += input_dir * time.delta_seconds * 10.0;
        }
    }
}

fn random_movement(
    mut query: Query<(&mut Transform, &mut builder::core::Block)>,
) {
    for (mut transform,  block) in query.iter_mut() {
        let new_translation = Vec3::new(block.x as f32, block.y as f32, block.z as f32);

        transform.translation = new_translation;
    }
}

fn get_input_dir(keyboard_input: Res<Input<KeyCode>>) -> Vec3 {
    let mut input_dir = Vec3::default();

    if keyboard_input.pressed(KeyCode::W) {
        let forward = Vec3::unit_z();
        input_dir -= forward;
    }

    if keyboard_input.pressed(KeyCode::S) {
        let forward = Vec3::unit_z();
        input_dir += forward;
    }

    if keyboard_input.pressed(KeyCode::A) {
        let right = Vec3::unit_x();
        input_dir -= right;
    }

    if keyboard_input.pressed(KeyCode::D) {
        let right = Vec3::unit_x();
        input_dir += right;
    }

    if keyboard_input.pressed(KeyCode::Space) {
        let up = Vec3::unit_y();
        input_dir += up;
    }
    
    if keyboard_input.pressed(KeyCode::LShift) {
        let up = Vec3::unit_y();
        input_dir -= up;
    }

    input_dir
}
