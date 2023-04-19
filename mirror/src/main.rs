//! A 3D scene where a rotated cube is "reflected in a mirror", i.e. a second camera's view is
//! projected onto a plane. Code borrowed and revised from bevy examples/3d/3d_scene.rs and
//! examples/3d/render_to_texture.rs

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::*;
use bevy::render::render_resource::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(size);

    let image_handle = images.add(image);

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    let mut cube_transform = Transform::from_xyz(0.0, 0.0, 0.0);
    cube_transform.rotate_x(45.0f32.to_radians());

    // cube
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: cube_transform,
        ..default()
    },));

    // projection
    commands.spawn((Camera3dBundle {
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::GRAY),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 4.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        camera: Camera {
            order: -1,
            target: RenderTarget::Image(image_handle.clone()),
            ..default()
        },
        ..default()
    },));

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        reflectance: 0.02,
        unlit: true,
        ..default()
    });

    let plane_mesh = meshes.add(Mesh::from(shape::Plane::from_size(4.0)));
    let mut plane_transform = Transform::from_xyz(0.0, 0.0, 4.0);
    plane_transform.rotate_x(-90.0f32.to_radians());

    // "mirror"
    commands.spawn(PbrBundle {
        mesh: plane_mesh,
        material: material_handle,
        transform: plane_transform,
        ..default()
    });

    // player view
    commands.spawn(Camera3dBundle {
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        transform: Transform::from_xyz(-8.0, 0.0, -2.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });
}
