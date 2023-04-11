#![allow(dead_code)]

use bevy::prelude::*;
use moonshine_save::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    CreateMeshesAndMaterials,
    SpawnGeometry,
    Run,
}

#[derive(Resource, Default)]
pub struct GeometryHandles {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

const SAVE_FILE_PATH: &str = "assets/geometry.ron";

const PURPLE: &str = "301934";
const GREEN: &str = "39FF14";

fn main() {
    App::new()
        .add_state::<AppState>()
        .insert_resource(GeometryHandles::default())
        .insert_resource(ClearColor(
            Color::hex(PURPLE)
                .unwrap_or_else(|_| panic!("couldn't make clear color from {PURPLE}")),
        ))
        .add_plugins(DefaultPlugins)
        .add_plugin(SavePlugin)
        .add_plugin(LoadPlugin)
        .add_systems((
            //
            // these are the systems to use if we're loading from the save file, assets/geometry.ron
            //
            load_from_file(SAVE_FILE_PATH).in_schedule(OnEnter(AppState::CreateMeshesAndMaterials)),
            create_meshes_and_materials.run_if(in_state(AppState::CreateMeshesAndMaterials)),
            spawn_geometry.in_schedule(OnEnter(AppState::SpawnGeometry)),
            setup_camera.in_schedule(OnExit(AppState::SpawnGeometry)),

            //
            // these are the systems to use if we're creating geometry and saving it
            //
            // create_meshes_and_materials.run_if(in_state(AppState::CreateMeshesAndMaterials)),
            // create_geometry.in_schedule(OnEnter(AppState::SpawnGeometry)),
            // save_into_file(SAVE_FILE_PATH).in_schedule(OnExit(AppState::SpawnGeometry)),
            // setup_camera.in_schedule(OnExit(AppState::SpawnGeometry)),
        ))
        .run();
}

fn create_meshes_and_materials(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut handles: ResMut<GeometryHandles>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let geometry_color =
        Color::hex(GREEN).unwrap_or_else(|_| panic!("couldn't make color from {GREEN}"));

    handles.mesh = meshes.add(shape::Cube { size: 1.0 }.into());
    handles.material = materials.add(StandardMaterial {
        emissive: geometry_color,
        ..default()
    });

    app_state.set(AppState::SpawnGeometry)
}

fn spawn_geometry(
    mut commands: Commands,
    handles: Res<GeometryHandles>,
    mut app_state: ResMut<NextState<AppState>>,
    query: Query<&Transform, With<Save>>,
) {
    for transform in query.iter() {
        commands.spawn((
            PbrBundle {
                mesh: handles.mesh.clone(),
                material: handles.material.clone(),
                transform: *transform,
                ..default()
            },
            Save,
        ));
    }
    app_state.set(AppState::Run)
}

fn create_geometry(
    mut commands: Commands,
    handles: Res<GeometryHandles>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: handles.mesh.clone(),
            material: handles.material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Save,
    ));
    app_state.set(AppState::Run)
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-5.0, 2.5, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
