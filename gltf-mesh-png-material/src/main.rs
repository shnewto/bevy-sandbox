//! A 3D scene displaying a rotated cube built from a gltf named mesh and png material

use bevy::asset::LoadState;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::gltf::{Gltf, GltfMesh};
use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Load,
    Run,
}

#[derive(Resource)]
pub struct CubeHandle {
    pub gltf: Handle<Gltf>,
    pub texture: Handle<Image>,
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_systems((
            load.in_schedule(OnEnter(AppState::Load)),
            check_loaded.run_if(in_state(AppState::Load)),
            spawn.in_schedule(OnExit(AppState::Load)),
            lighting.in_schedule(OnEnter(AppState::Run)),
            camera.in_schedule(OnEnter(AppState::Run)),
        ))
        .run();
}

pub fn spawn(
    gltf_assets: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    cube_handle: Res<CubeHandle>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Some(scenes_gltf) = gltf_assets.get(&cube_handle.gltf) {
        let cube_gltf_mesh_handle = &scenes_gltf.named_meshes["Cube"];

        let cube_mesh_handle: Handle<Mesh> = gltf_meshes
            .get(cube_gltf_mesh_handle)
            .and_then(|gltf_mesh| gltf_mesh.primitives.get(0))
            .map(|cube_primitive| cube_primitive.mesh.clone())
            .unwrap();

        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(cube_handle.texture.clone()),
            reflectance: 0.02,
            unlit: true,
            ..default()
        });

        commands.spawn((PbrBundle {
            mesh: cube_mesh_handle,
            material: material_handle,
            ..default()
        },));
    }
}

pub fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(CubeHandle {
        gltf: asset_server.load("gltf/cube.gltf"),
        texture: asset_server.load("texture/cube.png"),
    });
}

pub fn check_loaded(
    asset_server: Res<AssetServer>,
    cube_handle: Res<CubeHandle>,
    mut state: ResMut<NextState<AppState>>,
) {
    if LoadState::Loaded != asset_server.get_load_state(&cube_handle.gltf) {
        return;
    }

    if LoadState::Loaded != asset_server.get_load_state(&cube_handle.texture) {
        return;
    }

    state.set(AppState::Run)
}

fn lighting(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 10.0, 10.0),
        ..default()
    });
}

fn camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        transform: Transform::from_xyz(-8.0, 4.0, 1.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });
}
