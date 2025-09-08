// #[cfg(feature = "dev")]
// mod dev_tools;
mod solar_system;

use avian3d::{parry::na::ComplexField, prelude::*};
use bevy::{ecs::name::Name, prelude::*};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

// 1000KM on 1M
const WORLD_SCALE: f32 = 1.0 / 1_000_000.0;

const EARTH_RADIUS: f32 = 6_378_000.0 * WORLD_SCALE;
const MOON_RADIUS: f32 = 1_736_000.0 * WORLD_SCALE;
const EARTH_MASS: f64 = 5.9722e24;
const MOON_MASS: f64 = 7.342e22;

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

//todo: SolarSystem with list of planets

#[derive(Reflect, Component)]
#[reflect(Component)]
struct OrbitPath {
    points: Vec<Vec3>,
}

fn main() {
    // std::env::set_var("WGPU_BACKEND", "vulkan");
    // #[cfg(feature = "dev")]
    let app = App::new()
        // External plugins
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        // .add_plugins(
        //     WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        // )
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        // .add_systems(Update, move_system)
        // .add_systems(Update, slow_down)
        .insert_resource(Gravity::ZERO)
        // .register_type::<OrbitPath>()
        // Module level plugins
        .add_plugins(solar_system::plugin)
        // .add_plugins(dev_tools::plugin) // #[cfg(feature = "dev")]
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        RigidBody::Static,
        Collider::sphere(EARTH_RADIUS),
        Mesh3d(meshes.add(Sphere::new(EARTH_RADIUS))),
        MeshMaterial3d(materials.add(Color::BLACK)),
        GravityScale(0.0),
        // LinearVelocity(Vec3::X),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Planet,
    ));

    commands.spawn((
        CelestialBody {
            rb: RigidBody::Dynamic,
            collider: Collider::sphere(MOON_RADIUS),
            gravity_pull: Pull(Vec3::new(0.0, -9.81, 0.0)),
            // velocity: LinearVelocity::ZERO,
            velocity: LinearVelocity(Vec3::new(15.0, 0.0, 0.0)),
            mesh: Mesh3d(meshes.add(Sphere::new(MOON_RADIUS))),
            material: MeshMaterial3d(materials.add(Color::WHITE)),
        },
        ExternalForce::ZERO,
        GravityScale(0.0),
        ExternalImpulse::ZERO,
        Transform::from_xyz(0.0, 384_400_000.0 * WORLD_SCALE / 2.0, 0.0),
        OrbitPath { points: vec![] },
        Controllable,
        Name::new("Celestial Object"),
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
    planet: Query<&Transform, (With<Planet>, Without<Controllable>)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    paths: Query<&mut OrbitPath>,
    gizmos: Gizmos,
) {
    let dt = time.delta_secs();
    let G = 6.674e-11;
    for (mut velocity, transform, mut force, mut impulse, gravity) in &mut celestial_body {
        let dest = planet.single().unwrap().translation;
        let current_pos = transform.translation;

        let distance = (current_pos - dest).length() / WORLD_SCALE;
        let dir = (dest - current_pos).normalize_or_zero();
        // Fg = G(m1*m2)/l^2
        let fg = G * (EARTH_MASS * MOON_MASS) / distance.powi(2) as f64;
        // let fall_dir = ((current_pos - dest) * WORLD_SCALE).normalize_or_zero() * -fg * dt; // * gravity.0.y ;
        // if distance - 3.0 - 0.5 < 0.1 {
        //     force.set_force(Vec3::ZERO);
        // } else {
        // let acceleration = dir * -fg as f32 / MOON_MASS as f32; // a = F/m
        // dbg!(fg as f32 / MOON_MASS as f32 * WORLD_SCALE * dt);
        let acceleration = dir * fg as f32 / MOON_MASS as f32 * WORLD_SCALE * dt; // a = F/m

        // dbg!(acceleration / WORLD_SCALE * 1000.0);
        velocity.0 += acceleration * dt;
        // velocity.0 += fall_dir;
        // velocity.0 -= Vec3::new(0.0, 0.1, 0.0);
        // force.apply_force(fall_dir);
        // }
        // dbg!(velocity.0);
        // dbg!(distance - 3.0 - 0.5);
        // dbg!(velocity.0.length());

        // dbg!(dir);
        // dbg!(fg);

        // gizmos.arrow(current_pos, dest, YELLOW);

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
}

fn slow_down(
    mut spheres: Query<(&mut ExternalForce, &LinearVelocity)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for (mut f, vel) in &mut spheres {
            // dbg!(f.force());
            dbg!(vel.0);
            f.set_force(Vec3::ZERO);
        }
    }
}
