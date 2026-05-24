use bevy::camera::visibility::NoFrustumCulling;
use bevy::core_pipeline::core_3d::Transparent3d;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::ecs::query::QueryItem;
use bevy::ecs::system::{lifetimeless::*, SystemParamItem};
use bevy::mesh::{MeshVertexBufferLayoutRef, VertexBufferLayout};
use bevy::pbr::{
    MeshPipeline, MeshPipelineKey, RenderMeshInstances, SetMeshBindGroup, SetMeshViewBindGroup,
    SetMeshViewBindingArrayBindGroup,
};
use bevy::prelude::*;
use bevy::render::extract_component::{ExtractComponent, ExtractComponentPlugin};
use bevy::render::mesh::{allocator::MeshAllocator, RenderMesh, RenderMeshBufferInfo};
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_phase::{
    AddRenderCommand, DrawFunctions, PhaseItem, PhaseItemExtraIndex, RenderCommand,
    RenderCommandResult, SetItemPipeline, TrackedRenderPass, ViewSortedRenderPhases,
};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderDevice;
use bevy::render::sync_world::MainEntity;
use bevy::render::view::{ExtractedView, NoIndirectDrawing};
use bevy::render::{Render, RenderApp, RenderStartup, RenderSystems};
use bytemuck::{Pod, Zeroable};
use rayon::prelude::*;
use std::env;
use unigen::builder;

const DEFAULT_SIZE: u32 = 30;
const SHADER_ASSET_PATH: &str = "shaders/instancing.wgsl";

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            CustomMaterialPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_atoms, camera_movement))
        .run();
}

#[derive(Component)]
struct CameraMatcher;

#[derive(Component)]
struct Blocks(Vec<builder::core::Block>);

fn charge_to_color(charge: i8) -> [f32; 4] {
    match charge {
        -1 => [1.0, 0.3, 0.3, 1.0],
        0 => [0.2, 0.4, 1.0, 1.0],
        1 => [1.0, 0.3, 1.0, 1.0],
        2 => [1.0, 1.0, 0.3, 1.0],
        _ => [1.0, 1.0, 1.0, 1.0],
    }
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let parsed_size: u32 = if let Some(arg) = env::args().nth(1) {
        arg.trim().parse().unwrap()
    } else {
        DEFAULT_SIZE
    };

    let blocks = builder::generate_universe(parsed_size);

    let instance_data: Vec<InstanceData> = blocks
        .iter()
        .map(|b| InstanceData {
            position: Vec3::new(b.x as f32, b.y as f32, b.z as f32),
            scale: 1.0,
            color: charge_to_color(b.charge),
        })
        .collect();

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.15).mesh().ico(1).unwrap())),
        InstanceMaterialData(instance_data),
        Blocks(blocks),
        NoFrustumCulling,
    ));

    let up = Vec3::new(0.0, 1.0, 0.0);

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(200.0, 150.0, 250.0)).looking_at(Vec3::default(), up),
        NoIndirectDrawing,
        CameraMatcher,
    ));
}

fn update_atoms(mut query: Query<(&mut Blocks, &mut InstanceMaterialData)>) {
    for (mut blocks, mut data) in &mut query {
        let blocks_slice = blocks.0.as_mut_slice();
        let data_slice = data.0.as_mut_slice();
        blocks_slice
            .par_iter_mut()
            .zip(data_slice.par_iter_mut())
            .for_each(|(block, inst)| {
                builder::mutate_blocks_with_new_particles(&mut unigen::rand::rng(), block);
                builder::calculate_charge(block);
                inst.position = Vec3::new(block.x as f32, block.y as f32, block.z as f32);
                inst.color = charge_to_color(block.charge);
            });
    }
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<CameraMatcher>>,
    blocks_query: Query<&Blocks>,
) {
    let (input_dir, snap_to_grid) = get_input_dir(keyboard_input, blocks_query);

    if snap_to_grid {
        for mut transform in query.iter_mut() {
            transform.translation = input_dir;
        }
    } else if input_dir.length() > 0. {
        for mut transform in query.iter_mut() {
            let input_dir = (transform.rotation * input_dir).normalize();
            transform.translation += input_dir * (time.delta_secs_f64() * 50.0) as f32;
        }
    }
}

