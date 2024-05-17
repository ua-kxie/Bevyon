use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    input::mouse::MouseWheel,
    prelude::*,
    render::{
        mesh::PrimitiveTopology,
        render_asset::RenderAssetUsages,
    },
};
use bevyon::*;

#[derive(Component)]
struct SchematicCameraMarker;

const MINSCALE: f32 = 0.001;
const MAXSCALE: f32 = 1.0;
use std::f32::consts::PI;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyonPlugin))
        .add_systems(Startup, (setup_camera, setup))
        .add_systems(Update, camera_transform)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // lyon stuff
    let mut geometry: VertexBuffers<Vec3, u16> = VertexBuffers::new();
    let mut geometry_builder = BuffersBuilder::new(&mut geometry, |vertex: FillVertex| Vec3 {
        x: vertex.position().x,
        y: vertex.position().y,
        z: 0.0,
    });
    let options = FillOptions::tolerance(0.1);
    let mut tessellator = FillTessellator::new();

    let mut builder = tessellator.builder(&options, &mut geometry_builder);

    builder.add_rounded_rectangle(
        &Box2D {
            min: point(0.0, 0.0),
            max: point(100.0, 50.0),
        },
        &BorderRadii {
            top_left: 10.0,
            top_right: 5.0,
            bottom_left: 20.0,
            bottom_right: 25.0,
        },
        Winding::Positive,
    );
    let _ = builder.build();

    // bevy stuff
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, geometry.vertices)
    .with_inserted_indices(bevy::render::mesh::Indices::U16(geometry.indices));
    let meshid = meshes.add(mesh);
    let bundle = MaterialMeshBundle {
        mesh: meshid,
        material: materials.add(Color::RED),
        ..Default::default()
    };
    commands.spawn(bundle);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 0., -2.0)
                .with_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
            projection: Projection::Orthographic(OrthographicProjection {
                scale: 0.1,
                ..Default::default()
            }),
            camera: Camera {
                hdr: true, // HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // Using a tonemapper that desaturates to white is recommended (bloom)
            ..default()
        },
        SchematicCameraMarker,
        InheritedVisibility::VISIBLE,
        BloomSettings::NATURAL, // Enable bloom for the camera
    ));
}

fn camera_transform(
    mb: Res<ButtonInput<MouseButton>>,
    mut mm: EventReader<CursorMoved>,
    mut mw: EventReader<MouseWheel>,
    mut camera: Query<(&mut Transform, &mut Projection), With<SchematicCameraMarker>>,
) {
    if let Ok((mut transform, mut pj)) = camera.get_single_mut() {
        if mb.pressed(MouseButton::Middle) {
            if let Projection::Orthographic(opj) = &mut *pj {
                let mut pan = Vec3::ZERO;
                for m in mm.read() {
                    if let Some(d) = m.delta {
                        pan += Vec3::new(d.x, d.y, 0.0);
                    }
                }
                let t = pan * opj.scale;
                transform.translation += t;
            }
        }

        for mwe in mw.read() {
            if let Projection::Orthographic(opj) = &mut *pj {
                opj.scale = (opj.scale * (1. - mwe.y / 5.)).clamp(MINSCALE, MAXSCALE);
            }
        }
    }
}
