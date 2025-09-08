use avian3d::prelude::{AngularVelocity, Collider, LinearVelocity, Mass, RigidBody};
use bevy::prelude::*;

const SUN_SIZE: f32 = 20.0;
const PLANET_SIZE: f32 = 10.0;

#[derive(Component, Default)]
pub(crate) struct Planet {
    rigid_body: RigidBody,
    collider: Collider,
    velocity: LinearVelocity,
    angular_velocity: AngularVelocity,
    mass: Mass,
    transform: Transform,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, add_planets);
}

pub(crate) fn add_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for i in 0..7 {
        let entity = commands.spawn((
            Planet {
                // radius: 10.0,
                ..Default::default()
            },
            RigidBody::Dynamic,
            Collider::sphere(PLANET_SIZE),
            Mesh3d(meshes.add(Sphere::new(PLANET_SIZE))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_xyz(1.0 * i as f32, 0.0, 0.0),
        ));
        // #[cfg(feature = "dev")]
        // entity.insert(OrbitPath { points: vec![] });
    }

    commands.spawn((
        RigidBody::Static,
        Collider::sphere(SUN_SIZE),
        Mesh3d(meshes.add(Sphere::new(SUN_SIZE))),
        MeshMaterial3d(materials.add(Color::BLACK)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

// fn spawn_planets