fn get_input_dir(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    blocks_query: Query<&Blocks>,
) -> (Vec3, bool) {
    let mut input_dir = Vec3::default();
    let mut snap_to_universe = false;

    if keyboard_input.pressed(KeyCode::KeyW) {
        input_dir -= Vec3::new(0.0, 0.0, 1.0);
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        input_dir += Vec3::new(0.0, 0.0, 1.0);
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        input_dir -= Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        input_dir += Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Space) {
        input_dir += Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        input_dir -= Vec3::new(0.0, 1.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::KeyF) {
        let mut known_location = Vec3::default();
        'outer: for blocks in &blocks_query {
            for block in &blocks.0 {
                if block.id == 0 {
                    info!(
                        "block_id_zero x: {} - y: {} - z: {}",
                        block.x, block.y, block.z
                    );
                    known_location = Vec3::new(block.x as f32, block.y as f32, block.z as f32);
                    snap_to_universe = true;
                    break 'outer;
                }
            }
        }
        input_dir = known_location;
    }

    (input_dir, snap_to_universe)
}

// ---------------------------------------------------------------------------
// Custom GPU instancing pipeline
// Adapted from Bevy 0.18.1 examples/shader_advanced/custom_shader_instancing.rs
// ---------------------------------------------------------------------------

#[derive(Component, Deref)]
struct InstanceMaterialData(Vec<InstanceData>);

impl ExtractComponent for InstanceMaterialData {
    type QueryData = &'static InstanceMaterialData;
    type QueryFilter = ();
    type Out = Self;

    fn extract_component(item: QueryItem<'_, '_, Self::QueryData>) -> Option<Self> {
        Some(InstanceMaterialData(item.0.clone()))
    }
}

struct CustomMaterialPlugin;

impl Plugin for CustomMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<InstanceMaterialData>::default());
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent3d, DrawCustom>()
            .init_resource::<SpecializedMeshPipelines<CustomPipeline>>()
            .add_systems(RenderStartup, init_custom_pipeline)
            .add_systems(
                Render,
                (
                    queue_custom.in_set(RenderSystems::QueueMeshes),
                    prepare_instance_buffers.in_set(RenderSystems::PrepareResources),
                ),
            );
    }
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct InstanceData {
    position: Vec3,
    scale: f32,
    color: [f32; 4],
}

fn queue_custom(
    transparent_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
    custom_pipeline: Res<CustomPipeline>,
    mut pipelines: ResMut<SpecializedMeshPipelines<CustomPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    meshes: Res<RenderAssets<RenderMesh>>,
    render_mesh_instances: Res<RenderMeshInstances>,
    material_meshes: Query<(Entity, &MainEntity), With<InstanceMaterialData>>,
    mut transparent_render_phases: ResMut<ViewSortedRenderPhases<Transparent3d>>,
    views: Query<(&ExtractedView, &Msaa)>,
) {
    let draw_custom = transparent_3d_draw_functions.read().id::<DrawCustom>();

    for (view, msaa) in &views {
        let Some(transparent_phase) =
            transparent_render_phases.get_mut(&view.retained_view_entity)
        else {
            continue;
        };

        let msaa_key = MeshPipelineKey::from_msaa_samples(msaa.samples());
        let view_key = msaa_key | MeshPipelineKey::from_hdr(view.hdr);
        let rangefinder = view.rangefinder3d();

        for (entity, main_entity) in &material_meshes {
            let Some(mesh_instance) =
                render_mesh_instances.render_mesh_queue_data(*main_entity)
            else {
                continue;
            };
            let Some(mesh) = meshes.get(mesh_instance.mesh_asset_id) else {
                continue;
            };
            let key =
                view_key | MeshPipelineKey::from_primitive_topology(mesh.primitive_topology());
            let pipeline = pipelines
                .specialize(&pipeline_cache, &custom_pipeline, key, &mesh.layout)
                .unwrap();
            transparent_phase.add(Transparent3d {
                entity: (entity, *main_entity),
                pipeline,
                draw_function: draw_custom,
                distance: rangefinder.distance(&mesh_instance.center),
                batch_range: 0..1,
                extra_index: PhaseItemExtraIndex::None,
                indexed: true,
            });
        }
    }
}

