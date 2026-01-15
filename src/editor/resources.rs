use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct EditorState {
    pub selected_entity: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct EditorConfig {
    pub show_editor: bool,
}
