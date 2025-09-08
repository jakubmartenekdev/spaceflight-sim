use bevy::prelude::*;

pub(crate) mod planets;

// #[derive(Component)]
// struct SolarSystem {
//     planets: Vec<Planet>,
// }

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(planets::plugin);
}