#[derive(Component)]
struct InstanceBuffer {
    buffer: Buffer,
    length: usize,
}

fn prepare_instance_buffers(
    mut commands: Commands,
    query: Query<(Entity, &InstanceMaterialData)>,
    render_device: Res<RenderDevice>,
) {
    for (entity, instance_data) in &query {
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("instance data buffer"),
            contents: bytemuck::cast_slice(instance_data.as_slice()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        commands.entity(entity).insert(InstanceBuffer {
            buffer,
            length: instance_data.len(),
        });
    }
}

#[derive(Resource)]
struct CustomPipeline {
    shader: Handle<Shader>,
    mesh_pipeline: MeshPipeline,
}

fn init_custom_pipeline(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mesh_pipeline: Res<MeshPipeline>,
) {
    commands.insert_resource(CustomPipeline {
        shader: asset_server.load(SHADER_ASSET_PATH),
        mesh_pipeline: mesh_pipeline.clone(),
    });
}

impl SpecializedMeshPipeline for CustomPipeline {
    type Key = MeshPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayoutRef,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;

        descriptor.vertex.shader = self.shader.clone();
        descriptor.vertex.buffers.push(VertexBufferLayout {
            array_stride: size_of::<InstanceData>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 3,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: VertexFormat::Float32x4.size(),
                    shader_location: 4,
                },
            ],
        });
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();

        // Atoms are opaque even though we're in the Transparent3d phase;
        // force depth write + depth test so a closer atom drawn later still
        // wins the pixel and stacked motion stays visible.
        if let Some(depth_stencil) = descriptor.depth_stencil.as_mut() {
            depth_stencil.depth_write_enabled = true;
            depth_stencil.depth_compare = CompareFunction::GreaterEqual;
        }

        Ok(descriptor)
    }
}

type DrawCustom = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshViewBindingArrayBindGroup<1>,
    SetMeshBindGroup<2>,
    DrawMeshInstanced,
);

struct DrawMeshInstanced;

impl<P: PhaseItem> RenderCommand<P> for DrawMeshInstanced {
    type Param = (
        SRes<RenderAssets<RenderMesh>>,
        SRes<RenderMeshInstances>,
        SRes<MeshAllocator>,
    );
    type ViewQuery = ();
    type ItemQuery = Read<InstanceBuffer>;

    #[inline]
    fn render<'w>(
        item: &P,
        _view: (),
        instance_buffer: Option<&'w InstanceBuffer>,
        (meshes, render_mesh_instances, mesh_allocator): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let mesh_allocator = mesh_allocator.into_inner();

        let Some(mesh_instance) =
            render_mesh_instances.render_mesh_queue_data(item.main_entity())
        else {
            return RenderCommandResult::Skip;
        };
        let Some(gpu_mesh) = meshes.into_inner().get(mesh_instance.mesh_asset_id) else {
            return RenderCommandResult::Skip;
        };
        let Some(instance_buffer) = instance_buffer else {
            return RenderCommandResult::Skip;
        };
        let Some(vertex_buffer_slice) =
            mesh_allocator.mesh_vertex_slice(&mesh_instance.mesh_asset_id)
        else {
            return RenderCommandResult::Skip;
        };

        pass.set_vertex_buffer(0, vertex_buffer_slice.buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer.buffer.slice(..));

        match &gpu_mesh.buffer_info {
            RenderMeshBufferInfo::Indexed {
                index_format,
                count,
            } => {
                let Some(index_buffer_slice) =
                    mesh_allocator.mesh_index_slice(&mesh_instance.mesh_asset_id)
                else {
                    return RenderCommandResult::Skip;
                };
                pass.set_index_buffer(index_buffer_slice.buffer.slice(..), *index_format);
                pass.draw_indexed(
                    index_buffer_slice.range.start..(index_buffer_slice.range.start + count),
                    vertex_buffer_slice.range.start as i32,
                    0..instance_buffer.length as u32,
                );
            }
            RenderMeshBufferInfo::NonIndexed => {
                pass.draw(vertex_buffer_slice.range, 0..instance_buffer.length as u32);
            }
        }
        RenderCommandResult::Success
    }
}
