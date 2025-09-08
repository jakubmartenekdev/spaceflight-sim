use bevy::{input::common_conditions::input_toggle_active, prelude::*, window::PrimaryWindow};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::{
    DefaultInspectorConfigPlugin,
    bevy_egui::{self, EguiContext, EguiContextPass},
    egui,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EguiPlugin {
        enable_multipass_for_primary_context: true,
    });
    app.add_plugins(DefaultInspectorConfigPlugin);
    // app.add_plugins(ResourceInspectorPlugin::<Configuration>::default());
    app.add_systems(
        EguiContextPass,
        ui_example_system.run_if(input_toggle_active(true, KeyCode::Escape)),
    );
}

fn ui_example_system(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
    else {
        return;
    };
    let egui_context = egui_context.clone();

    egui::SidePanel::left("entity_panel")
        .show(egui_context.clone().get_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Entities");
                bevy_inspector_egui::bevy_inspector::ui_for_entities(world, ui);
            });
        })
        .response
        .rect
        .width();

    egui::SidePanel::right("asset_panel")
        .resizable(true)
        .show(egui_context.clone().get_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
            });
        })
        .response
        .rect
        .height();
}
