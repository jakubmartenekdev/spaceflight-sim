use bevy::prelude::*;

mod debug_ui;
mod orbit_paths;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((debug_ui::plugin, orbit_paths::plugin));
}
