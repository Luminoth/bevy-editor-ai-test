use bevy::prelude::*;


#[derive(Resource, Default)]
pub struct EditorState {
    pub selected_entity: Option<Entity>,
    pub focused_input: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct EditorConfig {
    pub show_editor: bool,
}

#[derive(Resource, Default)]
pub struct InspectorUiState {
    pub is_adding_component: bool,
    pub component_filter: String,
}

#[derive(Resource, Default)]
pub struct IsResizing(pub bool);
