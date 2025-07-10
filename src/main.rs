use std::time::Duration;

use avian3d::{
    parry::na::{ComplexField, distance},
    prelude::*,
};
use bevy::{
    color::palettes::css::{GREEN, YELLOW},
    input::{common_conditions::input_toggle_active, keyboard::KeyboardInput},
    prelude::*,
    reflect::List,
    time::Timer,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

#[derive(Resource)]
struct Scheduler(Timer);

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

#[derive(Component)]
struct OrbitPath {
    points: Vec<Vec3>,
}

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
        .add_systems(
            Update,
            draw_path.run_if(input_toggle_active(false, KeyCode::Tab)),
        )
        // .add_systems(Update, slow_down)
        .insert_resource(Gravity::default())
        .insert_resource(Scheduler(Timer::new(
            Duration::from_secs_f32(0.1),
            TimerMode::Repeating,
        )))
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
        ExternalForce::ZERO,
        GravityScale(0.0),
        ExternalImpulse::ZERO,
        Transform::from_xyz(0.0, 10.0, 0.0),
        OrbitPath { points: vec![] },
        Controllable,
        // Mass(1.0),
    ));

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
            &mut ExternalForce,
            &mut ExternalImpulse,
            &Pull,
        ),
        With<Controllable>,
    >,
    planet: Query<(&Transform), (With<Planet>, Without<Controllable>)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut paths: Query<&mut OrbitPath>,
    mut timer: ResMut<Scheduler>,
    mut gizmos: Gizmos,
) {
    let dt = time.delta_secs();
    // let G = 6.674e-11;
    let (mut velocity, mut transform, mut force, mut impulse, gravity) =
        celestial_body.single_mut().unwrap();
    let dest = planet.single().unwrap().translation;
    let mut current_pos = transform.translation;

    let distance = (current_pos - dest).length();
    // Fg = G(m1*m2)/l^2
    let fg = 1.0 * (20.0 * 30.0) / distance.powi(2);
    let fall_dir = (current_pos - dest).normalize_or_zero() * -fg * dt; // * gravity.0.y ;
    if distance - 3.0 - 0.5 < 0.1 {
        force.set_force(Vec3::ZERO);
    } else {
        let acceleration = fall_dir / 20.0; // a = F/m
        // dbg!(acceleration);
        velocity.0 += acceleration;
        // velocity.0 += fall_dir;
        // force.apply_force(fall_dir);
    }
    // dbg!(distance - 3.0 - 0.5);
    // dbg!(velocity.0.length() < 0.1);

    // dbg!(fall_dir);
    // dbg!(fg);

    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        paths.single_mut().unwrap().points.push(current_pos);
    }

    gizmos.arrow(current_pos, dest, YELLOW);

    if input.just_pressed(KeyCode::Backspace) {
        // velocity.0 = Vec3::ZERO;
        force.set_force(Vec3::ZERO);
        // force.apply_force(Vec3::ZERO);
        // println!("JAKOOOOO");
    }

    if !input.pressed(KeyCode::ShiftLeft) && input.just_pressed(KeyCode::Space) {
        impulse.apply_impulse(Vec3::new(0.1, 0., 0.));
        // let tangent = (dest - current_pos).normalize().cross(Vec3::Z); // Perpendicular vector
        // impulse.set_impulse(tangent * 4.0);
    }

    if input.pressed(KeyCode::ShiftLeft) && input.just_pressed(KeyCode::Space) {
        impulse.set_impulse(Vec3::new(-0.1, 0., 0.));
    }
}

fn draw_path(mut gizmos: Gizmos, paths: Query<&OrbitPath>) {
    for path in &paths {
        gizmos.linestrip(path.points.clone(), GREEN);
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
