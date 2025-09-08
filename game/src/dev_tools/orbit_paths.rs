use std::time::Duration;

use bevy::{
    color::palettes::css::GREEN, input::common_conditions::input_toggle_active, prelude::*,
};

use crate::solar_system::planets::{Planet, add_planets};

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct OrbitPath {
    points: Vec<Vec3>,
}

#[derive(Resource)]
struct RedrawTimer(Timer);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, insert_orbit_path.after(add_planets));
    app.add_systems(
        Update,
        (
            draw_path.run_if(input_toggle_active(false, KeyCode::Tab)),
            tick_redraw_timer,
        ),
    );
    app.insert_resource(RedrawTimer(Timer::new(
        Duration::from_secs_f32(0.1),
        TimerMode::Repeating,
    )));
}

fn insert_orbit_path(mut commands: Commands, query: Query<Entity, With<Planet>>) {
    for entity in &query {
        commands.entity(entity).insert(OrbitPath::default());
    }
}

fn tick_redraw_timer(mut timer: ResMut<RedrawTimer>, time: Res<Time>) {
    timer.0.tick(time.delta());
}

// todo draw ui text for key movement
fn draw_path(
    mut gizmos: Gizmos,
    mut paths: Query<(&mut OrbitPath, &Transform)>,
    input: Res<ButtonInput<KeyCode>>,
    timer: ResMut<RedrawTimer>,
) {
    let should_clear = input.pressed(KeyCode::ControlLeft) && input.just_pressed(KeyCode::KeyC);

    if timer.0.just_finished() {
        for (mut path, transform) in &mut paths {
            let current_pos = transform.translation;
            path.points.push(current_pos);
        }
    }

    for (mut path, _) in &mut paths {
        if should_clear {
            path.points.clear();
        }

        gizmos.linestrip(path.points.clone(), GREEN); // VecDeque
        // gizmos.line(start, end, color);
    }
}

// todo: vektory In Out Prograda Retrograda
// todo: nakreslit trajektoriu draw_trajectory
