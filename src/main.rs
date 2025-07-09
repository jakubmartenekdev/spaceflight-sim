use std::ops::DerefMut;

use avian3d::{collision::CollisionDiagnostics, prelude::*};
use bevy::{
    asset::io::memory::Dir,
    ecs::entity::hash_map::Keys,
    input::{common_conditions::input_toggle_active, keyboard::KeyboardInput},
    math::{VectorSpace, cubic_splines::LinearSpline},
    prelude::*,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

#[derive(Component)]
struct Pull(Vec3);

#[derive(Bundle)]
struct CelestialBody<T>
where
    T: Material,
{
    rb: RigidBody,
    collider: Collider,
    gravity_pull: Pull,
    velocity: LinearVelocity,
    mesh: Mesh3d,
    material: MeshMaterial3d<T>,
}

#[derive(Component)]
struct Controllable;

#[derive(Component)]
struct Planet;

fn main() {
    App::new()
        // Enable physics
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, move_system)
        // .add_systems(Update, slow_down)
        .insert_resource(Gravity::default())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        RigidBody::Static,
        Collider::sphere(3.0),
        Mesh3d(meshes.add(Sphere::new(3.0))),
        MeshMaterial3d(materials.add(Color::BLACK)),
        GravityScale(0.0),
        // LinearVelocity(Vec3::X),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Planet,
    ));

    commands.spawn((
        CelestialBody {
            rb: RigidBody::Dynamic,
            collider: Collider::sphere(0.5),
            gravity_pull: Pull(Vec3::new(0.0, -9.81, 0.0)),
            velocity: LinearVelocity::ZERO,
            mesh: Mesh3d(meshes.add(Sphere::new(0.5))),
            material: MeshMaterial3d(materials.add(Color::WHITE)),
        },
        GravityScale(0.0),
        ExternalImpulse::ZERO,
        Transform::from_xyz(0.0, -10.0, 0.0),
        Controllable,
    ));
    // commands.spawn((
    //     RigidBody::Dynamic,
    //     Collider::sphere(1.0),
    //     Mesh3d(meshes.add(Sphere::new(1.0))),
    //     MeshMaterial3d(materials.add(Color::BLACK)),
    //     Transform::from_xyz(0.5, 10.0, 0.0),
    //     // LinearVelocity(Vec3::new(0., 0., -2.)),
    //     LinearDamping(1.0),
    //     LinearVelocity::ZERO,
    //     // Friction::new(0.4).with_dynamic_coefficient(0.6),
    //     // ExternalImpulse::new(Vec3::new(1., 0., 0.)).with_persistence(true),
    //     // ExternalForce::new(Vec3::new(1., 0., 0.)),
    //     // ExternalForce::new(Vec3::new(10.0, 0., 0.)),
    //     Name::new("Black marble"),
    //     // Mass(10.0),
    //     // Friction::new(40.0).with_dynamic_coefficient(0.6),
    //     GravityScale(0.0),
    // ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // commands.spawn((
    //     Camera3d::default(),
    //     Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)).looking_at(Vec3::ZERO, Dir3::Y),
    // ));
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)).looking_at(Vec3::ZERO, Dir3::Y),
        PanOrbitCamera::default(),
    ));
}

fn move_system(
    mut celestial_body: Query<
        (
            &mut LinearVelocity,
            &mut Transform,
            &mut ExternalImpulse,
            &Pull,
        ),
        With<Controllable>,
    >,
    planet: Query<(&Transform), (With<Planet>, Without<Controllable>)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let (mut velocity, mut transform, mut impulse, gravity) = celestial_body.single_mut().unwrap();
    let dest = planet.single().unwrap().translation;
    let mut current_pos = transform.translation;
    let fall = (dest - current_pos).normalize_or_zero() * -(gravity.0.y) * 10. * dt;

    // dbg!(dest - current_pos);
    dbg!(current_pos);
    // dbg!((dest - current_pos).normalize() * gravity.0);
    // current_pos = fall;
    velocity.0 += fall;

    if input.just_pressed(KeyCode::Space) {
        impulse.set_impulse(Vec3::new(0., 0., 20.));
    }
}

fn slow_down(
    mut spheres: Query<(&mut ExternalForce, &LinearVelocity)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for (mut f, mut vel) in &mut spheres {
            // dbg!(f.force());
            dbg!(vel.0);
            f.set_force(Vec3::ZERO);
        }
    }
}
