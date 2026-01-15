use bevy::prelude::*;

pub mod camera;
pub mod components;
pub mod hierarchy;
pub mod inspector;
pub mod resources;
pub mod styles;
pub mod ui;
pub mod menu;

use resources::{EditorConfig, EditorState};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EditorState>()
           .init_resource::<EditorConfig>()
           .add_systems(Startup, ui::setup_editor_ui)
           .add_systems(Update, (
                ui::toggle_editor,
                hierarchy::update_hierarchy_list,
                hierarchy::update_hierarchy,
                menu::handle_file_menu_button,
                menu::menu_action_system,
                menu::load_system,
           ))
           .add_systems(Update, inspector::inspector_ui_system)
           .add_systems(PostUpdate, menu::save_system);
    }
}
