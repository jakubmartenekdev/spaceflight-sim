use bevy::{
    color::palettes::css::GREEN, input::common_conditions::input_toggle_active, prelude::*,
};

use crate::{OrbitPath, Scheduler};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        draw_path.run_if(input_toggle_active(false, KeyCode::Tab)),
    );
}

// todo draw ui text for key movement
fn draw_path(
    mut gizmos: Gizmos,
    mut paths: Query<(&mut OrbitPath, &Transform)>,
    mut timer: ResMut<Scheduler>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    timer.0.tick(time.delta());
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
