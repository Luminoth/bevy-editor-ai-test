use bevy::prelude::*;

pub mod camera;
pub mod components;
pub mod hierarchy;
pub mod inspector;
pub mod menu;
pub mod resources;
pub mod styles;
pub mod ui;
pub mod input;
pub mod actions;

use resources::{EditorConfig, EditorState, InspectorUiState};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EditorState>()
           .init_resource::<EditorConfig>()
           .init_resource::<InspectorUiState>()
           .add_systems(Startup, ui::setup_editor_ui)
           .add_systems(Update, (
                ui::toggle_editor,
                hierarchy::update_hierarchy_list,
                hierarchy::update_hierarchy,
                hierarchy::handle_hierarchy_actions,
                menu::handle_file_menu_button,
                menu::menu_action_system,
           ))
           .add_systems(Update, (
                menu::load_system,
                input::text_input_system,
                actions::handle_delete_entity,
                actions::handle_add_component_toggle,
                actions::handle_remove_component,
                actions::handle_add_component_confirm,
           ))
           .add_systems(Update, inspector::inspector_ui_system)
           .add_systems(PostUpdate, menu::save_system);
    }
}
